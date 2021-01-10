use cty::*;

pub const PROS_ERR: i32 = i32::max_value();
pub const PROS_ERR_F: f64 = f64::INFINITY;

extern "C"{
    pub fn printf(format: *const c_char, ...);
}
