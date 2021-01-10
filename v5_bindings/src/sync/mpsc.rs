use alloc::boxed::Box;
use alloc::sync::{Arc, Weak};
use core::marker::PhantomData;
use core::mem::{MaybeUninit, size_of, forget};
use core::time::Duration;

use cty::c_void;

use crate::raw::pros::apix::*;
use crate::sync::option_to_timeout;
use crate::sync::queue::Queue;

pub fn channel<T>(length: u32) -> (Sender<T>, Receiver<T>){
    let queue = Arc::new(Queue::new(length));
    (Sender::new(Arc::downgrade(&queue)), Receiver::new(queue))
}

#[derive(Clone, Debug)]
pub struct Sender<T> where T: 'static + Send{
    queue: Weak<Queue<T>>,
}
impl<T> Sender<T> where T: 'static + Send{
    fn new(queue: Weak<Queue<T>>) -> Self{
        Self{ queue }
    }

    pub fn try_send(&self, t: T) -> Result<(), SendError<T>>{
        self.send_timeout(t, Some(Duration::new(0, 0)))
    }
    /**
     *  Sends a T, blocking until sent
     */
    pub fn send(&self, t: T) -> Result<(), SendError<T>>{
        self.send_timeout(t, None)
    }
    pub fn send_timeout(&self, t: T, timeout: Option<Duration>) -> Result<(), SendError<T>>{
        match self.queue.upgrade(){
            Some(arc) => {
                match arc.append(t, timeout){
                    Ok(_) => Ok(()),
                    Err(val) => Err(SendError::QueueTimeout(val)),
                }
            },
            None => Err(SendError::NoReceiver(t)),
        }
    }

    pub fn clear(&self) -> Result<(), SendError<()>>{
        self.upgrade()?.clear();
        Ok(())
    }

    pub fn len(&self) -> Result<u32, SendError<()>>{
        Ok(self.upgrade()?.len())
    }
    pub fn max_len(&self) -> Result<u32, SendError<()>>{
        Ok(self.upgrade()?.max_len())
    }

    fn upgrade(&self) -> Result<Arc<Queue<T>>, SendError<()>>{
        match self.queue.upgrade() {
            None => Err(SendError::NoReceiver(())),
            Some(arc) => Ok(arc),
        }
    }
}
unsafe impl<T> Send for Sender<T> where T: 'static + Send{}
unsafe impl<T> Sync for Sender<T>{}
pub enum SendError<T>{
    NoReceiver(T),
    MutexError(T),
    QueueTimeout(T),
}

pub struct Receiver<T> where T: 'static + Send{
    queue: Arc<Queue<T>>,
}
impl<T> Receiver<T> where T: 'static + Send{
    fn new(queue: Arc<Queue<T>>) -> Self{
        Self{ queue }
    }

    pub fn try_recv(&self) -> Option<T>{
        self.recv_timeout(Some(Duration::new(0, 0)))
    }
    pub fn recv(&self) -> Option<T>{
        self.recv_timeout(None)
    }
    pub fn recv_timeout(&self, timeout: Option<Duration>) -> Option<T>{
        self.queue.receive(timeout)
    }

    pub fn len(&self) -> u32{
        self.queue.len()
    }
    pub fn max_len(&self) -> u32{
        self.queue.max_len()
    }

    pub fn clear(&self){
        self.queue.clear()
    }
}
impl<T> Receiver<T> where T: 'static + Send + Copy{
    pub fn try_peek(&self) -> Option<T>{
        self.peek_timeout(Some(Duration::new(0, 0)))
    }
    pub fn peek(&self) -> Option<T>{
        self.peek_timeout(None)
    }
    pub fn peek_timeout(&self, timeout: Option<Duration>) -> Option<T>{
        self.queue.peek(timeout)
    }
}
