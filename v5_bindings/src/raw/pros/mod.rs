use cty::c_int;
use num_derive::*;
use num_traits::FromPrimitive;

use crate::raw::pros::api::{PROS_ERR, PROS_ERR_F};

pub mod adi;
pub mod api;
pub mod apix;
pub mod misc;
pub mod motors;
pub mod rtos;
pub mod vision;

extern "C" {
    pub fn get_errno() -> c_int;
}
pub fn get_error() -> Option<ProsError>{
    ProsError::from_i32(unsafe{get_errno()})
}
pub fn is_i_error(val: i32) -> bool{
    val == PROS_ERR
}
pub fn is_f_error(val: f64) -> bool{
    val == PROS_ERR_F
}
pub fn check_error(val: i32, source: &str) -> i32{
    if is_i_error(val) {
        panic!("Error from {}: {:?}", source, get_error());
    }
    val
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, FromPrimitive)]
pub enum ProsError {
    EPERM = 1,
    ENOENT = 2,
    ESRCH = 3,
    EINTR = 4,
    EIO = 5,
    ENXIO = 6,
    E2BIG = 7,
    ENOEXEC = 8,
    EBADF = 9,
    ECHILD = 10,
    EAGAIN = 11,
    ENOMEM = 12,
    EACCES = 13,
    EFAULT = 14,
    ENOTBLK = 15,
    EBUSY = 16,
    EEXIST = 17,
    EXDEV = 18,
    ENODEV = 19,
    ENOTDIR = 20,
    EISDIR = 21,
    EINVAL = 22,
    ENFILE = 23,
    EMFILE = 24,
    ENOTTY = 25,
    ETXTBSY = 26,
    EFBIG = 27,
    ENOSPC = 28,
    ESPIPE = 29,
    EROFS = 30,
    EMLINK = 31,
    EPIPE = 32,
    EDOM = 33,
    ERANGE = 34,
}
