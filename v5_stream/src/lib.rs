#![no_std]

#[cfg(any(test, feature = "std"))]
#[cfg_attr(test, macro_use)]
extern crate std;
#[macro_use]
extern crate alloc;

pub mod checksum_byte_stream;
pub mod composed_stream;
pub mod identifiable;
pub mod serialize_stream;
pub mod simple_byte_stream;
pub mod split_stream;

pub struct TaskTracker<T>{
    pub multiplex_inbound: T,
    pub multiplex_outbound: T,
    pub byte_stream_management: T,
}

#[cfg(all(test, feature = "std"))]
mod test{
    use v5_traits::{UniversalFunctions, LogLevel, NamedUniversal};
    use core::time::Duration;
    use core::fmt::Display;
    use std::time::SystemTime;
    use std::thread::{sleep, spawn, JoinHandle};
    use ansi_rgb::{Foreground, white, Background, red, yellow, orange, blue};
    use v5_traits::stream::std_impls::MPSCMessageCreator;
    use crate::composed_stream::ComposedStream;
    use v5_traits::stream::{MessageStreamCreator, SendStream, ReceiveStream};
    use alloc::sync::Arc;
    use alloc::vec;
    use crate::checksum_byte_stream::{ChecksumByteStream};
    use v5_traits::task::{TaskRunner, TaskFunction};
    use crate::serialize_stream::SerializeStream;
    use parking_lot::Mutex;

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

        fn run_task(&self, _name: impl Display, task: impl TaskFunction<T, O>, task_argument: T) -> Self::TaskTracker{
            spawn(move||task(task_argument))
        }
    }

    #[test]
    fn checksum_byte_stream_test(){
        let creator = MPSCMessageCreator();
        let ((sender1, receiver1), (sender2, receiver2)) = creator.create_bidirectional_stream();
        let composed1 = ComposedStream::new(sender1, receiver2);
        let composed2 = ComposedStream::new(sender2, receiver1);
        let checksum_byte_stream1 = Arc::new(ChecksumByteStream::<_, _, Mutex<_>>::new(NamedUniversal::new(Uf(), "checksum_byte_stream1"), composed1));
        let checksum_byte_stream2 = Arc::new(ChecksumByteStream::<_, _, Mutex<_>>::new(NamedUniversal::new(Uf(), "checksum_byte_stream2"), composed2));

        Uf().log_info(||"Sending vec![100; 100]");
        checksum_byte_stream1.send(vec![100; 100]);
        Uf().log_info(||"Receiving...");
        assert_eq!(checksum_byte_stream2.receive(), vec![100; 100]);
    }

    #[test]
    fn super_stream_test(){
        Uf().log_trace(||"Beginning super_stream_test!");
        let creator = MPSCMessageCreator();
        let ((sender1, receiver1), (sender2, receiver2)) = creator.create_bidirectional_stream();
        let composed1 = ComposedStream::new(sender1, receiver2);
        let composed2 = ComposedStream::new(sender2, receiver1);
        let super1 = Arc::new(
            SerializeStream::new(
                NamedUniversal::new(Uf(), "Super1 Serialize"),
                ChecksumByteStream::<_, _, Mutex<_>>::new(
                    NamedUniversal::new(Uf(), "Super1 Checksum"),
                    composed1
                )
            )
        );
        let super2 = Arc::new(
            SerializeStream::new(
                NamedUniversal::new(Uf(), "Super2 Serialize"),
                ChecksumByteStream::<_, _, Mutex<_>>::new(
                    NamedUniversal::new(Uf(), "Super2 Checksum"),
                    composed2
                )
            )
        );

        for _ in 0..100{
            super1.send(100);
        }
        assert_eq!(super2.receive(), 100);

        super2.send_vec(vec![1998; 1024]);
        let out = super1.receive_whole_vec(1024);
        for val in out{
            assert_eq!(val, 1998);
        }
    }
}
