use alloc::sync::Arc;
use alloc::vec::Vec;
use core::convert::TryFrom;

use cstr_core::CString;
use cty::c_void;

use crate::raw::{close, O_RDONLY, O_WRONLY, open, read, write};
use crate::raw::pros::apix::{serctl, SERCTL_ACTIVATE, SERCTL_DEACTIVATE};
use crate::sync::lock::Mutex;

pub struct SerialOut {
    stream_id: [u8; 4],
    file_descriptor: Mutex<i32>,
}
impl SerialOut {
    fn new(stream_id: [u8; 4]) -> Self{
        unsafe {serctl(SERCTL_ACTIVATE, *(stream_id.as_ptr() as *const u32) as *mut c_void)};
        let mut file_name: Vec<u8> = "/ser/".into();
        file_name.extend_from_slice(&stream_id);
        Self{
            stream_id,
            file_descriptor: Mutex::new(unsafe{open(CString::new(file_name).unwrap().as_ptr(), O_WRONLY)}),
        }
    }

    pub fn write(&self, data: &Vec<u8>) -> Result<(), SerialError>{
        let mut index = 0;
        let fd_guard = self.file_descriptor.lock();
        while index < data.len(){
            let write_return = unsafe{write(*fd_guard, data.as_ptr().add(index) as *const c_void, data.len() - index)};
            if write_return < 0{
                return Err(SerialError::ReturnError(write_return));
            }
            index += write_return as usize;
        }
        Ok(())
    }
}
impl Drop for SerialOut{
    fn drop(&mut self) {
        unsafe {
            serctl(SERCTL_DEACTIVATE, *(self.stream_id.as_ptr() as *const u32) as *mut c_void);
            close(*self.file_descriptor.get_mut());
        };
    }
}
pub enum SerialError {
    ReturnError(isize),
}

pub struct SerialOutRegistry{
    serials: Vec<Arc<SerialOut>>,
}
impl SerialOutRegistry{
    pub fn new() -> Self{
        Self{
            serials: vec![
                Arc::new(SerialOut::new(<[u8; 4]>::try_from("sout".as_bytes()).unwrap())),
                Arc::new(SerialOut::new(<[u8; 4]>::try_from("serr".as_bytes()).unwrap())),
            ],
        }
    }

    pub fn serial(&self, stream_id: [u8; 4]) -> Option<Arc<SerialOut>>{
        Some(self.serials.iter().find(|&x| x.stream_id == stream_id)?.clone())
    }

    pub fn create(&mut self, stream_id: [u8; 4]) -> Arc<SerialOut>{
        match self.serial(stream_id) {
            None => {
                let out = Arc::new(SerialOut::new(stream_id));
                self.serials.push(out.clone());
                out
            },
            Some(out) => out,
        }
    }
}

pub struct SerialIn{
    file_descriptor: i32,
}
impl SerialIn{
    pub fn new() -> Self{
        Self{
            file_descriptor: unsafe{open(CString::new("/ser/").unwrap().as_ptr(), O_RDONLY)},
        }
    }

    pub fn read(&mut self, buffer: &mut Vec<u8>, max_read: usize) -> Result<usize, SerialError>{
        let read_return = unsafe{read(self.file_descriptor, buffer.as_mut_ptr() as *mut c_void, max_read)};
        if read_return < 0{
            Err(SerialError::ReturnError(read_return))
        }
        else{
            Ok(read_return as usize)
        }
    }
}
impl Drop for SerialIn{
    fn drop(&mut self) {
        unsafe {close(self.file_descriptor)};
    }
}
