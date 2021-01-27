use crate::robot::port::Port;
use crate::raw::vex_os::api::*;
use v5_traits::stream::*;
use alloc::vec::Vec;
use crate::sync::queue::Queue;
use crate::Task;
use alloc::sync::{Arc, Weak};
use v5_traits::UniversalFunctions;
use core::time::Duration;
use crate::sync::lock::Mutex;
use v5_traits::error::CustomError;
use core::ops::DerefMut;

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
impl ReceiveStream for Serial{
    type RData = u8;
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

const ASYNC_SERIAL_MAX_BUFFER_SIZE: usize = 1024;
pub struct AsyncSerial<UF> where UF: UniversalFunctions{
    uf: UF,
    serial: Serial,
    inbound: Queue<Vec<u8>>,
    outbound: Queue<VecOrVal<u8>>,
    past_buffer: Mutex<(usize, Option<Vec<u8>>)>,
}
impl<UF> AsyncSerial<UF> where UF: UniversalFunctions{
    pub fn new(uf: UF, serial: Serial) -> Arc<Self>{
        let out = Arc::new(Self{
            uf,
            serial,
            inbound: Queue::new(128),
            outbound: Queue::new(128),
            past_buffer: Mutex::new((0, None)),
        });

        Task::new(None, None, format!("Async Serial {} inbound", out.serial.port.number()), |async_serial: Weak<AsyncSerial<UF>>|{
            while let Some(async_serial) = async_serial.upgrade(){
                match async_serial.serial.receive_vec(ASYNC_SERIAL_MAX_BUFFER_SIZE){
                    Ok(value) => {
                        if !value.is_empty(){
                            if let Err(_) = async_serial.inbound.append(value, None){
                                async_serial.uf.log_error(||format!("Error appending to inbound queue from port {}", async_serial.serial.port.number()))
                            }
                        }
                    }
                    Err(_) => async_serial.uf.log_error(||format!("Error receiving from serial on port {}", async_serial.serial.port.number())),
                }
            }
        }, Arc::downgrade(&out));
        Task::new(None, None, format!("Async Serial {} outbound", out.serial.port.number()), |async_serial: Weak<AsyncSerial<UF>>|{
            while let Some(async_serial) = async_serial.upgrade(){
                match async_serial.outbound.receive() {
                    Ok(value) => match value {
                        VecOrVal::Vec(vec) => if let Err(_) = async_serial.serial.send_vec(vec){
                            async_serial.uf.log_error(||format!("Error sending vec on port {}", async_serial.serial.port.number()));
                        },
                        VecOrVal::Val(val) => if let Err(_) = async_serial.serial.send(val){
                            async_serial.uf.log_error(||format!("Error sending value on port {}", async_serial.serial.port.number()));
                        },
                    }
                    Err(_) => async_serial.uf.log_error(||format!("Error receiving from queue on port {}", async_serial.serial.port.number())),
                }
            }
        }, Arc::downgrade(&out));

        out
    }
}
impl<UF> SendStream for AsyncSerial<UF> where UF: UniversalFunctions{
    type SData = u8;
    type Error = ();

    fn send(&self, val: u8) -> Result<(), Self::Error> {
        self.outbound.send(VecOrVal::Val(val))
    }

    fn send_slice(&self, slice: &[u8]) -> Result<(), Self::Error> {
        self.outbound.send(VecOrVal::Vec(slice.iter().cloned().collect()))
    }

    fn send_vec(&self, data: Vec<u8>) -> Result<(), Self::Error> {
        self.outbound.send(VecOrVal::Vec(data))
    }
}
impl<UF> SendTimeoutStream for AsyncSerial<UF> where UF: UniversalFunctions{
    fn send_timeout(&self, val: u8, timeout: Duration, _uf: &impl UniversalFunctions) -> Result<Option<u8>, Self::Error> {
        match self.outbound.append(VecOrVal::Val(val), Some(timeout)){
            Ok(_) => Ok(None),
            Err(_) => Ok(Some(val)),
        }
    }

    fn send_slice_timeout(&self, slice: &[u8], timeout: Duration, _uf: &impl UniversalFunctions) -> Result<usize, Self::Error>{
        match self.outbound.append(VecOrVal::Vec(slice.iter().cloned().collect()), Some(timeout)){
            Ok(_) => Ok(slice.len()),
            Err(_) => Ok(0),
        }
    }

    fn send_vec_timeout(&self, data: Vec<u8>, timeout: Duration, _uf: &impl UniversalFunctions) -> Result<Option<Vec<u8>>, Self::Error> {
        match self.outbound.append(VecOrVal::Vec(data), Some(timeout)) {
            Ok(_) => Ok(None),
            Err(error) => match error{
                VecOrVal::Vec(vec) => Ok(Some(vec)),
                VecOrVal::Val(_) => unreachable!(),
            }
        }
    }
}
impl<UF> ReceiveStream for AsyncSerial<UF> where UF: UniversalFunctions{
    type RData = u8;
    type Error = CustomError;

    fn try_receive(&self) -> Result<Option<u8>, Self::Error> {
        let mut guard = match self.past_buffer.try_lock(){
            None => return Ok(None),
            Some(guard) => guard,
        };

        if let (index, Some(vec)) = guard.deref_mut() {
            let out = vec[*index];
            *index += 1;
            if *index == vec.len(){
                guard.1 = None;
            }
            Ok(Some(out))
        }
        else{
            let received = match self.inbound.try_receive()?{
                None => return Ok(None),
                Some(received) => received,
            };
            if received.is_empty(){
                return Ok(None);
            }
            if received.len() == 1{
                return Ok(Some(received[0]));
            }
            let out = received[0];
            guard.0 = 1;
            guard.1 = Some(received);
            Ok(Some(out))
        }
    }

    fn receive(&self) -> Result<u8, Self::Error> {
        let mut guard = self.past_buffer.lock();
        if let (index, Some(vec)) = guard.deref_mut() {
            let out = vec[*index];
            *index += 1;
            if *index == vec.len(){
                guard.1 = None;
            }
            Ok(out)
        }
        else{
            let mut received = self.inbound.receive()?;
            while received.is_empty(){
                received = self.inbound.receive()?;
            }
            if received.len() == 1{
                return Ok(received[0]);
            }
            let out = received[0];
            guard.0 = 1;
            guard.1 = Some(received);
            Ok(out)
        }
    }

    fn receive_slice(&self, buffer: &mut [u8]) -> Result<usize, Self::Error> {
        let mut guard = self.past_buffer.lock();
        let vec = if let Some(vec) = &guard.1{
            vec
        }
        else{
            let mut vec = self.inbound.receive()?;
            while vec.is_empty(){
                vec = self.inbound.receive()?;
            }
            guard.0 = 0;
            guard.1 = Some(vec);
            guard.1.as_deref().unwrap()
        };
        let len_remaining = vec.len() - guard.0;
        if len_remaining <= buffer.len() {
            buffer[0..len_remaining].copy_from_slice(&vec[guard.0..]);
            guard.1 = None;
            Ok(len_remaining)
        }
        else{
            buffer.copy_from_slice(&vec[guard.0..guard.0 + buffer.len()]);
            guard.0 += buffer.len();
            Ok(buffer.len())
        }
    }

    fn receive_all(&self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        let mut guard = self.past_buffer.lock();
        let mut received = 0;
        while received < buffer.len() {
            let sub_buffer = &mut buffer[received..];
            let vec = if let Some(vec) = &guard.1 {
                vec
            } else {
                let mut vec = self.inbound.receive()?;
                while vec.is_empty() {
                    vec = self.inbound.receive()?;
                }
                guard.0 = 0;
                guard.1 = Some(vec);
                guard.1.as_deref().unwrap()
            };
            let len_remaining = vec.len() - guard.0;
            if len_remaining <= sub_buffer.len() {
                sub_buffer[0..len_remaining].copy_from_slice(&vec[guard.0..]);
                guard.1 = None;
                received += len_remaining;
            } else {
                sub_buffer.copy_from_slice(&vec[guard.0..guard.0 + sub_buffer.len()]);
                guard.0 += sub_buffer.len();
                received += sub_buffer.len()
            }
        }
        if received > buffer.len(){
            self.uf.log_debug(||format!("This probably shouldn't be happening... received = {}, buffer.len() = {}", received, buffer.len()));
        }
        Ok(())
    }

    fn receive_vec(&self, limit: usize) -> Result<Vec<u8>, Self::Error> {
        let mut out = vec![0; limit];
        let returned = self.receive_slice(&mut out)?;
        out.resize(returned, 0);
        Ok(out)
    }

    fn receive_whole_vec(&self, limit: usize) -> Result<Vec<u8>, Self::Error> {
        let mut out = vec![0; limit];
        self.receive_all(&mut out)?;
        Ok(out)
    }
}
impl<UF> ReceiveTimoutStream for AsyncSerial<UF> where UF: UniversalFunctions{
    fn receive_timeout(&self, timeout: Duration, uf: &impl UniversalFunctions) -> Result<Option<u8>, Self::Error> {
        unimplemented!()
    }

    fn receive_slice_timeout(&self, buffer: &mut [u8], timeout: Duration, uf: &impl UniversalFunctions) -> Result<usize, Self::Error> {
        unimplemented!()
    }

    fn receive_all_timeout(&self, buffer: &mut [u8], timeout: Duration, uf: &impl UniversalFunctions) -> Result<bool, Self::Error> {
        unimplemented!()
    }

    fn receive_vec_timeout(&self, limit: usize, timeout: Duration, uf: &impl UniversalFunctions) -> Result<Vec<u8>, Self::Error> {
        unimplemented!()
    }
}

#[derive(Clone, Debug)]
enum VecOrVal<T>{
    Vec(Vec<T>),
    Val(T),
}

pub enum BaudRate{
    B9600 = 9600,
    B19200 = 19200,
    B38400 = 38400,
    B57600 = 57600,
    B115200 = 115200,
}
