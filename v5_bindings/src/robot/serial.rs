use crate::robot::port::Port;
use crate::raw::vex_os::api::*;
use v5_traits::stream::{SendStream, ReceiveStream};
use alloc::vec::Vec;

pub struct Serial{
    port: Port,
}
impl Serial{
    pub fn new(port: Port, baud_rate: BaudRate) -> Self{
        unsafe {
            vexDeviceGenericSerialEnable(port.device(), 0);
            vexDeviceGenericSerialBaudrate(port.device(), baud_rate as i32);
            vexDeviceGenericSerialFlush(port.device());
        }
        Self{ port }
    }
}
impl SendStream<u8> for Serial{
    type Error = ();

    fn send(&self, val: u8) -> Result<(), Self::Error> {
        self.send_slice(&[val])
    }

    fn send_slice(&self, slice: &[u8]) -> Result<(), Self::Error> {
        let mut sent = 0;
        while sent < slice.len() {
            sent += unsafe { vexDeviceGenericSerialTransmit(self.port.device(), slice[sent] as *const u8 as *mut u8, slice[sent..].len() as i32) } as usize;
        }
        Ok(())
    }

    fn send_vec(&self, data: Vec<u8>) -> Result<(), Self::Error> {
        self.send_slice(&data)
    }
}
impl ReceiveStream<u8> for Serial{
    type Error = ();

    fn try_receive(&self) -> Result<Option<u8>, Self::Error> {
        if unsafe{ vexDeviceGenericSerialReceiveAvail(self.port.device()) } > 0{
            Ok(Some(self.receive()?))
        }
        else{
            Ok(None)
        }
    }

    fn receive(&self) -> Result<u8, Self::Error> {
        let mut buffer = [0];
        self.receive_slice(&mut buffer)?;
        Ok(buffer[0])
    }

    fn receive_slice(&self, buffer: &mut [u8]) -> Result<usize, Self::Error> {
        Ok(unsafe { vexDeviceGenericSerialReceive(self.port.device(), buffer[0] as *mut u8, buffer.len() as i32) } as usize)
    }

    fn receive_vec(&self, limit: usize) -> Result<Vec<u8>, Self::Error> {
        let mut buffer = vec![0; limit];
        let received = self.receive_slice(&mut buffer)?;
        buffer.resize(received, 0);
        Ok(buffer)
    }

    fn receive_whole_vec(&self, limit: usize) -> Result<Vec<u8>, Self::Error> {
        let mut out = vec![0; limit];
        self.receive_all(&mut out)?;
        Ok(out)
    }
}

pub enum BaudRate{
    B9600 = 9600,
    B19200 = 19200,
    B38400 = 38400,
    B57600 = 57600,
    B115200 = 115200,
}
