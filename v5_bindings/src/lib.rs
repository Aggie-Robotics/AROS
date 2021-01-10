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
pub use crate::task::TaskArgument;
pub use crate::task::TaskFunction;

// use std::sync::Mutex;

pub mod raw;
pub mod sync;
pub mod robot;

mod alloc_bindings;
mod export_functions;

pub mod error;
pub mod percent;
pub mod task;
pub mod usb_serial;
pub mod user_functions;
pub mod util;

#[cfg(feature = "v5_test")]
pub mod test;
#[cfg(feature = "example_functions")]
pub mod example_functions;

pub fn system_time() -> Duration {
    Duration::from_millis(unsafe { raw::pros::rtos::millis() } as u64)
}

pub fn console_print(fmt: &impl Display){
    unsafe {printf(fmt_to_char_ptr(fmt).as_ptr())};
}
pub fn console_println(fmt: &impl Display){
    console_print(&format!("{}\n", fmt))
}
