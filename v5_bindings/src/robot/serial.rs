use crate::robot::port::Port;
use crate::raw::vex_os::api::*;
use v5_traits::stream::*;
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
impl SendStream for Serial{
    type SData = u8;

    fn send(&self, val: u8) {
        self.send_slice(&[val])
    }

    fn send_slice(&self, slice: &[u8]){
        let mut sent = 0;
        while sent < slice.len() {
            sent += unsafe { vexDeviceGenericSerialTransmit(self.port.device(), slice[sent] as *const u8 as *mut u8, slice[sent..].len() as i32) } as usize;
        }
    }

    fn send_vec(&self, data: Vec<u8>) {
        self.send_slice(&data)
    }
}
impl ReceiveStream for Serial{
    type RData = u8;

    fn try_receive(&self) -> Option<u8> {
        if unsafe{ vexDeviceGenericSerialReceiveAvail(self.port.device()) } > 0{
            Some(self.receive())
        }
        else{
            None
        }
    }

    fn receive(&self) -> u8 {
        let mut buffer = [0];
        self.receive_slice(&mut buffer);
        buffer[0]
    }

    fn receive_slice(&self, buffer: &mut [u8]) -> usize {
        unsafe { vexDeviceGenericSerialReceive(self.port.device(), buffer[0] as *mut u8, buffer.len() as i32) as usize }
    }

    fn receive_vec(&self, limit: usize) -> Vec<u8> {
        let mut buffer = vec![0; limit];
        let received = self.receive_slice(&mut buffer);
        buffer.resize(received, 0);
        buffer
    }

    fn receive_whole_vec(&self, limit: usize) -> Vec<u8> {
        let mut out = vec![0; limit];
        self.receive_all(&mut out);
        out
    }
}

pub struct DuplexSerial{
    read_serial: Serial,
    write_serial: Serial,
}
impl DuplexSerial{
    pub fn new(read_serial: Serial, write_serial: Serial) -> Self{
        Self{ read_serial, write_serial }
    }
}
impl SendStream for DuplexSerial {
    type SData = u8;

    fn send(&self, val: Self::SData) {
        self.write_serial.send(val)
    }

    fn send_slice(&self, slice: &[Self::SData]) where Self::SData: Copy {
        self.write_serial.send_slice(slice)
    }

    fn send_vec(&self, data: Vec<Self::SData>) {
        self.write_serial.send_vec(data)
    }
}
impl ReceiveStream for DuplexSerial {
    type RData = u8;

    fn try_receive(&self) -> Option<Self::RData> {
        self.read_serial.try_receive()
    }

    fn receive(&self) -> Self::RData{
        self.read_serial.receive()
    }

    fn receive_slice(&self, buffer: &mut [Self::RData]) -> usize {
        self.read_serial.receive_slice(buffer)
    }

    fn receive_all(&self, buffer: &mut [Self::RData])  {
        self.read_serial.receive_all(buffer)
    }

    fn receive_vec(&self, limit: usize) -> Vec<Self::RData> {
        self.read_serial.receive_vec(limit)
    }

    fn receive_whole_vec(&self, limit: usize) -> Vec<Self::RData> {
        self.read_serial.receive_whole_vec(limit)
    }
}

pub enum BaudRate{
    B9600 = 9600,
    B19200 = 19200,
    B38400 = 38400,
    B57600 = 57600,
    B115200 = 115200,
}
