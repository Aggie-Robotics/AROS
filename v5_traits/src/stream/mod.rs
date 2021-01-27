#[cfg(feature = "std")]
pub mod std_impls;

use alloc::vec::Vec;
use crate::error::Error;
use core::fmt::Debug;
use core::time::Duration;
use alloc::sync::Arc;
use core::ops::Deref;
use crate::UniversalFunctions;
use core::iter::once;

pub trait SendStream{
    type SData: 'static + Send;
    type Error: Error + Debug;

    fn send(&self, val: Self::SData) -> Result<(), Self::Error>;
    fn send_slice(&self, slice: &[Self::SData]) -> Result<(), Self::Error> where Self::SData: Copy{
        for val in slice{
            self.send(*val)?
        }
        Ok(())
    }
    fn send_vec(&self, data: Vec<Self::SData>) -> Result<(), Self::Error>{
        for val in data{
            self.send(val)?;
        }
        Ok(())
    }
}
impl<S> SendStream for Arc<S> where S: SendStream{
    type SData = S::SData;
    type Error = S::Error;

    fn send(&self, val: Self::SData) -> Result<(), Self::Error> {
        self.deref().send(val)
    }

    fn send_slice(&self, slice: &[Self::SData]) -> Result<(), Self::Error> where Self::SData: Copy {
        self.deref().send_slice(slice)
    }

    fn send_vec(&self, data: Vec<Self::SData>) -> Result<(), Self::Error> {
        self.deref().send_vec(data)
    }
}
pub trait SendTimeoutStream: SendStream{
    fn send_timeout(&self, val: Self::SData, timeout: Duration, uf: &impl UniversalFunctions) -> Result<Option<Self::SData>, Self::Error>;
    fn send_slice_timeout(&self, slice: &[Self::SData], timeout: Duration, uf: &impl UniversalFunctions) -> Result<usize, Self::Error> where Self::SData: Copy{
        let end_time = uf.system_time() + timeout;
        let mut sent = 0;
        while end_time > uf.system_time() && sent < slice.len(){
            if let Some(_) = self.send_timeout(slice[sent], end_time - uf.system_time(), uf)?{
                return Ok(sent);
            }
            sent += 1;
        }
        Ok(sent)
    }
    ///Returns true if whole vec was sent, otherwise false
    fn send_vec_timeout(&self, data: Vec<Self::SData>, timeout: Duration, uf: &impl UniversalFunctions) -> Result<Option<Vec<Self::SData>>, Self::Error>{
        let end_time = uf.system_time() + timeout;
        let mut sent = 0;
        let length = data.len();
        let mut val_iter = data.into_iter();
        while end_time > uf.system_time() && sent < length{
            let val = val_iter.next().unwrap();
            match self.send_timeout(val, end_time - uf.system_time(), uf){
                Ok(receive) => if let Some(receive) = receive{
                    return Ok(Some(once(receive).chain(val_iter).collect()));
                },
                Err(error) => return Err(error),
            }
            sent += 1;
        }
        if sent == length{
            Ok(None)
        }
        else{
            Ok(Some(val_iter.collect()))
        }
    }
}
impl<S> SendTimeoutStream for Arc<S> where S: SendTimeoutStream{
    fn send_timeout(&self, val: Self::SData, timeout: Duration, uf: &impl UniversalFunctions) -> Result<Option<Self::SData>, Self::Error> {
        self.deref().send_timeout(val, timeout, uf)
    }

    fn send_slice_timeout(&self, slice: &[Self::SData], timeout: Duration, uf: &impl UniversalFunctions) -> Result<usize, Self::Error> where Self::SData: Copy {
        self.deref().send_slice_timeout(slice, timeout, uf)
    }

    fn send_vec_timeout(&self, data: Vec<Self::SData>, timeout: Duration, uf: &impl UniversalFunctions) -> Result<Option<Vec<Self::SData>>, Self::Error> {
        self.deref().send_vec_timeout(data, timeout, uf)
    }
}

pub trait ReceiveStream{
    type RData: 'static + Send;
    type Error: Error + Debug;

    fn try_receive(&self) -> Result<Option<Self::RData>, Self::Error>;
    fn receive(&self) -> Result<Self::RData, Self::Error>;
    fn receive_slice(&self, buffer: &mut [Self::RData]) -> Result<usize, Self::Error> {
        for val in &mut *buffer {
            *val = self.receive()?;
        }
        Ok(buffer.len())
    }
    fn receive_all(&self, buffer: &mut [Self::RData]) -> Result<(), Self::Error> {
        let mut received = 0;
        while received < buffer.len(){
            received += self.receive_slice(&mut buffer[received..])?;
        }
        Ok(())
    }
    fn receive_vec(&self, limit: usize) -> Result<Vec<Self::RData>, Self::Error>{
        self.receive_whole_vec(limit)
    }
    /// Appends to vec
    fn receive_whole_vec(&self, limit: usize) -> Result<Vec<Self::RData>, Self::Error>{
        let mut out = Vec::with_capacity(limit);
        while out.len() < limit{
            out.push(self.receive()?)
        }
        Ok(out)
    }
}
impl<S> ReceiveStream for Arc<S> where S: ReceiveStream{
    type RData = S::RData;
    type Error = S::Error;

    fn try_receive(&self) -> Result<Option<Self::RData>, Self::Error> {
        self.deref().try_receive()
    }

    fn receive(&self) -> Result<Self::RData, Self::Error> {
        self.deref().receive()
    }

    fn receive_slice(&self, buffer: &mut [Self::RData]) -> Result<usize, Self::Error> {
        self.deref().receive_slice(buffer)
    }

    fn receive_all(&self, buffer: &mut [Self::RData]) -> Result<(), Self::Error> {
        self.deref().receive_all(buffer)
    }

    fn receive_vec(&self, limit: usize) -> Result<Vec<Self::RData>, Self::Error> {
        self.deref().receive_vec(limit)
    }

    fn receive_whole_vec(&self, limit: usize) -> Result<Vec<Self::RData>, Self::Error> {
        self.deref().receive_whole_vec(limit)
    }
}
pub trait ReceiveTimoutStream: ReceiveStream{
    fn receive_timeout(&self, timeout: Duration, uf: &impl UniversalFunctions) -> Result<Option<Self::RData>, Self::Error>;
    fn receive_slice_timeout(&self, buffer: &mut [Self::RData], timeout: Duration, uf: &impl UniversalFunctions) -> Result<usize, Self::Error>{
        let end_time = uf.system_time() + timeout;
        let mut received = 0;
        while end_time > uf.system_time() && received < buffer.len(){
            match self.receive_timeout(end_time - uf.system_time(), uf)?{
                None => break,
                Some(value) => {
                    buffer[received] = value;
                    received += 1;
                }
            }
        }
        Ok(received)
    }
    /// Returns true if received, false if timed out
    fn receive_all_timeout(&self, buffer: &mut [Self::RData], timeout: Duration, uf: &impl UniversalFunctions) -> Result<bool, Self::Error>{
        let mut found = 0;
        let end_time = uf.system_time() + timeout;
        while uf.system_time() < end_time && found < buffer.len(){
            found += self.receive_slice_timeout(&mut buffer[found..], end_time - uf.system_time(), uf)?;
        }
        Ok(found == buffer.len())
    }
    fn receive_vec_timeout(&self, limit: usize, timeout: Duration, uf: &impl UniversalFunctions) -> Result<Vec<Self::RData>, Self::Error>{
        let end_time = uf.system_time() + timeout;
        let mut out = Vec::with_capacity(limit);
        while uf.system_time() < end_time && out.len() < limit{
            match self.receive_timeout(end_time - uf.system_time(), uf)?{
                None => break,
                Some(value) => out.push(value),
            }
        }
        Ok(out)
    }
}
impl<S> ReceiveTimoutStream for Arc<S> where S: ReceiveTimoutStream{
    fn receive_timeout(&self, timeout: Duration, uf: &impl UniversalFunctions) -> Result<Option<Self::RData>, Self::Error> {
        self.deref().receive_timeout(timeout, uf)
    }

    fn receive_slice_timeout(&self, buffer: &mut [Self::RData], timeout: Duration, uf: &impl UniversalFunctions) -> Result<usize, Self::Error> {
        self.deref().receive_slice_timeout(buffer, timeout, uf)
    }

    fn receive_all_timeout(&self, buffer: &mut [Self::RData], timeout: Duration, uf: &impl UniversalFunctions) -> Result<bool, Self::Error> {
        self.deref().receive_all_timeout(buffer, timeout, uf)
    }

    fn receive_vec_timeout(&self, limit: usize, timeout: Duration, uf: &impl UniversalFunctions) -> Result<Vec<Self::RData>, Self::Error> {
        self.deref().receive_vec_timeout(limit, timeout, uf)
    }
}

pub trait DuplexStream: SendStream + ReceiveStream<RData=<Self as SendStream>::SData>{}
impl<S> DuplexStream for Arc<S> where S: DuplexStream{}
pub trait DuplexTimeoutStream: DuplexStream + SendTimeoutStream + ReceiveTimoutStream{}
impl<S> DuplexTimeoutStream for Arc<S> where S: DuplexTimeoutStream{}
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

pub trait MessageStreamCreator<T>{
    type Sender: 'static + SendStream<SData=T> + Send + Sync;
    type Receiver: 'static + ReceiveStream<RData=T> + Send + Sync;

    fn create_stream(&self) -> (Self::Sender, Self::Receiver);
    fn create_bidirectional_stream(&self) -> ((Self::Sender, Self::Receiver), (Self::Sender, Self::Receiver)){
        let stream1 = self.create_stream();
        let stream2 = self.create_stream();
        (stream1, stream2)
    }
}
