#![no_std]

use ansi_rgb::{Foreground, orange};

use v5_bindings::console_println;
use v5_bindings::robot::port::Port;
use v5_bindings::robot::Robot;

#[no_mangle]
extern "Rust" fn rust_user_initialize(_ports: [Port; 21]) -> Robot {
    console_println(&"Hello from rust! rust_user_initialize".fg(orange()));
    Robot::new()
}

#[no_mangle]
extern "Rust" fn rust_user_disabled(_robot: &'static Robot) -> ! {
    console_println(&"Hello from rust! rust_disabled".fg(orange()));
    loop {}
}

#[no_mangle]
extern "Rust" fn rust_user_competition_initialize(_robot: &mut Robot) {
    console_println(&"Hello from rust! rust_competition_initialize".fg(orange()));
}

#[no_mangle]
extern "Rust" fn rust_user_autonomous(_robot: &'static Robot) -> ! {
    console_println(&"Hello from rust! rust_autonomous".fg(orange()));
    loop {}
}

#[no_mangle]
extern "Rust" fn rust_user_opcontrol(_robot: &'static Robot) -> ! {
    console_println(&"Hello from rust! rust_opcontrol".fg(orange()));
    loop {}
}

#[cfg(feature = "v5_test")]
mod test {
    use alloc::vec::Vec;

    use v5_bindings::robot::Robot;
    use v5_bindings::test::TestItem;
    use v5_bindings::sync::lock::test::{mutex_test, rw_lock_test};
    use v5_bindings::sync::queue::test::queue_test;

    // #[allow(improper_ctypes_definitions)]
    #[no_mangle]
    extern "Rust" fn get_tests(_robot: &'static Robot) -> Vec<TestItem> {
        let mut out = Vec::with_capacity(2);

        out.push(mutex_test());
        out.push(rw_lock_test());
        out.push(queue_test());

        out
    }
}
