use crate::raw::vex_os::api::*;
use crate::raw::vex_os::api_types::V5MotorControlMode;
use crate::robot::port::Port;
use core::time::Duration;

pub struct Motor {
    port: Port,
}

impl Motor {
    pub fn new(port: Port, reverse: bool, brake: bool) -> Self {
        unsafe {
            vexDeviceMotorReverseFlagSet(port.device(), reverse);
            vexDeviceMotorModeSet(
                port.device(),
                if brake { V5MotorControlMode::kMotorControlModeBRAKE } else { V5MotorControlMode::kMotorControlModeOFF }
            );
        }
        Self {
            port
        }
    }

    pub fn set_current_limit(&self, limit: i32) {
        unsafe { vexDeviceMotorCurrentLimitSet(self.port.device(), limit) }
    }
    pub fn get_current_limit(&self) -> i32 {
        unsafe { vexDeviceMotorCurrentLimitGet(self.port.device()) }
    }
    pub fn set_voltage_limit(&self, limit: i32) {
        unsafe { vexDeviceMotorVoltageLimitSet(self.port.device(), limit) }
    }
    pub fn get_voltage_limit(&self) -> i32 {
        unsafe { vexDeviceMotorVoltageLimitGet(self.port.device()) }
    }
    pub fn get_current(&self) -> i32 {
        unsafe { vexDeviceMotorCurrentGet(self.port.device()) }
    }
    pub fn get_voltage(&self) -> i32 {
        unsafe { vexDeviceMotorVoltageGet(self.port.device()) }
    }
    pub fn get_power(&self) -> f64 {
        unsafe { vexDeviceMotorPowerGet(self.port.device()) }
    }
    pub fn get_torque(&self) -> f64 {
        unsafe { vexDeviceMotorTorqueGet(self.port.device()) }
    }
    pub fn get_efficiency(&self) -> f64 {
        unsafe { vexDeviceMotorEfficiencyGet(self.port.device()) }
    }
    pub fn get_temperature(&self) -> f64 {
        unsafe { vexDeviceMotorTemperatureGet(self.port.device()) }
    }
    pub fn is_over_temp(&self) -> bool {
        unsafe { vexDeviceMotorOverTempFlagGet(self.port.device()) }
    }
    pub fn position(&self) -> i32{
        unsafe { vexDeviceMotorPositionRawGet(self.port.device(), &mut 0) }
    }
    pub fn position_with_timestamp(&self) -> (i32, Duration){
        let mut out = 0;
        (unsafe { vexDeviceMotorPositionRawGet(self.port.device(), &mut out)}, Duration::from_millis(out as u64))
    }
    pub fn reset_position(&self){
        unsafe { vexDeviceMotorPositionReset(self.port.device()) }
    }

    /// Sets motor voltage in mV from -12000 to 12000
    pub fn set_voltage(&self, voltage: i32) {
        unsafe { vexDeviceMotorVoltageSet(self.port.device(), voltage) }
    }

    pub fn is_braking(&self) -> bool {
        V5MotorControlMode::kMotorControlModeBRAKE == unsafe { vexDeviceMotorModeGet(self.port.device()) }
    }
    pub fn set_braking(&self, brake: bool) {
        unsafe {
            vexDeviceMotorModeSet(self.port.device(), if brake {
                V5MotorControlMode::kMotorControlModeBRAKE
            } else {
                V5MotorControlMode::kMotorControlModeOFF
            })
        }
    }
}
