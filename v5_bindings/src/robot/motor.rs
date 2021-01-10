use crate::raw::vex_os::api::*;
use crate::raw::vex_os::api_types::V5MotorControlMode;
use crate::robot::port::Port;

pub struct Motor {
    port: Port,
}

impl Motor {
    pub fn new(port: Port, reverse: bool, brake: bool) -> Self {
        unsafe {
            vexDeviceMotorReverseFlagSet(port.device(), reverse);
            vexDeviceMotorModeSet(port.device(), if brake {
                V5MotorControlMode::kMotorControlModeBRAKE
            } else {
                V5MotorControlMode::kMotorControlModeOFF
            })
        }
        Self {
            port
        }
    }

    pub fn is_braking(&self) -> bool {
        V5MotorControlMode::kMotorControlModeBRAKE == unsafe { vexDeviceMotorModeGet(self.port.device()) }
    }
    pub fn set_braking(&mut self, brake: bool) {
        unsafe {
            vexDeviceMotorModeSet(self.port.device(), if brake {
                V5MotorControlMode::kMotorControlModeBRAKE
            } else {
                V5MotorControlMode::kMotorControlModeOFF
            })
        }
    }
}
