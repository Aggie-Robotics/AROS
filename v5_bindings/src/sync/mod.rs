use core::time::Duration;

use crate::raw::pros::rtos::TIMEOUT_MAX;

pub mod lock;
pub mod queue;

fn option_to_timeout(timeout: Option<Duration>) -> u32{
    match timeout {
        None => TIMEOUT_MAX,
        Some(duration) => duration.as_millis() as u32,
    }
}
