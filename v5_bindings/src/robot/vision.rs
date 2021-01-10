use alloc::string::String;
use alloc::vec::Vec;
use core::convert::{TryFrom, TryInto};
use core::mem::MaybeUninit;

use rgb::{RGB, RGB8};

use crate::error::NumericError;
use crate::percent::Percent;
use crate::raw::vex_os::api::*;
use crate::raw::vex_os::api_types::{V5_DeviceVisionSignature, V5VisionLedMode};
pub use crate::raw::vex_os::api_types::V5_DeviceVisionObject as Object;
pub use crate::raw::vex_os::api_types::V5_DeviceVisionRgb as ColorCode;
pub use crate::raw::vex_os::api_types::V5VisionWifiMode as WifiMode;
use crate::robot::port::Port;

#[derive(Copy, Clone, Debug)]
pub struct Signature {
    pub u_min_max_mean: [i32; 3],
    pub v_min_max_mean: [i32; 3],
    pub range: f32,
    pub vision_type: u32,
}

impl From<(SignatureIndex, Signature)> for V5_DeviceVisionSignature {
    fn from(from: (SignatureIndex, Signature)) -> Self {
        Self::new(from.0.index, from.1.u_min_max_mean, from.1.v_min_max_mean, from.1.range, from.1.vision_type)
    }
}

impl From<V5_DeviceVisionSignature> for (SignatureIndex, Signature) {
    fn from(from: V5_DeviceVisionSignature) -> Self {
        (SignatureIndex::new(from.id).expect("Bad signature ID from vex!"), Signature {
            u_min_max_mean: [from.uMin, from.uMax, from.uMean],
            v_min_max_mean: [from.vMin, from.vMax, from.vMean],
            range: from.range,
            vision_type: from.mType,
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct SignatureIndex {
    index: u8,
}

impl SignatureIndex {
    pub fn new(index: u8) -> Result<Self, NumericError<u8>> {
        if index > 7 {
            Err(NumericError::new(index, String::from("SignatureIndex")))
        } else {
            Ok(Self {
                index
            })
        }
    }

    pub const unsafe fn new_unchecked(index: u8) -> Self {
        Self {
            index
        }
    }
}

impl TryFrom<u8> for SignatureIndex {
    type Error = NumericError<u8>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[derive(Debug)]
pub struct Vision {
    port: Port,
}

impl Vision {
    pub fn new(port: Port) -> Self {
        let out = Self {
            port
        };
        unsafe { vexDeviceVisionLedModeSet(out.port.device(), V5VisionLedMode::kVisionLedModeManual) };
        out
    }

    pub fn object_count(&self) -> usize {
        unsafe { vexDeviceVisionObjectCountGet(self.port.device()) as usize }
    }
    pub fn objects(&self) -> Vec<Object> {
        let count = self.object_count();
        let mut out = Vec::with_capacity(count);
        for i in 0..count {
            let mut object = MaybeUninit::uninit();
            unsafe {
                vexDeviceVisionObjectGet(self.port.device(), i as u32, object.as_mut_ptr());
                out.push(object.assume_init());
            }
        }
        out
    }

    pub fn signature(&self, index: SignatureIndex) -> (SignatureIndex, Signature) {
        let mut out = MaybeUninit::uninit();
        unsafe {
            vexDeviceVisionSignatureGet(self.port.device(), index.index as u32, out.as_mut_ptr());
            out.assume_init().into()
        }
    }
    pub fn set_signature(&mut self, index: SignatureIndex, signature: Signature) {
        unsafe { vexDeviceVisionSignatureSet(self.port.device(), &mut (index, signature).into()) }
    }

    pub fn auto_white_balance(&mut self, enable: bool) {
        unsafe { vexDeviceVisionWhiteBalanceModeSet(self.port.device(), enable.into()) }
    }

    pub fn white_balance(&self) -> RGB8 {
        unsafe { vexDeviceVisionWhiteBalanceGet(self.port.device()) }.into()
    }
    pub fn set_white_balance<T>(&mut self, white_balance: RGB<T>) where T: Into<u8> {
        unsafe { vexDeviceVisionWhiteBalanceSet(self.port.device(), white_balance.into()) }
    }

    pub fn brightness(&self) -> u32 {
        (unsafe { vexDeviceVisionBrightnessGet(self.port.device()) as u32 } * 255 + 50) / 100
    }
    pub fn set_brightness(&mut self, brightness: u8) {
        unsafe { vexDeviceVisionBrightnessSet(self.port.device(), ((brightness as u32 * 100 + 50) / 255) as u8) }
    }

    pub fn led(&self) -> RGB8 {
        unsafe { vexDeviceVisionLedColorGet(self.port.device()) }.into()
    }
    pub fn set_led<T>(&mut self, color: RGB<T>) where T: Into<u8> {
        unsafe { vexDeviceVisionLedColorSet(self.port.device(), color.into()) }
    }

    pub fn led_brightness(&self) -> Percent {
        unsafe { vexDeviceVisionBrightnessGet(self.port.device()) }.try_into().expect("Vex gave percent > 100!")
    }
    pub fn set_led_brightness(&mut self, percent: Percent) {
        unsafe { vexDeviceVisionBrightnessSet(self.port.device(), percent.value) }
    }

    pub fn wifi_mode(&self) -> WifiMode {
        unsafe { vexDeviceVisionWifiModeGet(self.port.device()) }
    }
    pub fn set_wifi_mode(&mut self, mode: WifiMode) {
        unsafe { vexDeviceVisionWifiModeSet(self.port.device(), mode) }
    }
}
