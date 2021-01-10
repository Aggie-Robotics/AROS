use core::fmt::Display;

use cstr_core::CString;
use cty::*;

pub mod pros;
pub mod vex_os;

extern "C"{
    pub fn exit(code: c_int) -> !;

    pub fn read(fd: c_int, buf: *mut c_void, nbytes: size_t) -> ssize_t;
    pub fn write(fd: c_int, buf: *const c_void, n: size_t) -> ssize_t;
    pub fn open(file: *const c_char, oflag: c_int, ...) -> c_int;
    pub fn close(fd: c_int) -> c_int;
}

pub static O_RDONLY: c_int = 00;
pub static O_WRONLY: c_int = 01;

pub fn str_to_char_ptr(str: &str) -> CString{
    CString::new(str).expect(format!("Cannot create CString! from {}", str).as_str())
}
pub fn fmt_to_char_ptr(fmt: &impl Display) -> CString{
    CString::new(format!("{}", fmt)).expect(format!("Cannot create CString! from {}", fmt).as_str())
}
