#![no_std]
extern crate alloc;

use core::any::Any;
use serde::{Serialize, Deserialize};

pub trait AnySerialize: Any + Serialize{}
impl<T> AnySerialize for T where T: Any + Serialize{}
pub trait AnyDeserialize: Any + for<'a> Deserialize<'a>{}
impl<T> AnyDeserialize for T where T: Any + for<'a> Deserialize<'a>{}
pub trait AnySerde: AnySerialize + AnyDeserialize{}
impl<T> AnySerde for T where T: AnySerialize + AnyDeserialize{}
