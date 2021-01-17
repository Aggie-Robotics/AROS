use crate::stream::{SendStream, ReceiveStream, MessageStreamCreator};
use std::sync::mpsc::{Sender, SendError, Receiver, TryRecvError, channel};
use std::sync::Mutex;
use crate::error::{Error, CustomError};
use alloc::format;
use core::ops::Deref;

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
