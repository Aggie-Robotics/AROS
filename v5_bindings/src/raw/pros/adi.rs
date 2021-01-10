#![allow(non_camel_case_types)]

use cty::*;

pub use adi_port_config_e as adi_port_config_e_t;

use crate::raw::pros::api::PROS_ERR;

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum adi_port_config_e{
    E_ADI_ANALOG_IN = 0,
    E_ADI_ANALOG_OUT = 1,
    E_ADI_DIGITAL_IN = 2,
    E_ADI_DIGITAL_OUT = 3,
    E_ADI_LEGACY_GYRO = 10,
    E_ADI_LEGACY_SERVO = 12,
    E_ADI_LEGACY_PWM = 13,
    E_ADI_LEGACY_ENCODER = 14,
    E_ADI_LEGACY_ULTRASONIC = 15,
    E_ADI_TYPE_UNDEFINED = 255,
    E_ADI_ERR = PROS_ERR as isize,
}

extern "C"{
    pub fn adi_port_get_config(port: uint8_t) -> adi_port_config_e_t;
    pub fn adi_port_get_value(port: uint8_t) -> int32_t;
    pub fn adi_port_set_config(port: uint8_t, adi_type: adi_port_config_e_t) -> int32_t;
    pub fn adi_port_set_value(port: uint8_t, value: int32_t) -> int32_t;
}
