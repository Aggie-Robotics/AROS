use alloc::vec::Vec;
use crate::error::Error;
use core::fmt::Debug;

pub trait SendStream<T>: Debug{
    type Error: Error;

    /// Returns None on success or Some on could not send without blocking
    fn try_send(&self, val: T) -> Result<Result<(), T>, Self::Error>;
    fn send(&self, val: T) -> Result<(), Self::Error>;
    fn send_slice(&self, slice: &[T]) -> Result<usize, Self::Error> where T: Copy{
        for val in slice {
            self.send(*val)?;
        }
        Ok(slice.len())
    }
    fn send_all(&self, slice: &[T]) -> Result<(), Self::Error> where T: Copy{
        let mut sent = 0;
        while sent < slice.len(){
            sent += self.send_slice(&slice[sent..])?;
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

pub trait ReceiveStream<T>: Debug{
    type Error: Error;

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
        while out.len() < limit{
            out.push(self.receive()?)
        }
        Ok(out)
    }
}

pub trait DuplexStream<T>: SendStream<T> + ReceiveStream<T>{}
#[derive(Clone, Debug)]
pub enum DuplexError<S, R> where S: Error, R: Error{
    SendError(S),
    ReceiveError(R),
}
impl<S, R> Error for DuplexError<S, R> where S: Error, R: Error{}

pub trait MessageStreamCreator<T>: Debug where T: 'static + Send{
    type Sender: SendStream<T> + Send + Sync;
    type Receiver: ReceiveStream<T> + Send + Sync;

    fn create_stream(&self) -> (Self::Sender, Self::Receiver);
    fn create_bidirectional_stream(&self) -> ((Self::Sender, Self::Receiver), (Self::Sender, Self::Receiver)){
        let stream1 = self.create_stream();
        let stream2 = self.create_stream();
        (stream1, stream2)
    }
}
