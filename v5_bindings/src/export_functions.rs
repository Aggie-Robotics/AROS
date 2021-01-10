use crate::robot::port::Port;
use crate::robot::Robot;
use crate::user_functions::*;

static mut ROBOT: Option<Robot> = None;

#[no_mangle]
extern "C" fn rust_initialize() {
    unsafe { ROBOT = Some(rust_user_initialize(Port::get_all())) };
    #[cfg(feature = "v5_test")]
        {
            use crate::test::*;
            test_runner(unsafe { get_tests(ROBOT.as_ref().unwrap()) });
        }
}

#[no_mangle]
extern "C" fn rust_disabled() {
    unsafe { rust_user_disabled(ROBOT.as_ref().unwrap()) };
}

#[no_mangle]
extern "C" fn rust_competition_initialize() {
    unsafe { rust_user_competition_initialize(ROBOT.as_mut().unwrap()) };
}

#[no_mangle]
extern "C" fn rust_autonomous() {
    unsafe { rust_user_autonomous(ROBOT.as_ref().unwrap()) };
}

#[no_mangle]
extern "C" fn rust_opcontrol() {
    unsafe { rust_user_opcontrol(ROBOT.as_ref().unwrap()) };
}
