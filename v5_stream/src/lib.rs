#![no_std]

#[cfg(any(test, feature = "std"))]
extern crate std;
extern crate alloc;
use crate::serialize_stream::SerializeStream;
use crate::multiplexed_stream::{MultiplexedStream, MultiplexPacket, ChannelIndexType, DataPacket};
use v5_traits::{NamedUniversal, UniversalFunctions};
use v5_traits::stream::{MessageStreamCreator, DuplexTimeoutStream};
use crate::checksum_byte_stream::{ChecksumByteStream, ChecksumByteTimeouts};
use serde::__private::Vec;
use alloc::sync::Arc;
use v5_traits::task::TaskRunner;
use core::time::Duration;

pub mod checksum_byte_stream;
pub mod composed_stream;
pub mod identifiable;
pub mod multiplexed_stream;
pub mod serialize_stream;
pub mod simple_byte_stream;
pub mod split_stream;

pub type SuperStream<UF, S, C1, C2> =
MultiplexedStream<
    NamedUniversal<UF>,
    SerializeStream<
        NamedUniversal<UF>,
        MultiplexPacket,
        ChecksumByteStream<
            NamedUniversal<UF>,
            S,
            C2
        >
    >,
    C1
>;
pub fn create_super_stream<UF, S, C1, C2>(uf: UF, stream: S, creator1: &C1, creator2: &C2, num_channels: ChannelIndexType) -> SuperStream<UF, S, C1, C2>
    where UF: UniversalFunctions + Clone,
          S: DuplexTimeoutStream<u8>,
          C1: MessageStreamCreator<DataPacket>,
          C2: MessageStreamCreator<Vec<u8>>{
    MultiplexedStream::new(
        NamedUniversal::new(uf.clone(), "MultiplexStream"),
        SerializeStream::new(
            NamedUniversal::new(uf.clone(), "SerializeStream"),
            ChecksumByteStream::new(
                NamedUniversal::new(uf, "ByteStream"),
                stream,
                creator2
            )
        ),
        creator1,
        num_channels
    )
}

pub fn super_stream_management<UF, S, C1, C2, TR>(super_stream: Arc<SuperStream<UF, S, C1, C2>>, multiplex_outbound_delay: Duration, timeouts: ChecksumByteTimeouts, runner: TR) -> TaskTracker<TR::TaskTracker>
    where UF: 'static + UniversalFunctions + Clone,
          S: 'static + DuplexTimeoutStream<u8> + Send + Sync,
          C1: 'static + MessageStreamCreator<DataPacket>,
          C2: 'static + MessageStreamCreator<Vec<u8>>,
          TR: 'static + TaskRunner<(), ()>{
    let super_stream_clone1 = super_stream.clone();
    let super_stream_clone2 = super_stream.clone();
    TaskTracker{
        multiplex_inbound: runner.run_task(move|_|super_stream_clone1.handle_inbound(), ()),
        multiplex_outbound: runner.run_task(move|_|super_stream_clone2.handle_outbound(multiplex_outbound_delay), ()),
        byte_stream_management: runner.run_task(move|_|super_stream.stream().stream().management_loop(timeouts), ()),
    }
}
pub struct TaskTracker<T>{
    pub multiplex_inbound: T,
    pub multiplex_outbound: T,
    pub byte_stream_management: T,
}

#[cfg(all(test, feature = "std"))]
mod test{
    use crate::{create_super_stream, super_stream_management};
    use v5_traits::{UniversalFunctions, LogLevel, NamedUniversal};
    use core::time::Duration;
    use core::fmt::Display;
    use std::{print, eprint, eprintln, println};
    use std::time::SystemTime;
    use std::thread::{sleep, spawn, JoinHandle};
    use alloc::format;
    use ansi_rgb::{Foreground, white, Background, red, yellow, orange, blue};
    use v5_traits::stream::std_impls::MPSCMessageCreator;
    use crate::composed_stream::ComposedStream;
    use v5_traits::stream::{MessageStreamCreator, SendStream, ReceiveStream};
    use alloc::sync::Arc;
    use alloc::vec;
    use alloc::vec::Vec;
    use crate::checksum_byte_stream::{ChecksumByteTimeouts, ChecksumByteStream};
    use v5_traits::task::{TaskRunner, TaskFunction};

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
                LogLevel::WARN => println!("{}", format!("{}", message).fg(orange())),
                LogLevel::DEBUG => println!("{}", format!("{}", message).fg(yellow())),
                LogLevel::INFO => println!("{}", format!("{}", message).fg(blue())),
                LogLevel::TRACE => println!("{}", format!("{}", message).fg(white())),
            }
        }
    }

    #[derive(Copy, Clone)]
    struct Tr();
    impl<T, O> TaskRunner<T, O> for Tr where T: 'static + Send, O: 'static + Send {
        type TaskTracker = JoinHandle<O>;

        fn run_task(&self, task: impl TaskFunction<T, O>, task_argument: T) -> Self::TaskTracker{
            spawn(move||task(task_argument))
        }
    }

    #[test]
    fn checksum_byte_stream_test(){
        let creator = MPSCMessageCreator();
        let ((sender1, receiver1), (sender2, receiver2)) = creator.create_bidirectional_stream();
        let composed1 = ComposedStream::new(sender1, receiver2);
        let composed2 = ComposedStream::new(sender2, receiver1);
        let checksum_byte_stream1 = Arc::new(ChecksumByteStream::new(NamedUniversal::new(Uf(), "checksum_byte_stream1"), composed1, &creator));
        let checksum_byte_stream2 = Arc::new(ChecksumByteStream::new(NamedUniversal::new(Uf(), "checksum_byte_stream2"), composed2, &creator));

        let checksum_byte_stream1_clone = checksum_byte_stream1.clone();
        let checksum_byte_stream2_clone = checksum_byte_stream2.clone();

        let timeouts = ChecksumByteTimeouts{
            receive_packet_start_timeout: Duration::from_millis(100),
            receive_packet_timeout: Duration::from_millis(100),
            send_packet_timeout: Duration::from_millis(100),
        };

        Tr().run_task(move|_|checksum_byte_stream1_clone.management_loop(timeouts), ());
        Tr().run_task(move|_|checksum_byte_stream2_clone.management_loop(timeouts), ());

        Uf().log_info(||format!("Sending vec![100; 100]"));
        checksum_byte_stream1.send(vec![100; 100]).expect("Send failed");
        Uf().log_info(||format!("Receiving..."));
        assert_eq!(checksum_byte_stream2.receive().expect("Receive failed"), vec![100; 100]);
    }

    #[test]
    fn super_stream_test(){
        Uf().log_trace(||"Beginning super_stream_test!");
        let creator = MPSCMessageCreator();
        let ((sender1, receiver1), (sender2, receiver2)) = creator.create_bidirectional_stream();
        let composed1 = ComposedStream::new(sender1, receiver2);
        let composed2 = ComposedStream::new(sender2, receiver1);
        let super1 = Arc::new(create_super_stream(NamedUniversal::new(Uf(), "Super1"), composed1, &creator, &creator, 10));
        let super2 = Arc::new(create_super_stream(NamedUniversal::new(Uf(), "Super2"), composed2, &creator, &creator, 12));

        let timeouts = ChecksumByteTimeouts{
            receive_packet_start_timeout: Duration::from_millis(100),
            receive_packet_timeout: Duration::from_millis(100),
            send_packet_timeout: Duration::from_millis(100),
        };

        // let super1_clone1 = super1.clone();
        // let super1_clone2 = super1.clone();
        // let super1_clone3 = super1.clone();
        // let super2_clone1 = super2.clone();
        // let super2_clone2 = super2.clone();
        // let super2_clone3 = super2.clone();
        //
        // spawn(move||super1_clone1.handle_inbound());
        // spawn(move||super1_clone2.handle_outbound(Duration::from_millis(100)));
        // spawn(move||super1_clone3.stream().stream().management_loop(timeouts));
        // spawn(move||super2_clone1.handle_inbound());
        // spawn(move||super2_clone2.handle_outbound(Duration::from_millis(100)));
        // spawn(move||super2_clone3.stream().stream().management_loop(timeouts));

        let runner = Tr();
        super_stream_management(super1.clone(), Duration::from_millis(100), timeouts, runner);
        super_stream_management(super2.clone(), Duration::from_millis(100), timeouts, runner);

        assert_eq!(super1.num_channels(), 10);
        assert_eq!(super2.num_channels(), 12);

        assert!(super1.get_channel::<u8>(10).is_none());
        assert!(super2.get_channel::<u8>(20).is_none());

        let super1_channel3 = super1.get_channel::<u64>(3).expect("Could not open super 1 channel 3");
        assert!(super1.get_channel::<u8>(3).is_none());
        let super2_channel3 = super2.get_channel::<u64>(3).expect("Could not open super 2 channel 3");
        assert!(super2.get_channel::<u8>(3).is_none());

        super1_channel3.send(100).expect("Could not send");
        super1_channel3.send(100).expect("Could not send");
        super1_channel3.send(100).expect("Could not send");
        super1_channel3.send(100).expect("Could not send");
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
