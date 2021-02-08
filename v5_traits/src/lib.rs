#![no_std]

#[cfg(feature = "std")]
#[macro_use]
extern crate std;
#[macro_use]
extern crate alloc;

pub mod error;
pub mod stream;
pub mod mutex;
pub mod sync_cell;
pub mod task;

use core::time::Duration;
use core::fmt::{Display, Debug};
use alloc::sync::Arc;
use alloc::string::String;
use core::ops::Deref;

pub trait EnsureSend: Send{}
pub trait EnsureSync: Sync{}

pub trait UniversalFunctions: 'static + Clone + Debug + Send + Sync{

    /// Delays the current thread for duration
    fn delay(&self, duration: Duration);
    fn system_time(&self) -> Duration;
    fn print(&self, out: impl Display);
    fn eprint(&self, out: impl Display);
    fn println(&self, out: impl Display){
        self.print(format_args!("{}\n", out))
    }
    fn eprintln(&self, out: impl Display){
        self.println(format_args!("{}\n", out))
    }

    fn min_log_level(&self) -> LogLevel;
    /// Logs a message to the console at a log level. This method should not be called if level < min_log_level
    fn log_intern(&self, message: impl Display, level: LogLevel);
    /// Call log_intern with the message if the log level
    /// Not designed to be user overwritten
    fn log<T>(&self, message: impl FnOnce() -> T, level: LogLevel) where T: Display{
        if self.min_log_level() >= level{
            self.log_intern(message(), level);
        }
    }
    fn log_fatal<T>(&self, message: impl FnOnce() -> T) where T: Display{
        self.log(message, LogLevel::FATAL);
    }
    fn log_error<T>(&self, message: impl FnOnce() -> T) where T: Display{
        self.log(message, LogLevel::ERROR);
    }
    fn log_warn<T>(&self, message: impl FnOnce() -> T) where T: Display{
        self.log(message, LogLevel::WARN)
    }
    fn log_debug<T>(&self, message: impl FnOnce() -> T) where T: Display{
        self.log(message, LogLevel::DEBUG);
    }
    fn log_info<T>(&self, message: impl FnOnce() -> T) where T: Display{
        self.log(message, LogLevel::INFO)
    }
    fn log_trace<T>(&self, message: impl FnOnce() -> T) where T: Display{
        self.log(message, LogLevel::TRACE)
    }
}
impl<U> UniversalFunctions for Arc<U> where U: UniversalFunctions{

    fn delay(&self, duration: Duration) {
        self.deref().delay(duration)
    }

    fn system_time(&self) -> Duration {
        self.deref().system_time()
    }

    fn print(&self, out: impl Display) {
        self.deref().print(out)
    }

    fn eprint(&self, out: impl Display) {
        self.deref().eprint(out)
    }

    fn println(&self, out: impl Display) {
        self.deref().println(out)
    }

    fn eprintln(&self, out: impl Display) {
        self.deref().eprintln(out)
    }

    fn min_log_level(&self) -> LogLevel {
        self.deref().min_log_level()
    }

    fn log_intern(&self, message: impl Display, level: LogLevel) {
        self.deref().log_intern(message, level)
    }

    fn log<T>(&self, message: impl FnOnce() -> T, level: LogLevel) where T: Display {
        self.deref().log(message, level)
    }
}

#[derive(PartialOrd, PartialEq, Ord, Eq)]
pub enum LogLevel{
    FATAL = 10,
    ERROR = 20,
    WARN  = 30,
    DEBUG = 40,
    INFO  = 50,
    TRACE = 60,
}

pub trait FormattedUniversal: Clone + Debug + Send + Sync{
    type U: UniversalFunctions;
    type D: Display;

    fn get_universal(&self) -> &Self::U;
    fn format(&self, message: impl Display) -> Self::D;
}
impl<T> UniversalFunctions for T where T: 'static + FormattedUniversal{
    fn delay(&self, duration: Duration) {
        self.get_universal().delay(duration)
    }

    fn system_time(&self) -> Duration {
        self.get_universal().system_time()
    }

    fn print(&self, out: impl Display) {
        self.get_universal().print(self.format(out))
    }

    fn eprint(&self, out: impl Display) {
        self.get_universal().eprint(self.format(out))
    }

    fn println(&self, out: impl Display) {
        self.get_universal().println(self.format(out))
    }

    fn eprintln(&self, out: impl Display) {
        self.get_universal().eprintln(self.format(out))
    }

    fn min_log_level(&self) -> LogLevel {
        self.get_universal().min_log_level()
    }

    fn log_intern(&self, message: impl Display, level: LogLevel) {
        self.get_universal().log_intern(self.format(message), level)
    }
}

#[derive(Clone, Debug)]
pub struct NamedUniversal<U> where U: UniversalFunctions{
    functions: U,
    name: String,
}
impl<U> NamedUniversal<U> where U: UniversalFunctions{
    pub fn new(functions: U, name: impl Display) -> Self{
        Self{ functions, name: format!("{}", name) }
    }
}
impl<U> FormattedUniversal for NamedUniversal<U> where U: UniversalFunctions{
    type U = U;
    type D = String;

    fn get_universal(&self) -> &Self::U {
        &self.functions
    }

    fn format(&self, message: impl Display) -> Self::D {
        format!("[{}]{}", self.name, message)
    }
}

#[derive(Clone, Debug)]
pub struct TimedUniversal<U> where U: UniversalFunctions{
    functions: U,
}
impl<U> TimedUniversal<U> where U: UniversalFunctions{
    pub fn new(functions: U) -> Self{
        Self{ functions }
    }
}
impl<U> FormattedUniversal for TimedUniversal<U> where U: UniversalFunctions{
    type U = U;
    type D = String;

    fn get_universal(&self) -> &Self::U {
        &self.functions
    }

    fn format(&self, message: impl Display) -> Self::D {
        format!("[{}]{}", self.functions.system_time().as_secs_f64(), message)
    }
}
