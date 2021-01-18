use crate::stream::{SendStream, ReceiveStream, MessageStreamCreator, SendTimeoutStream, ReceiveTimoutStream};
use std::sync::mpsc::{Sender, SendError, Receiver, TryRecvError, channel, RecvTimeoutError};
use std::sync::Mutex;
use crate::error::{Error, CustomError};
use alloc::format;
use core::ops::Deref;
use core::time::Duration;
use crate::UniversalFunctions;

impl<T> Error for SendError<T> where T: 'static{
    fn is_recoverable(&self) -> bool {
        true
    }
}
impl<T> SendStream<T> for Mutex<Sender<T>> where T: 'static + Send{
    type Error = SendError<T>;

    fn send(&self, val: T) -> Result<(), Self::Error> {
        let guard = self.lock().expect("Poisoned Mutex!!");
        Sender::send(guard.deref(), val)
    }
}
impl<T> SendTimeoutStream<T> for Mutex<Sender<T>> where T: 'static + Send{
    fn send_timeout(&self, val: T, timeout: Duration, uf: &impl UniversalFunctions) -> Result<Option<T>, Self::Error> {
        let end_time = uf.system_time() + timeout;
        let mut guard = None;
        while let None = guard {
            guard = match self.try_lock() {
                Ok(guard) => Some(guard),
                Err(_) => None,
            };
            if let None = guard{
                uf.delay(Duration::from_millis(1));
                if uf.system_time() > end_time{
                    return Ok(Some(val));
                }
            }
        }
        Sender::send(guard.unwrap().deref(), val)?;
        Ok(None)
    }
}

impl<T> ReceiveStream<T> for Mutex<Receiver<T>> where T: 'static + Send{
    type Error = CustomError;

    fn try_receive(&self) -> Result<Option<T>, Self::Error> {
        let guard = self.lock().expect("Poisoned Mutex");
        match guard.try_recv(){
            Ok(val) => Ok(Some(val)),
            Err(error) => match error {
                TryRecvError::Empty => Ok(None),
                TryRecvError::Disconnected => Err(CustomError::new(true, "Disconnected")),
            },
        }
    }

    fn receive(&self) -> Result<T, Self::Error> {
        let guard = self.lock().expect("Poisoned Mutex!");
        match guard.recv(){
            Ok(val) => Ok(val),
            Err(error) => Err(CustomError::new(true, format!("RecvError: {}", error))),
        }
    }
}
impl<T> ReceiveTimoutStream<T> for Mutex<Receiver<T>> where T: 'static + Send{
    fn receive_timeout(&self, timeout: Duration, uf: &impl UniversalFunctions) -> Result<Option<T>, Self::Error> {
        let end_time = uf.system_time() + timeout;
        let mut guard = None;
        while let None = guard {
            guard = match self.try_lock() {
                Ok(guard) => Some(guard),
                Err(_) => None,
            };
            if let None = guard{
                uf.delay(Duration::from_millis(1));
                if uf.system_time() > end_time{
                    return Ok(None);
                }
            }
        }
        match Receiver::recv_timeout(guard.unwrap().deref(), end_time - uf.system_time()){
            Ok(val) => Ok(Some(val)),
            Err(error) => match error {
                RecvTimeoutError::Timeout => Ok(None),
                RecvTimeoutError::Disconnected => Err(CustomError::new(true, "Receiver disconnected")),
            }
        }
    }
}

pub fn new_mpsc_channel<T>() -> (Mutex<Sender<T>>, Mutex<Receiver<T>>) where T: 'static + Send{
    let out = channel();
    (Mutex::new(out.0), Mutex::new(out.1))
}
pub struct MPSCMessageCreator();
impl<T> MessageStreamCreator<T> for MPSCMessageCreator where T: 'static + Send{
    type Sender = Mutex<Sender<T>>;
    type Receiver = Mutex<Receiver<T>>;

    fn create_stream(&self) -> (Self::Sender, Self::Receiver) {
        new_mpsc_channel()
    }
}
