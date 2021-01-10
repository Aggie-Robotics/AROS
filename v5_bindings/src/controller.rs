use core::fmt::Display;

pub use Id::kControllerMaster as Master;
pub use Id::kControllerPartner as Partner;
pub use Status::kV5ControllerOffline as Offline;
pub use Status::kV5ControllerTethered as Tethered;
pub use Status::kV5ControllerVexnet as Vexnet;

use crate::raw::fmt_to_char_ptr;
use crate::raw::vex_os::api::*;
use crate::raw::vex_os::api_types::{V5_ControllerIndex, V5_ControllerIndexFinal};
pub use crate::raw::vex_os::api_types::V5_ControllerId as Id;
pub use crate::raw::vex_os::api_types::V5_ControllerStatus as Status;

pub struct Controller{
    id: Id,
}
impl Controller{
    pub const fn get_all() -> [Self; 2] {
        [Self { id: Master }, Self { id: Partner }]
    }
    pub const fn id(&self) -> Id {
        self.id
    }

    pub fn axis(&self, axis: AxisIndex) -> i8 {
        (unsafe { vexControllerGet(self.id, axis.into()) }) as i8
    }
    pub const fn axis_max() -> i8 {
        127
    }
    pub const fn axis_min() -> i8 {
        -127
    }

    pub fn button(&self, button: ButtonIndex) -> bool{
        (unsafe {vexControllerGet(self.id, button.into())}) > 0
    }

    pub fn set_text(&mut self, line: u32, col: u32, fmt: &impl Display) -> bool{
        unsafe{vexControllerTextSet(self.id, line, col, fmt_to_char_ptr(fmt).as_ptr())}
    }

    pub fn status(&self) -> Status{
        unsafe {vexControllerConnectionStatusGet(self.id)}
    }

    pub fn battery_capacity(&self) -> i32{
        unsafe { vexControllerGet(self.id.into(), V5_ControllerIndex::BatteryCapacity) }
    }
    pub fn battery_level(&self) -> i32{
        unsafe { vexControllerGet(self.id.into(), V5_ControllerIndex::BatteryLevel) }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum AxisIndex {
    LeftX,
    LeftY,
    RightX,
    RightY,
}
impl From<AxisIndex> for V5_ControllerIndex{
    fn from(this: AxisIndex) -> Self {
        match this {
            AxisIndex::LeftX => V5_ControllerIndex::AnaLeftX,
            AxisIndex::LeftY => V5_ControllerIndex::AnaLeftY,
            AxisIndex::RightX => V5_ControllerIndex::AnaRightX,
            AxisIndex::RightY => V5_ControllerIndex::AnaRightY,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ButtonIndex {
    L1,
    L2,
    R1,
    R2,
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    X,
    Y,
}
impl From<ButtonIndex> for V5_ControllerIndex{
    fn from(this: ButtonIndex) -> Self {
        match this {
            ButtonIndex::L1 => V5_ControllerIndexFinal::ButtonL1.into(),
            ButtonIndex::L2 => V5_ControllerIndexFinal::ButtonL2.into(),
            ButtonIndex::R1 => V5_ControllerIndexFinal::ButtonR1.into(),
            ButtonIndex::R2 => V5_ControllerIndexFinal::ButtonR2.into(),
            ButtonIndex::Up => V5_ControllerIndexFinal::ButtonUp.into(),
            ButtonIndex::Down => V5_ControllerIndexFinal::ButtonDown.into(),
            ButtonIndex::Left => V5_ControllerIndexFinal::ButtonLeft.into(),
            ButtonIndex::Right => V5_ControllerIndexFinal::ButtonRight.into(),
            ButtonIndex::A => V5_ControllerIndexFinal::ButtonA.into(),
            ButtonIndex::B => V5_ControllerIndexFinal::ButtonB.into(),
            ButtonIndex::X => V5_ControllerIndexFinal::ButtonX.into(),
            ButtonIndex::Y => V5_ControllerIndexFinal::ButtonY.into(),
        }
    }
}
