#![allow(non_camel_case_types)]

use cty::*;
use cty::{c_float, int32_t, uint32_t, uint8_t};
pub use cty::uint16_t as vision_color_code_t;

pub use vision_object as vision_object_s_t;
pub use vision_object_type as vision_object_type_e_t;
pub use vision_signature as vision_signature_s_t;
pub use vision_zero as vision_zero_e_t;

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum vision_object_type {
    E_VISION_OBJECT_NORMAL = 0,
    E_VISION_OBJECT_COLOR_CODE = 1,
    E_VISION_OBJECT_LINE = 2
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct vision_signature{
    pub id: uint8_t,
    pub _pad: [uint8_t; 3],
    pub range: c_float,
    pub u_min: int32_t,
    pub u_max: int32_t,
    pub u_mean: int32_t,
    pub v_min: int32_t,
    pub v_max: int32_t,
    pub v_mean: int32_t,
    pub rgb: uint32_t,
    pub signature_type: uint32_t,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct vision_object{
    pub signature: uint16_t,
    pub object_type: vision_object_type_e_t,
    pub left_coord: int16_t,
    pub top_coord: int16_t,
    pub width: int16_t,
    pub height: int16_t,
    pub angle: uint16_t,
    pub x_middle_coord: int16_t,
    pub y_middle_coord: int16_t,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum vision_zero {
    E_VISION_ZERO_TOPLEFT = 0,  // (0,0) coordinate is the top left of the FOV
    E_VISION_ZERO_CENTER = 1    // (0,0) coordinate is the center of the FOV
}

extern "C"{
    pub fn vision_clear_led(port: uint8_t) -> int32_t;
    pub fn vision_signature_from_utility(id: int32_t, u_min: int32_t, u_max: int32_t, u_mean: int32_t, v_min: int32_t, v_max: int32_t, v_mean: int32_t, range: c_float, signature_type: int32_t) -> vision_signature_s_t;
    pub fn vision_create_color_code(port: uint8_t, sig_id1: uint32_t, sig_id2: uint32_t, sig_id3: uint32_t, sig_id4: uint32_t, sig_id5: uint32_t) -> vision_color_code_t;
    pub fn vision_get_by_size(port: uint8_t, size_id: uint32_t) -> vision_object_s_t;
    pub fn vision_get_by_sig(port: uint8_t, size_id: uint32_t) -> vision_object_s_t;
    pub fn vision_get_by_code(port: uint8_t, size_id: uint32_t, color_code: vision_color_code_t) -> vision_object_s_t;
    pub fn vision_get_exposure(port: uint8_t) -> int32_t;
    pub fn vision_get_object_count(port: uint8_t) -> int32_t;
    pub fn vision_get_white_balance(port: uint8_t) -> int32_t;
    pub fn vision_print_signature(sig: vision_signature_s_t) -> int32_t;
    pub fn vision_read_by_size(port: uint8_t, size_id: uint32_t, object_count: uint32_t, object_arr: *mut vision_object_s_t) -> int32_t;
    pub fn vision_read_by_sig(port: uint8_t, size_id: uint32_t, sig_id: uint32_t, object_count: uint32_t, object_arr: *mut vision_object_s_t) -> int32_t;
    pub fn vision_read_by_code(port: uint8_t, size_id: uint32_t, color_code: vision_color_code_t, object_count: uint32_t, object_arr: *mut vision_object_s_t) -> int32_t;
    pub fn vision_get_signature(port: uint8_t, signature_id: uint8_t) -> vision_signature_s_t;
    pub fn vision_set_signature(port: uint8_t, signature_id: uint8_t, signature_ptr: *const vision_signature_s_t) -> int32_t;
    pub fn vision_set_auto_white_balance(port: uint8_t, enable: uint8_t) -> int32_t;
    pub fn vision_set_exposure(port: uint8_t, exposure: uint8_t) -> int32_t;
    pub fn vision_set_led(port: uint8_t, rgb: int32_t) -> int32_t;
    pub fn vision_set_white_balance(port: uint8_t, rgb: int32_t) -> int32_t;
    pub fn vision_set_zero_point(port: uint8_t, zero_point: vision_zero_e_t) -> int32_t;
    pub fn vision_set_wifi_mode(port: uint8_t, enable: uint8_t) -> int32_t;
}

