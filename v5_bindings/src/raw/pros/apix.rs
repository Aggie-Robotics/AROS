#![allow(non_camel_case_types)]

use cty::*;

pub type queue_t = *mut c_void;
pub type sem_t = *mut c_void;

extern "C" {
    pub fn sem_delete(sem: sem_t);

    pub fn queue_create(length: uint32_t, item_size: uint32_t) -> queue_t;
    pub fn queue_prepend(queue: queue_t, item: *const c_void, timeout: uint32_t) -> bool;
    pub fn queue_append(queue: queue_t, item: *const c_void, timeout: uint32_t) -> bool;
    pub fn queue_peek(queue: queue_t, buffer: *mut c_void, timeout: uint32_t) -> bool;
    pub fn queue_recv(queue: queue_t, buffer: *mut c_void, timeout: uint32_t) -> bool;
    pub fn queue_get_waiting(queue: queue_t) -> uint32_t;
    pub fn queue_delete(queue: queue_t);
    pub fn queue_reset(queue: queue_t);

    pub fn serctl(action: uint32_t, extra_arg: *mut c_void);
    pub fn fdctl(file: c_int, action: uint32_t, extra_arg: *mut c_void);
}

pub static SERCTL_ACTIVATE: uint32_t = 10;
pub static SERCTL_DEACTIVATE: uint32_t = 11;
pub static SERCTL_BLKWRITE: uint32_t = 12;
pub static SERCTL_NOBLKWRITE: uint32_t = 13;
pub static SERCTL_ENABLE_COBS: uint32_t = 14;
pub static SERCTL_DISABLE_COBS: uint32_t = 15;
pub static DEVCTL_FIONREAD: uint32_t = 16;
pub static DEVCTL_FIONWRITE: uint32_t = 18;
pub static DEVCTL_SET_BAUDRATE: uint32_t = 17;
