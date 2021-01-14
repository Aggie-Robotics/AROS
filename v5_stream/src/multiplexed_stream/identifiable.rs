use core::any::Any;
use core::fmt::{Debug, Display};
use core::marker::{Send, Sync};
use serde::de::Deserialize;
use serde::ser::Serialize;
use crate::multiplexed_stream::TypeIdType;

/// The id of a type to unify the type system
/// Ids 0-100 are reserved for system use
pub trait Identifiable: 'static + Serialize + for<'a> Deserialize<'a> + Any + Debug + Send + Sync{
    type NameType: Display;
    const ID: TypeIdType;
    const NAME: Self::NameType;
}
