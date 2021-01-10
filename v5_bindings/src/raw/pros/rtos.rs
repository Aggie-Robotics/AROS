#![allow(non_camel_case_types)]

use cty::*;

pub type task_t = *mut c_void;
pub type task_fn_t = extern fn(arg: *mut c_void);

pub const TASK_PRIORITY_DEFAULT: u32 = 8;
pub const TASK_STACK_DEPTH_DEFAULT: u16 = 0x2000;
pub const TIMEOUT_MAX: u32 = 0xffffffff;

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum task_state_e_t{
    E_TASK_STATE_RUNNING = 0,
    E_TASK_STATE_READY,
    E_TASK_STATE_BLOCKED,
    E_TASK_STATE_SUSPENDED,
    E_TASK_STATE_DELETED,
    E_TASK_STATE_INVALID,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum notify_action_e_t{
    E_NOTIFY_ACTION_NONE,
    E_NOTIFY_ACTION_BITS,
    E_NOTIFY_ACTION_INCR,
    E_NOTIFY_ACTION_OWRITE,
    E_NOTIFY_ACTION_NO_OWRITE,
}

pub type mutex_t = *mut c_void;

extern "C"{
    pub fn millis() -> uint32_t;

    pub fn task_create(function: task_fn_t, parameters: *mut c_void, priority: uint32_t, stack_depth: uint16_t, name: *const c_char) -> task_t;
    pub fn task_delete(task: task_t);
    pub fn task_delay(milliseconds: uint32_t);
    pub fn delay(milliseconds: uint32_t);
    pub fn task_delay_until(prev_time: *mut uint32_t, delta: uint32_t);
    pub fn task_get_priority(task: task_t) -> uint32_t;
    pub fn task_set_priority(task: task_t, priority: uint32_t);
    pub fn task_get_state(task: task_t) -> task_state_e_t;
    pub fn task_suspend(task: task_t);
    pub fn task_resume(task: task_t);
    pub fn task_get_count() -> uint32_t;
    pub fn task_get_name(task: task_t) -> *mut c_char;
    pub fn task_get_by_name(name: *const c_char) -> task_t;
    pub fn task_get_current() -> task_t;
    pub fn task_notify(task: task_t) -> uint32_t;
    pub fn task_notify_ext(task: task_t, value: uint32_t, action: notify_action_e_t, prev_value: *mut uint32_t) -> uint32_t;
    pub fn task_notify_take(clear_on_exit: bool, timeout: uint32_t) -> uint32_t;
    pub fn task_notify_clear(task: task_t) -> bool;
    pub fn mutex_create() -> mutex_t;
    pub fn mutex_take(mutex: mutex_t, timeout: uint32_t) -> bool;
    pub fn mutex_give(mutex: mutex_t) -> bool;
}
