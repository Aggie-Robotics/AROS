use core::fmt::{Display, Debug};
use core::marker::Send;
use serde::de::Deserialize;
use serde::ser::Serialize;
use crate::multiplexed_stream::TypeIdType;

/// The id of a type to unify the type system
/// Ids 0-100 are reserved for system use
pub trait Identifiable: 'static + Debug + Serialize + for<'a> Deserialize<'a> + Send{
    type NameType: Display;
    const ID: TypeIdType;
    const NAME: Self::NameType;
}
