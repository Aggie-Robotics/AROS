use crate::robot::port::Port;
use crate::robot::Robot;

extern "Rust" {
    #[cfg(not(feature = "zero_based_ports"))]
    pub fn rust_user_initialize(ports: [Option<Port>; 22]) -> Robot;
    #[cfg(feature = "zero_based_ports")]
    pub fn rust_user_initialize(ports: [Port; 21]) -> Robot;
    pub fn rust_user_disabled(robot: &'static Robot) -> !;
    pub fn rust_user_competition_initialize(robot: &mut Robot);
    pub fn rust_user_autonomous(robot: &'static Robot) -> !;
    pub fn rust_user_opcontrol(robot: &'static Robot) -> !;
}
