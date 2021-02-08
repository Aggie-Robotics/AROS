use crate::stream::{SendStream, ReceiveStream, MessageStreamCreator, SendTimeoutStream, ReceiveTimoutStream};
use std::sync::mpsc::{Sender, Receiver, channel, RecvTimeoutError};
use core::ops::{Deref, DerefMut};
use core::time::Duration;
use crate::UniversalFunctions;
use parking_lot::Mutex;

impl<T> SendStream for Mutex<Sender<T>> where T: 'static + Send{
    type SData = T;

    fn send(&self, val: T) {
        let guard = self.lock();
        match Sender::send(guard.deref(), val){
            Ok(_) => {}
            Err(error) => eprintln!("Error sending!: {}", error),
        }
    }
}
impl<T> SendTimeoutStream for Mutex<Sender<T>> where T: 'static + Send{
    fn send_timeout(&self, val: T, timeout: Duration, uf: &impl UniversalFunctions) -> Option<T> {
        let end_time = uf.system_time() + timeout;
        let mut guard = None;
        while let None = guard {
            guard = self.try_lock();
            if let None = guard{
                uf.delay(Duration::from_millis(1));
                if uf.system_time() > end_time{
                    return Some(val);
                }
            }
        }
        if let Err(error) = Sender::send(guard.unwrap().deref(), val){
            Some(error.0)
        }
        else {
            None
        }
    }
}

impl<T> ReceiveStream for Mutex<Receiver<T>> where T: 'static + Send{
    type RData = T;

    fn try_receive(&self) -> Option<T> {
        let guard = self.lock();
        match guard.try_recv(){
            Ok(val) => Some(val),
            Err(_) => None,
        }
    }

    fn receive(&self) -> T {
        self.lock().recv().expect("Receive error")
    }
}
impl<T> ReceiveTimoutStream for Mutex<Receiver<T>> where T: 'static + Send{
    fn receive_timeout(&self, timeout: Duration, uf: &impl UniversalFunctions) -> Option<T> {
        let end_time = uf.system_time() + timeout;
        let mut guard = None;
        while let None = guard {
            guard = self.try_lock();
            if let None = guard{
                uf.delay(Duration::from_millis(1));
                if uf.system_time() > end_time{
                    return None;
                }
            }
        }
        match Receiver::recv_timeout(guard.unwrap().deref(), end_time - uf.system_time()){
            Ok(val) => Some(val),
            Err(error) => match error {
                RecvTimeoutError::Timeout => None,
                RecvTimeoutError::Disconnected => panic!("Receive disconnected"),
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

impl<T> crate::mutex::Mutex for Mutex<T> where T: Send{
    type Inner = T;

    fn new(val: Self::Inner) -> Self where Self: Sized {
        Self::new(val)
    }

    fn lock<R>(&self, f: impl FnOnce(&mut Self::Inner) -> R) -> R {
        f(self.lock().deref_mut())
    }

    fn is_locked(&self) -> bool {
        self.is_locked()
    }

    fn try_lock<R, F>(&self, f: F) -> Result<R, F> where F: FnOnce(&mut Self::Inner) -> R {
        match self.try_lock(){
            None => Err(f),
            Some(mut guard) => Ok(f(guard.deref_mut()))
        }
    }

    fn into_inner(self) -> Self::Inner {
        self.into_inner()
    }
}
