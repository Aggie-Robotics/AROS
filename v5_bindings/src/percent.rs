use alloc::string::String;
use core::convert::TryFrom;

use crate::error::NumericError;

pub struct Percent {
    pub value: u8,
}

impl Percent {
    pub fn new(value: u8) -> Result<Self, NumericError<u8>> {
        if value > 100 {
            Err(NumericError::new(value, String::from("Percent")))
        } else {
            Ok(Self {
                value
            })
        }
    }

    pub const unsafe fn new_unchecked(value: u8) -> Self {
        Self {
            value
        }
    }
}

impl TryFrom<u8> for Percent {
    type Error = NumericError<u8>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
