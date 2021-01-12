#![no_std]

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;
use core::time::Duration;
use core::fmt::Display;

pub mod error;
pub mod stream;
pub mod sync;
pub mod task;

pub trait UniversalFunctions{
    /// Delays the current thread for duration
    fn delay(duration: Duration);
    fn print(out: impl Display);
    fn eprint(out: impl Display);
}
