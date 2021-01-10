#![allow(non_snake_case)]

use cty::*;

use crate::raw::vex_os::api_types::*;

extern "C" {
    pub fn vexDelay(timems: uint32_t);

    pub fn vexLedSet(index: uint32_t, value: V5_DeviceLedColor);
    pub fn vexLedRgbSet(index: uint32_t, color: uint32_t);
    pub fn vexLedGet(index: uint32_t) -> V5_DeviceLedColor;
    pub fn vexLedRgbGet(index: uint32_t) -> uint32_t;

    pub fn vexAdiPortConfigSet(index: uint32_t, port: uint32_t, adi_type: V5_AdiPortConfiguration);
    pub fn vexAdiPortConfigGet(index: uint32_t, port: uint32_t) -> V5_AdiPortConfiguration;
    pub fn vexAdiValueSet(index: uint32_t, port: uint32_t, value: int32_t);
    pub fn vexAdiValueGet(index: uint32_t, port: uint32_t) -> int32_t;

    pub fn vexBumperGet(index: uint32_t) -> V5_DeviceBumperState;

    pub fn vexGyroReset(index: uint32_t);
    pub fn vexGyroHeadingGet(index: uint32_t) -> c_double;
    pub fn vexGyroDegreesGet(index: uint32_t) -> c_double;

    pub fn vexSonarValueGet(index: uint32_t) -> int32_t;

    pub fn vexGenericValueGet(index: uint32_t) -> int32_t;

    pub fn vexMotorVelocitySet(index: uint32_t, velocity: int32_t);
    pub fn vexMotorVelocityUpdate(index: uint32_t, velocity: int32_t);
    pub fn vexMotorVoltageSet(index: uint32_t, value: int32_t);
    pub fn vexMotorVelocityGet(index: uint32_t) -> int32_t;
    pub fn vexMotorDirectionGet(index: uint32_t) -> int32_t;
    pub fn vexMotorActualVelocityGet(index: uint32_t) -> c_double;
    pub fn vexMotorModeSet(index: uint32_t, mode: V5MotorControlMode);
    pub fn vexMotorModeGet(index: uint32_t) -> V5MotorControlMode;
    pub fn vexMotorPwmSet(index: uint32_t, value: int32_t);
    pub fn vexMotorPwmGet(index: uint32_t) -> int32_t;
    pub fn vexMotorCurrentLimitSet(index: uint32_t, value: int32_t);
    pub fn vexMotorCurrentLimitGet(index: uint32_t) -> int32_t;
    pub fn vexMotorVoltageLimitSet(index: uint32_t, value: int32_t);
    pub fn vexMotorVoltageLimitGet(index: uint32_t) -> int32_t;
    pub fn vexMotorPositionPidSet(index: uint32_t, pid: *mut V5_DeviceMotorPid);
    pub fn vexMotorVelocityPidSet(index: uint32_t, pid: *mut V5_DeviceMotorPid);
    pub fn vexMotorCurrentGet(index: uint32_t) -> int32_t;
    pub fn vexMotorVoltageGet(index: uint32_t) -> int32_t;
    pub fn vexMotorPowerGet(index: uint32_t) -> c_double;
    pub fn vexMotorTorqueGet(index: uint32_t) -> c_double;
    pub fn vexMotorEfficiencyGet(index: uint32_t) -> c_double;
    pub fn vexMotorTemperatureGet(index: uint32_t) -> c_double;
    pub fn vexMotorOverTempFlagGet(index: uint32_t) -> bool;
    pub fn vexMotorCurrentLimitFlagGet(index: uint32_t) -> bool;
    pub fn vexMotorFaultsGet(index: uint32_t) -> uint32_t;
    pub fn vexMotorZeroVelocityFlagGet(index: uint32_t) -> bool;
    pub fn vexMotorZeroPositionFlagGet(index: uint32_t) -> bool;
    pub fn vexMotorFlagsGet(index: uint32_t) -> uint32_t;
    pub fn vexMotorReverseFlagSet(index: uint32_t, value: bool);
    pub fn vexMotorReverseFlagGet(index: uint32_t) -> bool;
    pub fn vexMotorEncoderUnitsSet(index: uint32_t, units: V5MotorEncoderUnits);
    pub fn vexMotorEncoderUnitsGet(index: uint32_t) -> V5MotorEncoderUnits;
    pub fn vexMotorBrakeModeSet(index: uint32_t, mode: V5MotorBrakeMode);
    pub fn vexMotorBrakeModeGet(index: uint32_t) -> V5MotorBrakeMode;
    pub fn vexMotorPositionSet(index: uint32_t, position: c_double);
    pub fn vexMotorPositionGet(index: uint32_t) -> c_double;
    pub fn vexMotorPositionRawGet(index: uint32_t, timestamp: *mut uint32_t) -> int32_t;
    pub fn vexMotorPositionReset(index: uint32_t);
    pub fn vexMotorTargetGet(index: uint32_t) -> c_double;
    pub fn vexMotorServoTargetSet(index: uint32_t, position: c_double);
    pub fn vexMotorAbsoluteTargetSet(index: uint32_t, position: c_double, velocity: int32_t);
    pub fn vexMotorRelativeTargetSet(index: uint32_t, position: c_double, velocity: int32_t);
    pub fn vexMotorGearingSet(index: uint32_t, value: V5MotorGearset);
    pub fn vexMotorGearingGet(index: uint32_t) -> V5MotorGearset;

    pub fn vexVisionModeSet(index: uint32_t, mode: V5VisionMode);
    pub fn vexVisionModeGet(index: uint32_t) -> V5VisionMode;
    pub fn vexVisionObjectCountGet(index: uint32_t) -> int32_t;
    pub fn vexVisionObjectGet(
        index: uint32_t,
        indexObj: uint32_t,
        pObject: *mut V5_DeviceVisionObject,
    ) -> int32_t;
    pub fn vexVisionSignatureSet(index: uint32_t, pSignature: *mut V5_DeviceVisionSignature);
    pub fn vexVisionSignatureGet(
        index: uint32_t,
        id: uint32_t,
        pSignature: *mut V5_DeviceVisionSignature,
    ) -> bool;
    pub fn vexVisionBrightnessSet(index: uint32_t, percent: uint8_t);
    pub fn vexVisionBrightnessGet(index: uint32_t) -> uint8_t;
    pub fn vexVisionWhiteBalanceModeSet(index: uint32_t, mode: V5VisionWBMode);
    pub fn vexVisionWhiteBalanceModeGet(index: uint32_t) -> V5VisionWBMode;
    pub fn vexVisionWhiteBalanceSet(index: uint32_t, color: V5_DeviceVisionRgb);
    pub fn vexVisionWhiteBalanceGet(index: uint32_t) -> V5_DeviceVisionRgb;
    pub fn vexVisionLedModeSet(index: uint32_t, mode: V5VisionLedMode);
    pub fn vexVisionLedModeGet(index: uint32_t) -> V5VisionLedMode;
    pub fn vexVisionLedBrigntnessSet(index: uint32_t, percent: uint8_t);
    pub fn vexVisionLedBrigntnessGet(index: uint32_t) -> uint8_t;
    pub fn vexVisionLedColorSet(index: uint32_t, color: V5_DeviceVisionRgb);
    pub fn vexVisionLedColorGet(index: uint32_t) -> V5_DeviceVisionRgb;
    pub fn vexVisionWifiModeSet(index: uint32_t, mode: V5VisionWifiMode);
    pub fn vexVisionWifiModeGet(index: uint32_t) -> V5VisionWifiMode;

    pub fn vexImuReset(index: uint32_t);
    pub fn vexImuHeadingGet(index: uint32_t) -> c_double;
    pub fn vexImuDegreesGet(index: uint32_t) -> c_double;
    pub fn vexImuQuaternionGet(index: uint32_t, data: *mut V5_DeviceImuQuaternion);
    pub fn vexImuAttitudeGet(index: uint32_t, data: *mut V5_DeviceImuAttitude);
    pub fn vexImuRawGyroGet(index: uint32_t, data: *mut V5_DeviceImuRaw);
    pub fn vexImuRawAccelGet(index: uint32_t, data: *mut V5_DeviceImuRaw);
    pub fn vexImuStatusGet(index: uint32_t) -> uint32_t;
    pub fn vexImuModeSet(index: uint32_t, mode: uint32_t);
    pub fn vexImuModeGet(index: uint32_t) -> uint32_t;

    pub fn vexRangeValueGet(index: uint32_t) -> int32_t;

    pub fn vexAbsEncReset(index: uint32_t);
    pub fn vexAbsEncPositionSet(index: uint32_t, position: int32_t);
    pub fn vexAbsEncPositionGet(index: uint32_t) -> int32_t;
    pub fn vexAbsEncVelocityGet(index: uint32_t) -> int32_t;
    pub fn vexAbsEncAngleGet(index: uint32_t) -> int32_t;
    pub fn vexAbsEncReverseFlagSet(index: uint32_t, value: bool);
    pub fn vexAbsEncReverseFlagGet(index: uint32_t) -> bool;
    pub fn vexAbsEncStatusGet(index: uint32_t) -> uint32_t;

    pub fn vexGenericSerialEnable(index: uint32_t, options: int32_t);
    pub fn vexGenericSerialBaudrate(index: uint32_t, baudrate: int32_t);
    pub fn vexGenericSerialWriteChar(index: uint32_t, c: uint8_t) -> int32_t;
    pub fn vexGenericSerialWriteFree(index: uint32_t) -> int32_t;
    pub fn vexGenericSerialTransmit(
        index: uint32_t,
        buffer: *mut uint8_t,
        length: int32_t,
    ) -> int32_t;
    pub fn vexGenericSerialReadChar(index: uint32_t) -> int32_t;
    pub fn vexGenericSerialPeekChar(index: uint32_t) -> int32_t;
    pub fn vexGenericSerialReceiveAvail(index: uint32_t) -> int32_t;
    pub fn vexGenericSerialReceive(
        index: uint32_t,
        buffer: *mut uint8_t,
        length: int32_t,
    ) -> int32_t;
    pub fn vexGenericSerialFlush(index: uint32_t);
}
