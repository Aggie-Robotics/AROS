#![no_std]

#[cfg(any(test, feature = "std"))]
extern crate std;
extern crate alloc;
use crate::serialize_stream::SerializeStream;
use crate::simple_byte_stream::SimpleByteStream;
use crate::multiplexed_stream::{MultiplexedStream, MultiplexPacket, ChannelIndexType, DataPacket};
use v5_traits::{NamedUniversal, UniversalFunctions};
use v5_traits::stream::{DuplexStream, MessageStreamCreator};

pub mod composed_stream;
pub mod identifiable;
pub mod multiplexed_stream;
pub mod serialize_stream;
pub mod simple_byte_stream;

pub type SuperStream<UF, S, C> =
MultiplexedStream<
    NamedUniversal<UF>,
    SerializeStream<
        NamedUniversal<UF>,
        MultiplexPacket,
        SimpleByteStream<
            NamedUniversal<UF>,
            S
        >
    >,
    C
>;
pub fn create_super_stream<UF, S, C>(uf: UF, stream: S, creator: &C, num_channels: ChannelIndexType) -> SuperStream<UF, S, C>
    where UF: UniversalFunctions + Clone,
          S: DuplexStream<u8>,
          C: MessageStreamCreator<DataPacket>{
    MultiplexedStream::new(
        NamedUniversal::new(uf.clone(), "MultiplexStream"),
        SerializeStream::new(
            NamedUniversal::new(uf.clone(), "SerializeStream"),
            SimpleByteStream::new(
                NamedUniversal::new(uf, "ByteStream"),
                stream
            )
        ),
        creator,
        num_channels
    )
}

#[cfg(all(test, feature = "std"))]
mod test{
    use crate::create_super_stream;
    use v5_traits::{UniversalFunctions, LogLevel, NamedUniversal};
    use core::time::Duration;
    use core::fmt::Display;
    use std::{print, eprint, eprintln};
    use std::time::SystemTime;
    use std::thread::{sleep, spawn};
    use alloc::format;
    use ansi_rgb::{Foreground, white, Background, red, yellow, orange, blue};
    use v5_traits::stream::std_impls::MPSCMessageCreator;
    use crate::composed_stream::ComposedStream;
    use v5_traits::stream::{MessageStreamCreator, SendStream, ReceiveStream};
    use alloc::sync::Arc;
    use alloc::vec;
    use alloc::vec::Vec;

    #[derive(Debug, Clone)]
    struct Uf();
    impl UniversalFunctions for Uf{
        fn delay(&self, duration: Duration) {
            sleep(duration)
        }

        fn system_time(&self) -> Duration {
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap()
        }

        fn print(&self, out: impl Display) {
            print!("{}", out)
        }

        fn eprint(&self, out: impl Display) {
            eprint!("{}", out)
        }

        fn min_log_level(&self) -> LogLevel {
            LogLevel::TRACE
        }

        fn log_intern(&self, message: impl Display, level: LogLevel) {
            match level {
                LogLevel::FATAL => eprintln!("{}", format!("{}", message).fg(white()).bg(red())),
                LogLevel::ERROR => eprintln!("{}", format!("{}", message).fg(red())),
                LogLevel::WARN => eprintln!("{}", format!("{}", message).fg(orange())),
                LogLevel::DEBUG => eprintln!("{}", format!("{}", message).fg(yellow())),
                LogLevel::INFO => eprintln!("{}", format!("{}", message).fg(blue())),
                LogLevel::TRACE => eprintln!("{}", format!("{}", message).fg(white())),
            }
        }
    }


    #[test]
    fn super_stream_test(){
        let creator = MPSCMessageCreator();
        let ((sender1, receiver1), (sender2, receiver2)) = creator.create_bidirectional_stream();
        let composed1 = ComposedStream::new(sender1, receiver2);
        let composed2 = ComposedStream::new(sender2, receiver1);
        let super1 = Arc::new(create_super_stream(NamedUniversal::new(Uf(), "Super1"), composed1, &creator, 10));
        let super2 = Arc::new(create_super_stream(NamedUniversal::new(Uf(), "Super2"), composed2, &creator, 12));

        let super1_clone1 = super1.clone();
        let super1_clone2 = super1.clone();
        let super2_clone1 = super2.clone();
        let super2_clone2 = super2.clone();

        spawn(move||super1_clone1.handle_inbound());
        spawn(move||super1_clone2.handle_outbound(Duration::from_millis(100)));
        spawn(move||super2_clone1.handle_inbound());
        spawn(move||super2_clone2.handle_outbound(Duration::from_millis(100)));

        assert_eq!(super1.num_channels(), 10);
        assert_eq!(super2.num_channels(), 12);

        assert!(super1.get_channel::<u8>(10).is_none());
        assert!(super2.get_channel::<u8>(20).is_none());

        let super1_channel3 = super1.get_channel::<u64>(3).expect("Could not open super 1 channel 3");
        assert!(super1.get_channel::<u8>(3).is_none());
        let super2_channel3 = super2.get_channel::<u64>(3).expect("Could not open super 2 channel 3");
        assert!(super2.get_channel::<u8>(3).is_none());

        super1_channel3.send(100).expect("Could not send");
        assert_eq!(super2_channel3.receive().expect("Could not receive"), 100);

        super2_channel3.send_vec(vec![1998; 1024]).expect("Could not send vec");
        let mut out = Vec::with_capacity(1024);
        super1_channel3.receive_whole_vec(&mut out, 1024).expect("Could not receive vec");
        for val in out{
            assert_eq!(val, 1998);
        }
    }
}

//TODO: Make a binary packet handler
