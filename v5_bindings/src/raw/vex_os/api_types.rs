#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use cty::*;
use rgb::{RGB, RGB8};

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Default)]
pub struct time {
    /// Hours
    pub ti_hour: uint8_t,
    /// Minutes
    pub ti_min: uint8_t,
    /// Seconds
    pub ti_sec: uint8_t,
    /// Hundredths of seconds
    pub ti_hund: uint8_t,
}

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Default)]
pub struct date {
    /// Year - 1980
    pub da_year: uint16_t,
    /// Day of the month
    pub da_day: uint8_t,
    /// Month (1 = Jan)
    pub da_mon: uint8_t,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum V5_DeviceType {
    kDeviceTypeNoSensor = 0,
    kDeviceTypeMotorSensor = 2,
    kDeviceTypeLedSensor = 3,
    kDeviceTypeAbsEncSensor = 4,
    kDeviceTypeCrMotorSensor = 5,
    kDeviceTypeImuSensor = 6,
    kDeviceTypeRangeSensor = 7,
    kDeviceTypeRadioSensor = 8,
    kDeviceTypeTetherSensor = 9,
    kDeviceTypeBrainSensor = 10,
    kDeviceTypeVisionSensor = 11,
    kDeviceTypeAdiSensor = 12,
    kDeviceTypeBumperSensor = 0x40,
    kDeviceTypeGyroSensor = 0x46,
    kDeviceTypeSonarSensor = 0x47,
    kDeviceTypeGenericSensor = 128,
    kDeviceTypeGenericSerial = 129,
    kDeviceTypeUndefinedSensor = 255,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct _V5_Device {
    _private: [u8; 0],
}

pub type V5_DeviceT = *mut _V5_Device;

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum V5_ControllerIndex {
    AnaLeftX = 0,
    AnaLeftY,
    AnaRightX,
    AnaRightY,
    AnaSpare1,
    AnaSpare2,

    Button5U,
    Button5D,
    Button6U,
    Button6D,
    Button7U,
    Button7D,
    Button7L,
    Button7R,
    Button8U,
    Button8D,
    Button8L,
    Button8R,

    ButtonSEL,

    BatteryLevel,

    ButtonAll,
    Flags,
    BatteryCapacity,
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum V5_ControllerIndexFinal {
    // Final V5 names
    Axis1 = V5_ControllerIndex::AnaRightX as isize,
    Axis2 = V5_ControllerIndex::AnaRightY as isize,
    Axis3 = V5_ControllerIndex::AnaLeftY as isize,
    Axis4 = V5_ControllerIndex::AnaLeftX as isize,
    ButtonL1 = V5_ControllerIndex::Button5U as isize,
    ButtonL2 = V5_ControllerIndex::Button5D as isize,
    ButtonR1 = V5_ControllerIndex::Button6U as isize,
    ButtonR2 = V5_ControllerIndex::Button6D as isize,
    ButtonUp = V5_ControllerIndex::Button7U as isize,
    ButtonDown = V5_ControllerIndex::Button7D as isize,
    ButtonLeft = V5_ControllerIndex::Button7L as isize,
    ButtonRight = V5_ControllerIndex::Button7R as isize,
    ButtonX = V5_ControllerIndex::Button8U as isize,
    ButtonB = V5_ControllerIndex::Button8D as isize,
    ButtonY = V5_ControllerIndex::Button8L as isize,
    ButtonA = V5_ControllerIndex::Button8R as isize,
}
impl From<V5_ControllerIndexFinal> for V5_ControllerIndex {
    fn from(final_index: V5_ControllerIndexFinal) -> Self {
        match final_index {
            V5_ControllerIndexFinal::Axis1 => V5_ControllerIndex::AnaRightX,
            V5_ControllerIndexFinal::Axis2 => V5_ControllerIndex::AnaRightY,
            V5_ControllerIndexFinal::Axis3 => V5_ControllerIndex::AnaLeftY,
            V5_ControllerIndexFinal::Axis4 => V5_ControllerIndex::AnaLeftX,
            V5_ControllerIndexFinal::ButtonL1 => V5_ControllerIndex::Button5U,
            V5_ControllerIndexFinal::ButtonL2 => V5_ControllerIndex::Button5D,
            V5_ControllerIndexFinal::ButtonR1 => V5_ControllerIndex::Button6U,
            V5_ControllerIndexFinal::ButtonR2 => V5_ControllerIndex::Button6D,
            V5_ControllerIndexFinal::ButtonUp => V5_ControllerIndex::Button7U,
            V5_ControllerIndexFinal::ButtonDown => V5_ControllerIndex::Button7D,
            V5_ControllerIndexFinal::ButtonLeft => V5_ControllerIndex::Button7L,
            V5_ControllerIndexFinal::ButtonRight => V5_ControllerIndex::Button7R,
            V5_ControllerIndexFinal::ButtonX => V5_ControllerIndex::Button8U,
            V5_ControllerIndexFinal::ButtonB => V5_ControllerIndex::Button8D,
            V5_ControllerIndexFinal::ButtonY => V5_ControllerIndex::Button8L,
            V5_ControllerIndexFinal::ButtonA => V5_ControllerIndex::Button8R,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum V5_ControllerStatus {
    kV5ControllerOffline = 0,
    kV5ControllerTethered,
    kV5ControllerVexnet,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum V5_ControllerId {
    kControllerMaster = 0,
    kControllerPartner,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum V5_DeviceLedColor {
    kLedColorBlack = 0,
    kLedColorRed = 0xFF0000,
    kLedColorGreen = 0x00FF00,
    kLedColorBlue = 0x0000FF,
    kLedColorYellow = 0xFFFF00,
    kLedColorCyan = 0x00FFFF,
    kLedColorMagenta = 0xFF00FF,
    kLedColorWhite = 0xFFFFFF,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum V5_AdiPortConfiguration {
    kAdiPortTypeAnalogIn = 0,
    kAdiPortTypeAnalogOut,
    kAdiPortTypeDigitalIn,
    kAdiPortTypeDigitalOut,

    kAdiPortTypeSmartButton,
    kAdiPortTypeSmartPot,

    kAdiPortTypeLegacyButton,
    kAdiPortTypeLegacyPotentiometer,
    kAdiPortTypeLegacyLineSensor,
    kAdiPortTypeLegacyLightSensor,
    kAdiPortTypeLegacyGyro,
    kAdiPortTypeLegacyAccelerometer,

    kAdiPortTypeLegacyServo,
    kAdiPortTypeLegacyPwm,

    kAdiPortTypeQuadEncoder,
    kAdiPortTypeSonar,

    kAdiPortTypeLegacyPwmSlew,

    kAdiPortTypeUndefined = 255,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum V5_DeviceBumperState {
    kBumperReleased = 0,
    kBumperPressed = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum V5MotorControlMode {
    kMotorControlModeOFF = 0,
    kMotorControlModeBRAKE = 1,
    kMotorControlModeHOLD = 2,
    kMotorControlModeSERVO = 3,
    kMotorControlModePROFILE = 4,
    kMotorControlModeVELOCITY = 5,
    kMotorControlModeUNDEFINED = 6,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum V5MotorBrakeMode {
    kV5MotorBrakeModeCoast = 0,
    kV5MotorBrakeModeBrake = 1,
    kV5MotorBrakeModeHold = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum V5MotorEncoderUnits {
    kMotorEncoderDegrees = 0,
    kMotorEncoderRotations = 1,
    kMotorEncoderCounts = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum V5MotorGearset {
    ///Red
    kMotorGearSet_36 = 0,
    ///Green
    kMotorGearSet_18 = 1,
    ///Blue
    kMotorGearSet_06 = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct V5_DeviceMotorPid {
    kf: uint8_t,
    kp: uint8_t,
    ki: uint8_t,
    kd: uint8_t,
    filter: uint8_t,
    pad1: uint8_t,
    limit: uint16_t,
    threshold: uint8_t,
    loopspeed: uint8_t,
    pad2: [uint8_t; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum V5VisionMode {
    kVisionModeNormal = 0,
    kVisionModeMixed = 1,
    kVisionModeLineDetect = 2,
    kVisionTypeTest = 3,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum V5VisionBlockType {
    kVisionTypeNormal = 0,
    kVisionTypeColorCode = 1,
    kVisionTypeLineDetect = 2,
}
impl Default for V5VisionBlockType{
    fn default() -> Self {
        V5VisionBlockType::kVisionTypeNormal
    }
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum V5VisionWBMode {
    kVisionWBNormal = 0,
    kVisionWBStart = 1,
    kVisionWBManual = 2,
}

impl From<bool> for V5VisionWBMode {
    fn from(from: bool) -> Self {
        if from {
            Self::kVisionWBStart
        } else {
            Self::kVisionWBNormal
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum V5VisionLedMode {
    kVisionLedModeAuto = 0,
    kVisionLedModeManual = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum V5VisionWifiMode {
    Off = 0,
    On = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct V5_DeviceVisionSignature {
    pub id: uint8_t,
    pub flags: uint8_t,
    pub pad: [uint8_t; 2],
    pub range: c_float,
    pub uMin: int32_t,
    pub uMax: int32_t,
    pub uMean: int32_t,
    pub vMin: int32_t,
    pub vMax: int32_t,
    pub vMean: int32_t,
    pub mRgb: uint32_t,
    pub mType: uint32_t,
}
impl V5_DeviceVisionSignature{
    pub fn new(id: u8, u_min_max_mean: [i32; 3], v_min_max_mean: [i32; 3], range: f32, vision_type: u32) -> Self {
        Self {
            id,
            flags: 0,
            pad: [0; 2],
            range,
            uMin: u_min_max_mean[0],
            uMax: u_min_max_mean[1],
            uMean: u_min_max_mean[2],
            vMin: v_min_max_mean[0],
            vMax: v_min_max_mean[1],
            vMean: v_min_max_mean[2],
            mRgb: 0,
            mType: vision_type,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct V5_DeviceVisionObject {
    signature: uint16_t,
    vision_type: V5VisionBlockType,
    x_offset: uint16_t,
    y_offset: uint16_t,
    width: uint16_t,
    height: uint16_t,
    angle: uint16_t,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct V5_DeviceVisionRgb {
    pub red: uint8_t,
    pub green: uint8_t,
    pub blue: uint8_t,
    /// not used yet
    pub brightness: uint8_t,
}

impl V5_DeviceVisionRgb {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        V5_DeviceVisionRgb {
            red,
            green,
            blue,
            brightness: Default::default(),
        }
    }
}

impl<T> From<RGB<T>> for V5_DeviceVisionRgb where T: Into<u8> {
    fn from(from: RGB<T>) -> Self {
        Self::new(from.r.into(), from.g.into(), from.b.into())
    }
}

impl From<V5_DeviceVisionRgb> for RGB8 {
    fn from(from: V5_DeviceVisionRgb) -> Self {
        Self::new(from.red, from.green, from.blue)
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct V5_DeviceImuQuaternion {
    a: c_double,
    b: c_double,
    c: c_double,
    d: c_double,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct V5_DeviceImuAttitude {
    pitch: c_double,
    roll: c_double,
    yaw: c_double,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct V5_DeviceImuRaw {
    x: c_double,
    y: c_double,
    z: c_double,
    w: c_double,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum _V5ImuHeadingnMode {
    kImuHeadingNative = 0x00,
    kImuHeadingIQ = 0x01,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum V5ImuOrientationMode {
    kImuOrientationZUp = 0x00,
    kImuOrientationZDown = 0x10,
    kImuOrientationXUp = 0x20,
    kImuOrientationXDown = 0x30,
    kImuOrientationYUp = 0x40,
    kImuOrientationYDown = 0x50,
    kImuOrientationAuto = 0x80,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum V5ImuQuaternionMode {
    kImuQuaternionProcessed = 0x000,
    kImuQuaternionRaw = 0x100,
}

pub type FIL = c_void;

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FRESULT {
    FR_OK = 0,
    FR_DISK_ERR,
    FR_INT_ERR,
    FR_NOT_READY,
    FR_NO_FILE,
    FR_NO_PATH,
    FR_INVALID_NAME,
    FR_DENIED,
    FR_EXIST,
    FR_INVALID_OBJECT,
    FR_WRITE_PROTECTED,
    FR_INVALID_DRIVE,
    FR_NOT_ENABLED,
    FR_NO_FILESYSTEM,
    FR_MKFS_ABORTED,
    FR_TIMEOUT,
    FR_LOCKED,
    FR_NOT_ENOUGH_CORE,
    FR_TOO_MANY_OPEN_FILES,
    FR_INVALID_PARAMETER,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum V5_TouchEvent {
    kTouchEventRelease,
    kTouchEventPress,
    kTouchEventPressAuto,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct V5_TouchStatus {
    lastEvent: V5_TouchEvent,
    lastXpos: int16_t,
    lastYpos: int16_t,
    pressCount: int32_t,
    releaseCount: int32_t,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct v5_image {
    width: uint16_t,
    height: uint16_t,
    data: *mut uint32_t,
    p: *mut uint32_t,
}
