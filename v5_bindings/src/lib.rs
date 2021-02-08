#![no_std]
#![feature(alloc_error_handler)]
#![feature(negative_impls)]
#![feature(const_fn)]
// #![feature(restricted_std)]

#[macro_use]
extern crate alloc;

use core::fmt::Display;
use core::time::Duration;

use crate::raw::fmt_to_char_ptr;
use crate::raw::pros::api::printf;
pub use crate::raw::pros::rtos::notify_action_e_t as NotifyAction;
pub use crate::raw::pros::rtos::task_state_e_t as State;
pub use crate::task::Task;
use v5_traits::{UniversalFunctions, LogLevel};
use crate::raw::pros::rtos::delay;
use ansi_rgb::{Foreground, white, Background, red, orange, magenta, blue};

// use std::sync::Mutex;

pub mod raw;
pub mod sync;
pub mod robot;

mod alloc_bindings;
mod export_functions;

pub mod error;
pub mod percent;
pub mod task;
// Doesn't work
// pub mod usb_serial;
pub mod user_functions;
pub mod util;

#[cfg(feature = "v5_test")]
pub mod test;
#[cfg(feature = "example_functions")]
pub mod example_functions;

pub fn system_time() -> Duration {
    Duration::from_millis(unsafe { raw::pros::rtos::millis() } as u64)
}

pub fn console_print(fmt: impl Display){
    unsafe {printf(fmt_to_char_ptr(fmt).as_ptr())};
}
pub fn console_println(fmt: impl Display){
    console_print(&format!("{}\n", fmt))
}

#[derive(Copy, Clone, Debug)]
pub struct V5UniversalFunctions;
impl UniversalFunctions for V5UniversalFunctions{
    fn delay(&self, duration: Duration) {
        unsafe {delay(duration.as_millis() as u32)}
    }

    fn system_time(&self) -> Duration {
        system_time()
    }

    fn print(&self, out: impl Display) {
        console_print(out)
    }

    fn eprint(&self, out: impl Display) {
        console_print(out.fg(white()).bg(red()))
    }

    fn min_log_level(&self) -> LogLevel {
        LogLevel::DEBUG
    }

    fn log_intern(&self, message: impl Display, level: LogLevel) {
        match level {
            LogLevel::FATAL => console_print(message.fg(white()).bg(red())),
            LogLevel::ERROR => console_print(message.fg(red())),
            LogLevel::WARN => console_print(message.fg(orange())),
            LogLevel::DEBUG => console_print(message.fg(magenta())),
            LogLevel::INFO => console_print(message.fg(blue())),
            LogLevel::TRACE => console_print(message.fg(white())),
        }
    }
}
