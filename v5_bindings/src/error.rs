use alloc::string::String;
use core::fmt::Debug;

#[derive(Debug)]
pub struct Error {
    pub msg: String,
}

#[derive(Clone, Debug)]
pub struct NumericError<T> {
    value: T,
    name: String,
}

impl<T> NumericError<T> {
    pub const fn new(value: T, name: String) -> Self {
        Self {
            value,
            name,
        }
    }
}

impl<T> From<NumericError<T>> for Error where T: Debug {
    fn from(from: NumericError<T>) -> Self {
        Self {
            msg: format!("Numeric {} out of bounds: {:?}", from.name, from.value)
        }
    }
}
