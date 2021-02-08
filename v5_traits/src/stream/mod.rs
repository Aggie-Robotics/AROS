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

    fn send(&self, val: Self::SData);
    fn send_slice(&self, slice: &[Self::SData]) where Self::SData: Copy{
        for val in slice{
            self.send(*val)
        }
    }
    fn send_vec(&self, data: Vec<Self::SData>){
        for val in data{
            self.send(val)
        }
    }
}
impl<S> SendStream for Arc<S> where S: SendStream{
    type SData = S::SData;

    fn send(&self, val: Self::SData) {
        self.deref().send(val)
    }

    fn send_slice(&self, slice: &[Self::SData]) where Self::SData: Copy {
        self.deref().send_slice(slice)
    }

    fn send_vec(&self, data: Vec<Self::SData>) {
        self.deref().send_vec(data)
    }
}
pub trait SendTimeoutStream: SendStream{
    fn send_timeout(&self, val: Self::SData, timeout: Duration, uf: &impl UniversalFunctions) -> Option<Self::SData>;
    fn send_slice_timeout(&self, slice: &[Self::SData], timeout: Duration, uf: &impl UniversalFunctions) -> usize where Self::SData: Copy{
        let end_time = uf.system_time() + timeout;
        let mut sent = 0;
        while end_time > uf.system_time() && sent < slice.len(){
            if let Some(_) = self.send_timeout(slice[sent], end_time - uf.system_time(), uf){
                return sent;
            }
            sent += 1;
        }
        sent
    }
    fn send_vec_timeout(&self, data: Vec<Self::SData>, timeout: Duration, uf: &impl UniversalFunctions) -> Option<Vec<Self::SData>>{
        let end_time = uf.system_time() + timeout;
        let mut sent = 0;
        let length = data.len();
        let mut val_iter = data.into_iter();
        while end_time > uf.system_time() && sent < length{
            let val = val_iter.next().unwrap();
            let receive = self.send_timeout(val, end_time - uf.system_time(), uf);
            if let Some(receive) = receive{
                return Some(once(receive).chain(val_iter).collect());
            }
            sent += 1;
        }
        if sent == length{
            None
        }
        else{
            Some(val_iter.collect())
        }
    }
}
impl<S> SendTimeoutStream for Arc<S> where S: SendTimeoutStream{
    fn send_timeout(&self, val: Self::SData, timeout: Duration, uf: &impl UniversalFunctions) -> Option<Self::SData> {
        self.deref().send_timeout(val, timeout, uf)
    }

    fn send_slice_timeout(&self, slice: &[Self::SData], timeout: Duration, uf: &impl UniversalFunctions) -> usize where Self::SData: Copy {
        self.deref().send_slice_timeout(slice, timeout, uf)
    }

    fn send_vec_timeout(&self, data: Vec<Self::SData>, timeout: Duration, uf: &impl UniversalFunctions) -> Option<Vec<Self::SData>>{
        self.deref().send_vec_timeout(data, timeout, uf)
    }
}

pub trait ReceiveStream{
    type RData: 'static + Send;

    fn try_receive(&self) -> Option<Self::RData>;
    fn receive(&self) -> Self::RData;
    fn receive_slice(&self, buffer: &mut [Self::RData]) -> usize{
        for val in &mut *buffer {
            *val = self.receive();
        }
        buffer.len()
    }
    fn receive_all(&self, buffer: &mut [Self::RData]) {
        let mut received = 0;
        while received < buffer.len(){
            received += self.receive_slice(&mut buffer[received..]);
        }
    }
    fn receive_vec(&self, limit: usize) -> Vec<Self::RData>{
        self.receive_whole_vec(limit)
    }
    fn receive_whole_vec(&self, limit: usize) -> Vec<Self::RData>{
        let mut out = Vec::with_capacity(limit);
        while out.len() < limit{
            out.push(self.receive())
        }
        out
    }
}
impl<S> ReceiveStream for Arc<S> where S: ReceiveStream{
    type RData = S::RData;

    fn try_receive(&self) -> Option<Self::RData> {
        self.deref().try_receive()
    }

    fn receive(&self) -> Self::RData {
        self.deref().receive()
    }

    fn receive_slice(&self, buffer: &mut [Self::RData]) -> usize {
        self.deref().receive_slice(buffer)
    }

    fn receive_all(&self, buffer: &mut [Self::RData]) {
        self.deref().receive_all(buffer)
    }

    fn receive_vec(&self, limit: usize) -> Vec<Self::RData> {
        self.deref().receive_vec(limit)
    }

    fn receive_whole_vec(&self, limit: usize) -> Vec<Self::RData> {
        self.deref().receive_whole_vec(limit)
    }
}
pub trait ReceiveTimoutStream: ReceiveStream{
    fn receive_timeout(&self, timeout: Duration, uf: &impl UniversalFunctions) -> Option<Self::RData>;
    fn receive_slice_timeout(&self, buffer: &mut [Self::RData], timeout: Duration, uf: &impl UniversalFunctions) -> usize{
        let end_time = uf.system_time() + timeout;
        let mut received = 0;
        while end_time > uf.system_time() && received < buffer.len(){
            match self.receive_timeout(end_time - uf.system_time(), uf){
                None => break,
                Some(value) => {
                    buffer[received] = value;
                    received += 1;
                }
            }
        }
        received
    }
    fn receive_vec_timeout(&self, limit: usize, timeout: Duration, uf: &impl UniversalFunctions) -> Vec<Self::RData>{
        let end_time = uf.system_time() + timeout;
        let mut out = Vec::with_capacity(limit);
        while uf.system_time() < end_time && out.len() < limit{
            match self.receive_timeout(end_time - uf.system_time(), uf){
                None => break,
                Some(value) => out.push(value),
            }
        }
        out
    }
}
impl<S> ReceiveTimoutStream for Arc<S> where S: ReceiveTimoutStream{
    fn receive_timeout(&self, timeout: Duration, uf: &impl UniversalFunctions) -> Option<Self::RData> {
        self.deref().receive_timeout(timeout, uf)
    }

    fn receive_slice_timeout(&self, buffer: &mut [Self::RData], timeout: Duration, uf: &impl UniversalFunctions) -> usize {
        self.deref().receive_slice_timeout(buffer, timeout, uf)
    }

    fn receive_vec_timeout(&self, limit: usize, timeout: Duration, uf: &impl UniversalFunctions) -> Vec<Self::RData> {
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
