#![allow(non_camel_case_types)]

use cty::*;

use crate::raw::vex_os::api_types::V5_ControllerId;

pub const COMPETITION_DISABLED: u8 = 1 << 0;
pub const COMPETITION_AUTONOMOUS: u8 = 1 << 1;
pub const COMPETITION_CONNECTED: u8 = 1 << 2;

extern "C"{
    pub fn competition_get_status() -> uint8_t;
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum controller_id_e_t{
    E_CONTROLLER_MASTER = 0,
    E_CONTROLLER_PARTNER,
}
impl From<V5_ControllerId> for controller_id_e_t{
    fn from(this: V5_ControllerId) -> Self {
        match this {
            V5_ControllerId::kControllerMaster => controller_id_e_t::E_CONTROLLER_MASTER,
            V5_ControllerId::kControllerPartner => controller_id_e_t::E_CONTROLLER_PARTNER,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum controller_analog_e_t{
    E_CONTROLLER_ANALOG_LEFT_X = 0,
    E_CONTROLLER_ANALOG_LEFT_Y,
    E_CONTROLLER_ANALOG_RIGHT_X,
    E_CONTROLLER_ANALOG_RIGHT_Y,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum controller_digital_e_t{
    E_CONTROLLER_DIGITAL_L1 = 6,
    E_CONTROLLER_DIGITAL_L2,
    E_CONTROLLER_DIGITAL_R1,
    E_CONTROLLER_DIGITAL_R2,
    E_CONTROLLER_DIGITAL_UP,
    E_CONTROLLER_DIGITAL_DOWN,
    E_CONTROLLER_DIGITAL_LEFT,
    E_CONTROLLER_DIGITAL_RIGHT,
    E_CONTROLLER_DIGITAL_X,
    E_CONTROLLER_DIGITAL_B,
    E_CONTROLLER_DIGITAL_Y,
    E_CONTROLLER_DIGITAL_A,
}

extern "C"{
    pub fn controller_is_connected(id: controller_id_e_t) -> int32_t;
    pub fn controller_get_analog(id: controller_id_e_t, channel: controller_analog_e_t) -> int32_t;
    pub fn controller_get_battery_capacity(id: controller_id_e_t) -> int32_t;
    pub fn controller_get_battery_level(id: controller_id_e_t) -> int32_t;
    pub fn controller_get_digital(id: controller_id_e_t, button: controller_digital_e_t) -> int32_t;
    pub fn controller_get_digital_new_press(id: controller_digital_e_t, button: controller_digital_e_t) -> int32_t;
    pub fn controller_print(id: controller_id_e_t, line: uint8_t, col: uint8_t, fmt: *const c_char, ...) -> int32_t;
    pub fn controller_set_text(id: controller_id_e_t, line: uint8_t, col: uint8_t, str: *const c_char) -> int32_t;
    pub fn controller_clear_line(id: controller_id_e_t, line: uint8_t) -> int32_t;
    pub fn controller_clear(id: controller_id_e_t) -> int32_t;
    pub fn controller_rumble(id: controller_id_e_t, rumble_pattern: *const c_char) -> int32_t;

    pub fn battery_get_voltage() -> int32_t;
    pub fn battery_get_temperature() -> c_double;
    pub fn battery_get_capacity() -> c_double;

    pub fn usd_is_installed() -> int32_t;
}
