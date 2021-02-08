use core::fmt::{Debug, Display};
use alloc::boxed::Box;
use core::convert::Infallible;

pub trait Error: 'static + Debug{
    fn is_recoverable(&self) -> bool;

}
pub fn handle_error<E>(error: E, on_recoverable: impl FnOnce(E)) -> Result<(), E> where E: Error{
    if error.is_recoverable(){
        on_recoverable(error);
        Ok(())
    }
    else{
        Err(error)
    }
}

#[derive(Debug)]
pub struct CustomError{
    pub message: Box<dyn Debug>,
    pub is_recoverable: bool,
}
impl CustomError{
    #[inline]
    pub fn new(is_recoverable: bool, message: impl Debug + 'static) -> Self{
        Self{ message: Box::new(message), is_recoverable }
    }

    #[inline]
    pub fn from_error(function_name: impl Display, error: impl Error) -> Self{
        Self::new(error.is_recoverable(), format!("{}: {:?}", function_name, error))
    }
}
impl Error for CustomError{
    fn is_recoverable(&self) -> bool {
        self.is_recoverable
    }
}

#[derive(Debug)]
pub enum ComboError<E1, E2> where E1: Error, E2: Error{
    Error1(E1),
    Error2(E2),
}
impl<E1, E2> Error for ComboError<E1, E2> where E1: Error, E2: Error{
    fn is_recoverable(&self) -> bool {
        match self {
            ComboError::Error1(error) => error.is_recoverable(),
            ComboError::Error2(error) => error.is_recoverable(),
        }
    }
}
impl<E1, E2> From<E1> for ComboError<E1, E2> where E1: Error, E2: Error{
    fn from(from: E1) -> Self {
        Self::Error1(from)
    }
}

impl Error for Infallible{
    fn is_recoverable(&self) -> bool {
        true
    }
}
