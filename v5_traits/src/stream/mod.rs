#[cfg(feature = "std")]
pub mod std_impls;

use alloc::vec::Vec;
use crate::error::Error;
use core::fmt::Debug;
use core::time::Duration;
use alloc::sync::Arc;
use core::ops::Deref;

pub trait SendStream<T> where T: 'static + Send{
    type Error: Error + Debug;

    fn send(&self, val: T) -> Result<(), Self::Error>;
    fn send_slice(&self, slice: &[T]) -> Result<(), Self::Error> where T: Copy{
        for val in slice{
            self.send(*val)?
        }
        Ok(())
    }
    fn send_vec(&self, data: Vec<T>) -> Result<(), Self::Error>{
        for val in data{
            self.send(val)?;
        }
        Ok(())
    }
}
impl<T, S> SendStream<T> for Arc<S> where T: 'static + Send, S: SendStream<T>{
    type Error = S::Error;

    fn send(&self, val: T) -> Result<(), Self::Error> {
        self.deref().send(val)
    }

    fn send_slice(&self, slice: &[T]) -> Result<(), Self::Error> where T: Copy {
        self.deref().send_slice(slice)
    }

    fn send_vec(&self, data: Vec<T>) -> Result<(), Self::Error> {
        self.deref().send_vec(data)
    }
}
pub trait SendTimeoutStream<T>: SendStream<T> where T: 'static + Send{
    fn send_timeout(&self, val: T, timeout: Duration) -> Result<T, Self::Error>;
    fn send_slice_timeout(&self, slice: &[T], timeout: Duration) -> Result<usize, Self::Error> where T: Copy;
    fn send_vec_timeout(&self, data: Vec<T>, timeout: Duration) -> Result<Option<Vec<T>>, Self::Error>;
}
impl<T, S> SendTimeoutStream<T> for Arc<S> where T: 'static + Send, S: SendTimeoutStream<T>{
    fn send_timeout(&self, val: T, timeout: Duration) -> Result<T, Self::Error> {
        self.deref().send_timeout(val, timeout)
    }

    fn send_slice_timeout(&self, slice: &[T], timeout: Duration) -> Result<usize, Self::Error> where T: Copy {
        self.deref().send_slice_timeout(slice, timeout)
    }

    fn send_vec_timeout(&self, data: Vec<T>, timeout: Duration) -> Result<Option<Vec<T>>, Self::Error> {
        self.deref().send_vec_timeout(data, timeout)
    }
}

pub trait ReceiveStream<T> where T: Send{
    type Error: Error + Debug;

    fn try_receive(&self) -> Result<Option<T>, Self::Error>;
    fn receive(&self) -> Result<T, Self::Error>;
    fn receive_slice(&self, buffer: &mut [T]) -> Result<usize, Self::Error> {
        for val in &mut *buffer {
            *val = self.receive()?;
        }
        Ok(buffer.len())
    }
    fn receive_all(&self, buffer: &mut [T]) -> Result<(), Self::Error> {
        let mut received = 0;
        while received < buffer.len(){
            received += self.receive_slice(&mut buffer[received..])?;
        }
        Ok(())
    }
    fn receive_vec(&self, limit: usize) -> Result<Vec<T>, Self::Error>{
        let mut out = Vec::with_capacity(limit);
        self.receive_whole_vec(&mut out, limit)?;
        Ok(out)
    }
    /// Appends to vec
    fn receive_whole_vec(&self, vec: &mut Vec<T>, limit: usize) -> Result<(), Self::Error>{
        let start_size = vec.len();
        while vec.len() < start_size + limit{
            vec.push(self.receive()?)
        }
        Ok(())
    }
}
impl<T, S> ReceiveStream<T> for Arc<S> where T: 'static + Send, S: ReceiveStream<T>{
    type Error = S::Error;

    fn try_receive(&self) -> Result<Option<T>, Self::Error> {
        self.deref().try_receive()
    }

    fn receive(&self) -> Result<T, Self::Error> {
        self.deref().receive()
    }

    fn receive_slice(&self, buffer: &mut [T]) -> Result<usize, Self::Error> {
        self.deref().receive_slice(buffer)
    }

    fn receive_all(&self, buffer: &mut [T]) -> Result<(), Self::Error> {
        self.deref().receive_all(buffer)
    }

    fn receive_vec(&self, limit: usize) -> Result<Vec<T>, Self::Error> {
        self.deref().receive_vec(limit)
    }

    fn receive_whole_vec(&self, vec: &mut Vec<T>, limit: usize) -> Result<(), Self::Error> {
        self.deref().receive_whole_vec(vec, limit)
    }
}
pub trait ReceiveTimoutStream<T>: ReceiveStream<T> where T: Send{
    fn receive_timeout(&self, timeout: Duration) -> Result<Option<T>, Self::Error>;
    fn receive_slice_timeout(&self, buffer: &mut [T], timeout: Duration) -> Result<usize, Self::Error>;
    fn receive_vec_timeout(&self, limit: usize, timeout: Duration) -> Result<Vec<T>, Self::Error>;
}
impl<T, S> ReceiveTimoutStream<T> for Arc<S> where T: 'static + Send, S: ReceiveTimoutStream<T>{
    fn receive_timeout(&self, timeout: Duration) -> Result<Option<T>, Self::Error> {
        self.deref().receive_timeout(timeout)
    }

    fn receive_slice_timeout(&self, buffer: &mut [T], timeout: Duration) -> Result<usize, Self::Error> {
        self.deref().receive_slice_timeout(buffer, timeout)
    }

    fn receive_vec_timeout(&self, limit: usize, timeout: Duration) -> Result<Vec<T>, Self::Error> {
        self.deref().receive_vec_timeout(limit, timeout)
    }
}

pub trait DuplexStream<T>: SendStream<T> + ReceiveStream<T> where T: 'static + Send{}
impl<T, S> DuplexStream<T> for Arc<S> where T: 'static + Send, S: DuplexStream<T>{}
pub trait DuplexTimeoutStream<T>: SendTimeoutStream<T> + ReceiveTimoutStream<T> where T: 'static + Send{}
impl<T, S> DuplexTimeoutStream<T> for Arc<S> where T: 'static + Send, S: DuplexTimeoutStream<T>{}
#[derive(Clone, Debug)]
pub enum DuplexError<S, R> where S: Error, R: Error{
    SendError(S),
    ReceiveError(R),
}
impl<S, R> Error for DuplexError<S, R> where S: Error, R: Error{
    fn is_recoverable(&self) -> bool {
        match self {
            Self::SendError(error) => error.is_recoverable(),
            Self::ReceiveError(error)=> error.is_recoverable(),
        }
    }
}

pub trait MessageStreamCreator<T> where T: 'static + Send{
    type Sender: SendStream<T> + Send + Sync;
    type Receiver: ReceiveStream<T> + Send + Sync;

    fn create_stream(&self) -> (Self::Sender, Self::Receiver);
    fn create_bidirectional_stream(&self) -> ((Self::Sender, Self::Receiver), (Self::Sender, Self::Receiver)){
        let stream1 = self.create_stream();
        let stream2 = self.create_stream();
        (stream1, stream2)
    }
}
