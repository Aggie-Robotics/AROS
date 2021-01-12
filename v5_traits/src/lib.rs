#![no_std]

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;
use core::time::Duration;
use core::fmt::{Display, Debug};

pub mod error;
pub mod stream;
pub mod sync;
pub mod task;

pub trait EnsureSend: Send{}
pub trait EnsureSync: Sync{}

pub trait UniversalFunctions: Debug + Send + Sync{
    const MAX_LOG_LEVEL: LogLevel;

    /// Delays the current thread for duration
    fn delay(duration: Duration);
    fn print(out: impl Display);
    fn eprint(out: impl Display);
    fn println(out: impl Display){
        Self::print(format_args!("{}\n", out))
    }
    fn eprintln(out: impl Display){
        Self::println(format_args!("{}\n", out))
    }

    fn log_intern(message: impl Display, level: LogLevel);
    fn log<T>(message: impl FnOnce() -> T, level: LogLevel) where T: Display{
        if Self::MAX_LOG_LEVEL >= level{
            Self::log_intern(message(), level);
        }
    }
}

#[derive(PartialOrd, PartialEq, Ord, Eq)]
pub enum LogLevel{
    FATAL = 10,
    ERROR = 20,
    DEBUG = 30,
    INFO  = 40,
    TRACE = 50,
}

