#![allow(non_camel_case_types)]

use cty::*;

pub use motor_brake_mode_e as motor_brake_mode_e_t;
pub use motor_encoder_units_e as motor_encoder_units_e_t;
pub use motor_fault_e as motor_fault_e_t;
pub use motor_flag_e as motor_flag_e_t;
pub use motor_gearset_e as motor_gearset_e_t;
pub use motor_pid_full_s as motor_pid_full_s_t;
pub use motor_pid_s as motor_pid_s_t;

extern "C"{
    pub fn motor_move(port: uint8_t, voltage: int32_t) -> int32_t;
    pub fn motor_move_absolute(port: uint8_t, position: c_double, velocity: int32_t) -> int32_t;
    pub fn motor_move_relative(port: uint8_t, position: c_double, velocity: int32_t) -> int32_t;
    pub fn motor_move_velocity(port: uint8_t, velocity: int32_t) -> int32_t;
    pub fn motor_move_voltage(port: uint8_t, voltage: int32_t) -> int32_t;
    pub fn motor_modify_profiled_velocity(port: uint8_t, velocity: int32_t) -> int32_t;
    pub fn motor_get_target_position(port: uint8_t) -> c_double;
    pub fn motor_get_target_velocity(port: uint8_t) -> int32_t;
    pub fn motor_get_actual_velocity(port: uint8_t) -> c_double;
    pub fn motor_get_current_draw(port: uint8_t) -> int32_t;
    pub fn motor_get_direction(port: uint8_t) -> int32_t;
    pub fn motor_get_efficiency(port: uint8_t) -> c_double;
    pub fn motor_is_over_current(port: int8_t) -> int32_t;
    pub fn motor_is_over_temp(port: uint8_t) -> int32_t;
    pub fn motor_is_stopped(port: uint8_t) -> int32_t;
    pub fn motor_get_zero_position_flag(port: uint8_t) -> int32_t;
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum motor_fault_e{
    E_MOTOR_FAULT_NO_FAULTS = 0x00,
    E_MOTOR_FAULT_MOTOR_OVER_TEMP = 0x01,  // Analogous to motor_is_over_temp()
    E_MOTOR_FAULT_DRIVER_FAULT = 0x02,     // Indicates a motor h-bridge fault
    E_MOTOR_FAULT_OVER_CURRENT = 0x04,     // Analogous to motor_is_over_current()
    E_MOTOR_FAULT_DRV_OVER_CURRENT = 0x08  // Indicates an h-bridge over current
}

extern "C"{
    pub fn motor_get_faults(port: uint8_t) -> uint32_t;
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum motor_flag_e {
    E_MOTOR_FLAGS_NONE = 0x00,
    E_MOTOR_FLAGS_BUSY = 0x01,           // Cannot currently communicate to the motor
    E_MOTOR_FLAGS_ZERO_VELOCITY = 0x02,  // Analogous to motor_is_stopped()
    E_MOTOR_FLAGS_ZERO_POSITION = 0x04   // Analogous to motor_get_zero_position_flag()
}

extern "C"{
    pub fn motor_get_flags(port: uint8_t) -> uint32_t;
    pub fn motor_get_raw_position(port: uint8_t, timestamp: *mut uint32_t) -> int32_t;
    pub fn motor_get_position(port: uint8_t) -> c_double;
    pub fn motor_get_power(port: uint8_t) -> c_double;
    pub fn motor_get_temperature(port: uint8_t) -> c_double;
    pub fn motor_get_torque(port: uint8_t) -> c_double;
    pub fn motor_get_voltage(port: uint8_t) -> int32_t;
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum motor_brake_mode_e {
    E_MOTOR_BRAKE_COAST = 0,  // Motor coasts when stopped, traditional behavior
    E_MOTOR_BRAKE_BRAKE = 1,  // Motor brakes when stopped
    E_MOTOR_BRAKE_HOLD = 2,   // Motor actively holds position when stopped
    E_MOTOR_BRAKE_INVALID = i32::max_value() as isize,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum motor_encoder_units_e {
    E_MOTOR_ENCODER_DEGREES = 0,    // Position is recorded as angle in degrees
                                    // as a floating point number
    E_MOTOR_ENCODER_ROTATIONS = 1,  // Position is recorded as angle in rotations
                                    // as a floating point number
    E_MOTOR_ENCODER_COUNTS = 2,     // Position is recorded as raw encoder ticks
                                    // as a whole number
    E_MOTOR_ENCODER_INVALID = i32::max_value() as isize,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum motor_gearset_e {
    E_MOTOR_GEARSET_36 = 0,  // 36:1, 100 RPM, Red gear set
    E_MOTOR_GEARSET_18 = 1,  // 18:1, 200 RPM, Green gear set
    E_MOTOR_GEARSET_06 = 2,  // 6:1, 600 RPM, Blue gear set
    E_MOTOR_GEARSET_INVALID = i32::max_value() as isize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct motor_pid_full_s{
    kf: uint8_t,
    kp: uint8_t,
    ki: uint8_t,
    kd: uint8_t,
    filter: uint8_t,
    limit: uint16_t,
    threshold: uint8_t,
    loop_speed: uint8_t,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct motor_pid_s{
    kf: uint8_t,
    kp: uint8_t,
    ki: uint8_t,
    kd: uint8_t,
}

extern "C"{
    pub fn motor_set_zero_position(port: uint8_t, position: c_double) -> int32_t;
    pub fn motor_tare_position(port: uint8_t) -> int32_t;
    pub fn motor_set_brake_mode(port: uint8_t, mode: motor_brake_mode_e_t) -> int32_t;
    pub fn motor_set_current_limit(port: uint8_t, limit: int32_t) -> int32_t;
    pub fn motor_set_encoder_units(port: uint8_t, units: motor_encoder_units_e_t) -> int32_t;
    pub fn motor_set_gearing(port: uint8_t, gearset: motor_gearset_e_t) -> int32_t;
    pub fn motor_convert_pid(kf: c_double, kp: c_double, ki: c_double, kd: c_double) -> motor_pid_s_t;
    pub fn motor_convert_pid_full(kf: c_double, kp: c_double, ki: c_double, kd: c_double, filter: c_double, limit: c_double, threshold: c_double, loopspeed: c_double) -> motor_pid_full_s_t;
    pub fn motor_set_pos_pid(port: uint8_t, pid: motor_pid_s_t) -> int32_t;
    pub fn motor_set_pos_pid_full(port: uint8_t, pid: motor_pid_full_s_t) -> int32_t;
    pub fn motor_set_vel_pid(port: uint8_t, pid: motor_pid_s_t) -> int32_t;
    pub fn motor_set_vel_pid_full(port: uint8_t, pid: motor_pid_full_s_t) -> int32_t;
    pub fn motor_set_reversed(port: uint8_t, reverse: bool) -> int32_t;
    pub fn motor_set_voltage_limit(port: uint8_t, limits: int32_t) -> int32_t;
    pub fn motor_get_brake_mode(port: uint8_t) -> motor_brake_mode_e_t;
    pub fn motor_get_current_limit(port: uint8_t) -> int32_t;
    pub fn motor_get_encoder_units(port: uint8_t) -> motor_encoder_units_e_t;
    pub fn motor_get_gearing(port: uint8_t) -> motor_gearset_e_t;
    pub fn motor_get_pos_pid(port: uint8_t) -> motor_pid_full_s_t;
    pub fn motor_get_vel_pid(port: uint8_t) -> motor_pid_full_s_t;
    pub fn motor_is_reversed(port: uint8_t) -> int32_t;
    pub fn motor_get_voltage_limit(port: uint8_t) -> int32_t;
}
