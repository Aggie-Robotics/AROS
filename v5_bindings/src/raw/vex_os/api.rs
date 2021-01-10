#![allow(non_snake_case)]

use cty::*;

use crate::raw::vex_os::api_types::*;

extern "C"{
    pub fn vexDebug(fmt: *const c_char, ...) -> int32_t;
    pub fn vex_printf(fmt: *const c_char, ...) -> int32_t;
    pub fn vex_sprintf(out: *mut c_char, format: *const c_char, ...) -> int32_t;
    pub fn vex_snprintf(out: *mut c_char, max_len: uint32_t, format: *const c_char, ...) -> int32_t;
    /* VaList Unstable
    pub fn vex_vsprintf(out: *mut c_char, format: *const c_char, args: va_list) -> int32_t;
    pub fn vex_vsnprintf(out: *mut c_char, max_len: uint32_t, format: *const c_char, args: va_list) -> int32_t;
     */

    //System
    pub fn vexSystemTimeGet() -> uint32_t;
    pub fn vexGettime(pTime: *mut time);
    pub fn vexGetdate(pDate: *mut date);
    pub fn vexSystemMemoryDump();
    pub fn vexSystemDigitalIO(pin: uint32_t, value: uint32_t);
    pub fn vexSystemStartupOptions() -> uint32_t;
    pub fn vexSystemExitRequest();
    pub fn vexSystemHighResTimeGet() -> uint64_t;
    pub fn vexSystemPowerupTimeGet() -> uint64_t;
    pub fn vexSystemLinkAddrGet() -> uint32_t;
    pub fn vexSystemUsbStatus() -> uint32_t;

    //Generic Device
    pub fn vexDevicesGetNumber() -> uint32_t;
    pub fn vexDevicesGetNumberByType(device_type: V5_DeviceType) -> uint32_t;
    pub fn vexDevicesGet() -> V5_DeviceT;
    pub fn vexDeviceGetByIndex(index: uint32_t) -> V5_DeviceT;
    pub fn vexDeviceGetStatus(buffer: *mut V5_DeviceType) -> int32_t;

    //Controller
    pub fn vexControllerGet(id: V5_ControllerId, index: V5_ControllerIndex) -> int32_t;
    pub fn vexControllerConnectionStatusGet(id: V5_ControllerId) -> V5_ControllerStatus;
    pub fn vexControllerTextSet(id: V5_ControllerId, line: uint32_t, col: uint32_t, str: *const c_char) -> bool;

    //LED Sensor
    pub fn vexDeviceLedSet(device: V5_DeviceT, value: V5_DeviceLedColor);
    pub fn vexDeviceLedRgbSet(device: V5_DeviceT, color: uint32_t);
    pub fn vexDeviceLedGet(device: V5_DeviceT) -> V5_DeviceLedColor;
    pub fn vexDeviceLedRgbGet(device: V5_DeviceT) -> uint32_t;

    //ADI Sensor
    pub fn vexDeviceAdiPortConfigSet(device: V5_DeviceT, port: uint32_t, config: V5_AdiPortConfiguration);
    pub fn vexDeviceAdiPortConfigGet(device: V5_DeviceT, port: uint32_t) -> V5_AdiPortConfiguration;
    pub fn vexDeviceAdiValueSet(device: V5_DeviceT, port: uint32_t, value: int32_t);
    pub fn vexDeviceAdiValueGet(device: V5_DeviceT, port: uint32_t) -> int32_t;

    //Bumper Switch
    pub fn vexDeviceBumperGet(device: V5_DeviceT) -> V5_DeviceBumperState;

    //Gyro
    #[deprecated]
    pub fn vexDeviceGyroReset(device: V5_DeviceT);
    #[deprecated]
    pub fn vexDeviceGyroHeadingGet(device: V5_DeviceT) -> c_double;
    #[deprecated]
    pub fn vexDeviceGyroDegreesGet(device: V5_DeviceT) -> c_double;

    //Sonar
    #[deprecated]
    pub fn vexDeviceSonarValueGet(device: V5_DeviceT) -> int32_t;

    //Generic Sensor
    pub fn vexDeviceGenericValueGet(device: V5_DeviceT) -> int32_t;

    //Motor
    pub fn vexDeviceMotorVelocitySet(device: V5_DeviceT, velocity: int32_t);
    pub fn vexDeviceMotorVelocityUpdate(device: V5_DeviceT, velocity: int32_t);
    pub fn vexDeviceMotorVoltageSet(device: V5_DeviceT, value: int32_t);
    pub fn vexDeviceMotorVelocityGet(device: V5_DeviceT) -> int32_t;
    pub fn vexDeviceMotorActualVelocityGet(device: V5_DeviceT) -> c_double;
    pub fn vexDeviceMotorDirectionGet(device: V5_DeviceT) -> int32_t;
    pub fn vexDeviceMotorModeSet(device: V5_DeviceT, mode: V5MotorControlMode);
    pub fn vexDeviceMotorModeGet(device: V5_DeviceT) -> V5MotorControlMode;
    pub fn vexDeviceMotorPwmSet(device: V5_DeviceT, value: int32_t);
    pub fn vexDeviceMotorPwmGet(device: V5_DeviceT) -> int32_t;
    pub fn vexDeviceMotorCurrentLimitSet(device: V5_DeviceT, value: int32_t);
    pub fn vexDeviceMotorCurrentLimitGet(device: V5_DeviceT) -> int32_t;
    pub fn vexDeviceMotorVoltageLimitSet(device: V5_DeviceT, value: int32_t);
    pub fn vexDeviceMotorVoltageLimitGet(device: V5_DeviceT) -> int32_t;
    pub fn vexDeviceMotorPositionPidSet(device: V5_DeviceT, pid: *mut V5_DeviceMotorPid);
    pub fn vexDeviceMotorVelocityPidSet(device: V5_DeviceT, pid: *mut V5_DeviceMotorPid);
    pub fn vexDeviceMotorCurrentGet(device: V5_DeviceT) -> int32_t;
    pub fn vexDeviceMotorVoltageGet(device: V5_DeviceT) -> int32_t;
    pub fn vexDeviceMotorPowerGet(device: V5_DeviceT) -> c_double;
    pub fn vexDeviceMotorTorqueGet(device: V5_DeviceT) -> c_double;
    pub fn vexDeviceMotorEfficiencyGet(device: V5_DeviceT) -> c_double;
    pub fn vexDeviceMotorTemperatureGet(device: V5_DeviceT) -> c_double;
    pub fn vexDeviceMotorOverTempFlagGet(device: V5_DeviceT) -> bool;
    pub fn vexDeviceMotorCurrentLimitFlagGet(device: V5_DeviceT) -> bool;
    pub fn vexDeviceMotorFaultsGet(device: V5_DeviceT) -> uint32_t;
    pub fn vexDeviceMotorZeroVelocityFlagGet(device: V5_DeviceT) -> bool;
    pub fn vexDeviceMotorZeroPositionFlagGet(device: V5_DeviceT) -> bool;
    pub fn vexDeviceMotorFlagsGet(device: V5_DeviceT) -> uint32_t;
    pub fn vexDeviceMotorReverseFlagSet(device: V5_DeviceT, value: bool);
    pub fn vexDeviceMotorReverseFlagGet(device: V5_DeviceT) -> bool;
    pub fn vexDeviceMotorEncoderUnitsSet(device: V5_DeviceT, units: V5MotorEncoderUnits);
    pub fn vexDeviceMotorEncoderUnitsGet(device: V5_DeviceT) -> V5MotorEncoderUnits;
    pub fn vexDeviceMotorBrakeModeSet(device: V5_DeviceT, mode: V5MotorBrakeMode);
    pub fn vexDeviceMotorBrakeModeGet(device: V5_DeviceT) -> V5MotorBrakeMode;
    pub fn vexDeviceMotorPositionSet(device: V5_DeviceT, position: c_double);
    pub fn vexDeviceMotorPositionGet(device: V5_DeviceT) -> c_double;
    pub fn vexDeviceMotorPositionRawGet(device: V5_DeviceT, timestamp: *mut uint32_t) -> int32_t;
    pub fn vexDeviceMotorPositionReset(device: V5_DeviceT);
    pub fn vexDeviceMotorTargetGet(device: V5_DeviceT) -> c_double;
    pub fn vexDeviceMotorServoTargetSet(device: V5_DeviceT, position: c_double);
    pub fn vexDeviceMotorAbsoluteTargetSet(device: V5_DeviceT, position: c_double, velocity: int32_t);
    pub fn vexDeviceMotorRelativeTargetSet(device: V5_DeviceT, position: c_double, velocity: int32_t);
    pub fn vexDeviceMotorGearingSet(device: V5_DeviceT, value: V5MotorGearset);
    pub fn vexDeviceMotorGearingGet(device: V5_DeviceT) -> V5MotorGearset;

    //Vision sensor
    pub fn vexDeviceVisionModeSet(device: V5_DeviceT, mode: V5VisionMode);
    pub fn vexDeviceVisionModeGet(device: V5_DeviceT) -> V5VisionMode;
    pub fn vexDeviceVisionObjectCountGet(device: V5_DeviceT) -> int32_t;
    pub fn vexDeviceVisionObjectGet(device: V5_DeviceT, indexObj: uint32_t, pObject: *mut V5_DeviceVisionObject) -> int32_t;
    pub fn vexDeviceVisionSignatureSet(device: V5_DeviceT, pSignature: *mut V5_DeviceVisionSignature);
    pub fn vexDeviceVisionSignatureGet(device: V5_DeviceT, id: uint32_t, pSignature: *mut V5_DeviceVisionSignature) -> bool;
    pub fn vexDeviceVisionBrightnessSet(device: V5_DeviceT, percent: uint8_t);
    pub fn vexDeviceVisionBrightnessGet(device: V5_DeviceT) -> uint8_t;
    pub fn vexDeviceVisionWhiteBalanceModeSet(device: V5_DeviceT, mode: V5VisionWBMode);
    pub fn vexDeviceVisionWhiteBalanceModeGet(device: V5_DeviceT) -> V5VisionWBMode;
    pub fn vexDeviceVisionWhiteBalanceSet(device: V5_DeviceT, color: V5_DeviceVisionRgb);
    pub fn vexDeviceVisionWhiteBalanceGet(device: V5_DeviceT) -> V5_DeviceVisionRgb;
    pub fn vexDeviceVisionLedModeSet(device: V5_DeviceT, mode: V5VisionLedMode);
    pub fn vexDeviceVisionLedModeGet(device: V5_DeviceT) -> V5VisionLedMode;
    pub fn vexDeviceVisionLedBrigntnessSet(device: V5_DeviceT, percent: uint8_t);
    pub fn vexDeviceVisionLedBrigntnessGet(device: V5_DeviceT) -> uint8_t;
    pub fn vexDeviceVisionLedColorSet(device: V5_DeviceT, color: V5_DeviceVisionRgb);
    pub fn vexDeviceVisionLedColorGet(device: V5_DeviceT) -> V5_DeviceVisionRgb;
    pub fn vexDeviceVisionWifiModeSet(device: V5_DeviceT, mode: V5VisionWifiMode);
    pub fn vexDeviceVisionWifiModeGet(device: V5_DeviceT) -> V5VisionWifiMode;

    //IMU
    pub fn vexDeviceImuReset(device: V5_DeviceT);
    pub fn vexDeviceImuHeadingGet(device: V5_DeviceT) -> c_double;
    pub fn vexDeviceImuDegreesGet(device: V5_DeviceT) -> c_double;
    pub fn vexDeviceImuQuaternionGet(device: V5_DeviceT, data: *mut V5_DeviceImuQuaternion);
    pub fn vexDeviceImuAttitudeGet(device: V5_DeviceT, data: *mut V5_DeviceImuAttitude);
    pub fn vexDeviceImuRawGyroGet(device: V5_DeviceT, data: *mut V5_DeviceImuRaw);
    pub fn vexDeviceImuRawAccelGet(device: V5_DeviceT, data: *mut V5_DeviceImuRaw);
    pub fn vexDeviceImuStatusGet(device: V5_DeviceT) -> uint32_t;
    pub fn vexDeviceImuModeSet(device: V5_DeviceT, mode: uint32_t);
    pub fn vexDeviceImuModeGet(device: V5_DeviceT) -> uint32_t;

    //Rangefinder/Lidar
    pub fn vexDeviceRangeValueGet(device: V5_DeviceT) -> int32_t;

    //Absolute Encoder
    pub fn vexDeviceAbsEncReset(device: V5_DeviceT);
    pub fn vexDeviceAbsEncPositionSet(device: V5_DeviceT, position: int32_t);
    pub fn vexDeviceAbsEncPositionGet(device: V5_DeviceT) -> int32_t;
    pub fn vexDeviceAbsEncVelocityGet(device: V5_DeviceT) -> int32_t;
    pub fn vexDeviceAbsEncAngleGet(device: V5_DeviceT) -> int32_t;
    pub fn vexDeviceAbsEncReverseFlagSet(device: V5_DeviceT, value: bool);
    pub fn vexDeviceAbsEncReverseFlagGet(device: V5_DeviceT) -> bool;
    pub fn vexDeviceAbsEncStatusGet(device: V5_DeviceT) -> uint32_t;

    //Generic Serial Port Comms to any Device
    pub fn vexDeviceGenericSerialEnable(device: V5_DeviceT, options: int32_t);
    pub fn vexDeviceGenericSerialBaudrate(device: V5_DeviceT, baudrate: int32_t);
    pub fn vexDeviceGenericSerialWriteChar(device: V5_DeviceT, c: uint8_t) -> int32_t;
    pub fn vexDeviceGenericSerialWriteFree(device: V5_DeviceT) -> int32_t;
    pub fn vexDeviceGenericSerialTransmit(device: V5_DeviceT, buffer: *mut uint8_t, length: int32_t) -> int32_t;
    pub fn vexDeviceGenericSerialReadChar(device: V5_DeviceT) -> int32_t;
    pub fn vexDeviceGenericSerialPeekChar(device: V5_DeviceT) -> int32_t;
    pub fn vexDeviceGenericSerialReceiveAvail(device: V5_DeviceT) -> int32_t;
    pub fn vexDeviceGenericSerialReceive(device: V5_DeviceT, buffer: *mut uint8_t, length: int32_t) -> int32_t;
    pub fn vexDeviceGenericSerialFlush(device: V5_DeviceT);

    //Display/Graphics
    pub fn vexDisplayForegroundColor(col: uint32_t);
    pub fn vexDisplayBackgroundColor(col: uint32_t);
    pub fn vexDisplayForegroundColorGet() -> uint32_t;
    pub fn vexDisplayBackgroundColorGet() -> uint32_t;
    pub fn vexDisplayErase();
    pub fn vexDisplayScroll(nStartLine: int32_t, nLines: int32_t);
    pub fn vexDisplayScrollRect(x1: int32_t, y1: int32_t, x2: int32_t, y2: int32_t, nLines: int32_t);
    pub fn vexDisplayCopyRect(x1: int32_t, y1: int32_t, x2: int32_t, y2: int32_t, pSrc: *mut uint32_t, srcStride: int32_t);
    pub fn vexDisplayPixelSet(x: uint32_t, y: uint32_t);
    pub fn vexDisplayPixelClear(x: uint32_t, y: uint32_t);
    pub fn vexDisplayLineDraw(x1: int32_t, y1: int32_t, x2: int32_t, y2: int32_t);
    pub fn vexDisplayLineClear(x1: int32_t, y1: int32_t, x2: int32_t, y2: int32_t);
    pub fn vexDisplayRectDraw(x1: int32_t, y1: int32_t, x2: int32_t, y2: int32_t);
    pub fn vexDisplayRectClear(x1: int32_t, y1: int32_t, x2: int32_t, y2: int32_t);
    pub fn vexDisplayRectFill(x1: int32_t, y1: int32_t, x2: int32_t, y2: int32_t);
    pub fn vexDisplayCircleDraw(xc: int32_t, yc: int32_t, radius: int32_t);
    pub fn vexDisplayCircleClear(xc: int32_t, yc: int32_t, radius: int32_t);
    pub fn vexDisplayCircleFill(xc: int32_t, yc: int32_t, radius: int32_t);

    pub fn vexDisplayPrintf(xpos: int32_t, ypos: int32_t, bOpaque: int32_t, format: *const c_char, ...);
    pub fn vexDisplayString(nLineNumber: int32_t, format: *const c_char, ...);
    pub fn vexDisplayStringAt(xpos: int32_t, ypos: int32_t, format: *const c_char, ...);
    pub fn vexDisplayBigString(nLineNumber: int32_t, format: *const c_char, ...);
    pub fn vexDisplayBigStringAt(xpos: int32_t, ypos: int32_t, format: *const c_char, ...);
    pub fn vexDisplaySmallStringAt(xpos: int32_t, ypos: int32_t, format: *const c_char, ...);
    pub fn vexDisplayCenteredString(nLineNumber: int32_t, format: *const c_char, ...);
    pub fn vexDisplayBigCenteredString(nLineNumber: int32_t, format: *const c_char, ...);

    //Wrapper Functions

    /*vargs unstable
    pub fn vexDisplayVPrintf(xpos: int32_t, ypos: int32_t, bOpaque: int32_t, format: *const c_char, args: va_list);
    pub fn vexDisplayVString(nLineNumber: int32_t, format: *const c_char, args: va_list);
    pub fn vexDisplayVStringAt(xpos: int32_t, ypos: int32_t, format: *const c_char, args: va_list);
    pub fn vexDisplayVBigString(nLineNumber: int32_t, format: *const c_char, args: va_list);
    pub fn vexDisplayVBigStringAt(xpos: int32_t, ypos: int32_t, format: *const c_char, args: va_list);
    pub fn vexDisplayVSmallStringAt(xpos: int32_t, ypos: int32_t, format: *const c_char, args: va_list);
    pub fn vexDisplayVCenteredString(nLineNumber: int32_t, format: *const c_char, args: va_list);
    pub fn vexDisplayVBigCenteredString(nLineNumber: int32_t, format: *const c_char, args: va_list);
    */

    pub fn vexDisplayTextSize(n: uint32_t, d: uint32_t);
    pub fn vexDisplayFontNamedSet(pFontName: *const c_char);
    pub fn vexDisplayStringWidthGet(pString: *const c_char) -> int32_t;
    pub fn vexDisplayStringHeightGet(pString: *const c_char) -> int32_t;
    pub fn vexDisplayRender(bVsyncWait: bool, bRunScheduler: bool) -> bool;
    pub fn vexDisplayDoubleBufferDisable();
    pub fn vexDisplayClipRegionSet(x1: int32_t, y1: int32_t, x2: int32_t, y2: int32_t);
    pub fn vexDisplayClipRegionClear();
    pub fn vexImageBmpRead(ibuf: *const uint8_t, oBuf: *mut v5_image, maxw: uint32_t, maxh: uint32_t) -> uint32_t;
    pub fn vexImagePngRead(ibuf: *const uint8_t, oBuf: *mut v5_image, maxw: uint32_t, maxh: uint32_t, ibuflen: uint32_t) -> uint32_t;
    //James Function
    pub fn vexScratchMemoryPtr(ptr: *mut *mut c_void) -> int32_t;

    //SD Card
    pub fn vexFileMountSD() -> FRESULT;
    pub fn vexFileDirectoryGet(path: *const c_char, buffer: *mut c_char, len: uint32_t) -> FRESULT;
    pub fn vexFileOpen(filename: *const c_char, mode: *const c_char) -> *mut FIL;
    pub fn vexFileOpenWrite(filename: *const c_char) -> *mut FIL;
    pub fn vexFileOpenCreate(filename: *const c_char) -> *mut FIL;
    pub fn vexFileClose(fdp: *mut FIL);
    pub fn vexFileRead(buf: *mut c_char, size: uint32_t, nItems: uint32_t, fdp: *mut FIL) -> int32_t;
    pub fn vexFileWrite(buf: *mut c_char, size: uint32_t, nItems: uint32_t, fdp: *mut FIL) -> int32_t;
    pub fn vexFileSize(fdp: *mut FIL) -> int32_t;
    pub fn vexFileSeek(fdp: *mut FIL, offset: uint32_t, whence: int32_t) -> FRESULT;
    pub fn vexFileDriveStatus(drive: uint32_t) -> bool;
    pub fn vexFileTell(fdp: *mut FIL) -> int32_t;

    //CDC
    pub fn vexSerialWriteChar(channel: uint32_t, c: uint8_t) -> int32_t;
    pub fn vexSerialWriteBuffer(channel: uint32_t, data: *mut uint8_t, data_len: uint32_t) -> int32_t;
    pub fn vexSerialReadChar(channel: uint32_t) -> int32_t;
    pub fn vexSerialPeekChar(channel: uint32_t) -> int32_t;
    pub fn vexSerialWriteFree(channel: uint32_t) -> int32_t;

    //RTOS Hooks
    pub fn vexSystemTimerStop();
    pub fn vexSystemTimerClearInterrupt();
    pub fn vexSystemTimerReinitForRtos(priority: uint32_t, handler: extern "C" fn(data: *mut c_void)) -> int32_t;
    pub fn vexSystemApplicationIRQHandler(ulICCIAR: uint32_t);
    pub fn vexSystemWatchdogReinitRtos() -> int32_t;
    pub fn vexSystemWatchdogGet() -> uint32_t;

    //Interrupt Hooks
    pub fn vexSystemBoot();
    pub fn vexSystemUndefinedException();
    pub fn vexSystemFIQInterrupt();
    pub fn vexSystemIQRQnterrupt();
    pub fn vexSystemSWInterrupt();
    pub fn vexSystemDataAbortInterrupt();
    pub fn vexSystemPrefetchAbortInterrupt();

    //Touch
    pub fn vexTouchUserCallbackSet(callback: extern "C" fn(V5_TouchEvent, int32_t, int32_t));
    pub fn vexTouchDataGet(status: *mut V5_TouchStatus) -> bool;

    //System Utility
    pub fn vexSystemVersion() -> uint32_t;
    pub fn vexStdlibVersion() -> uint32_t;
    pub fn vexSdkVersion() -> uint32_t;
    pub fn vexStdlibVersionLinked() -> uint32_t;
    pub fn vexStdlibVersionVerify() -> bool;

    //Competition Status
    pub fn vexCompetitionStatus() -> uint32_t;
    pub fn vexCompetitionControl(data: uint32_t);

    //Battery
    pub fn vexBatteryVoltageGet() -> int32_t;
    pub fn vexBatteryCurrentGet() -> int32_t;
    pub fn vexBatteryTemperatureGet() -> c_double;
    pub fn vexBatteryCapacityGet() -> c_double;
}
