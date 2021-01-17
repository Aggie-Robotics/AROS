use serde::{Deserialize, Serialize};

pub type IdentifiableIDType = i64;
pub trait Identifiable: 'static + Send + Serialize + for<'de> Deserialize<'de>{
    const ID: IdentifiableIDType;
    const NAME: &'static str;
}

impl Identifiable for u8{
    const ID: i64 = -1;
    const NAME: &'static str = "u8";
}
impl Identifiable for u16{
    const ID: i64 = -2;
    const NAME: &'static str = "u16";
}
impl Identifiable for u32{
    const ID: i64 = -3;
    const NAME: &'static str = "u32";
}
impl Identifiable for u64{
    const ID: i64 = -4;
    const NAME: &'static str = "u64";
}
impl Identifiable for u128{
    const ID: i64 = -5;
    const NAME: &'static str = "u128";
}
impl Identifiable for i8{
    const ID: i64 = -6;
    const NAME: &'static str = "i8";
}
impl Identifiable for i16{
    const ID: i64 = -7;
    const NAME: &'static str = "i16";
}
impl Identifiable for i32{
    const ID: i64 = -8;
    const NAME: &'static str = "i32";
}
impl Identifiable for i64{
    const ID: i64 = -9;
    const NAME: &'static str = "i64";
}
impl Identifiable for i128{
    const ID: i64 = -10;
    const NAME: &'static str = "i128";
}
impl Identifiable for f32{
    const ID: i64 = -11;
    const NAME: &'static str = "f32";
}
impl Identifiable for f64{
    const ID: i64 = -12;
    const NAME: &'static str = "f64";
}
