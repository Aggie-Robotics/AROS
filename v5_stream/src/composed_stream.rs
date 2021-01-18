use core::fmt::Debug;
use core::marker::PhantomData;

use v5_traits::stream::*;
use alloc::vec::Vec;
use core::time::Duration;
use v5_traits::UniversalFunctions;

#[derive(Debug)]
pub struct ComposedStream<T, S, R> where T: 'static + Send, S: SendStream<T>, R: ReceiveStream<T>{
    pub send_stream: S,
    pub receive_stream: R,
    phantom_t: PhantomData<T>,
}
impl<T, S, R> ComposedStream<T, S, R> where T: 'static + Send, S: SendStream<T>, R: ReceiveStream<T>{
    pub fn new(send_stream: S, receive_stream: R) -> Self{
        Self{ send_stream, receive_stream, phantom_t: Default::default() }
    }
}
impl<T, S, R> SendStream<T> for ComposedStream<T, S, R> where T: 'static + Send, S: SendStream<T>, R: ReceiveStream<T>{
    type Error = S::Error;

    fn send(&self, val: T) -> Result<(), Self::Error> {
        self.send_stream.send(val)
    }

    fn send_slice(&self, slice: &[T]) -> Result<(), Self::Error> where T: Copy {
        self.send_stream.send_slice(slice)
    }

    fn send_vec(&self, data: Vec<T>) -> Result<(), Self::Error> {
        self.send_stream.send_vec(data)
    }
}
impl<T, S, R> SendTimeoutStream<T> for ComposedStream<T, S, R> where T: 'static + Send, S: SendTimeoutStream<T>, R: ReceiveStream<T>{
    fn send_timeout(&self, val: T, timeout: Duration, uf: &impl UniversalFunctions) -> Result<Option<T>, Self::Error> {
        self.send_stream.send_timeout(val, timeout, uf)
    }

    fn send_slice_timeout(&self, slice: &[T], timeout: Duration, uf: &impl UniversalFunctions) -> Result<usize, Self::Error> where T: Copy {
        self.send_stream.send_slice_timeout(slice, timeout, uf)
    }

    fn send_vec_timeout(&self, data: Vec<T>, timeout: Duration, uf: &impl UniversalFunctions) -> Result<Option<Vec<T>>, Self::Error> {
        self.send_stream.send_vec_timeout(data, timeout, uf)
    }
}
impl<T, S, R> ReceiveStream<T> for ComposedStream<T, S, R> where T: Send, S: SendStream<T>, R: ReceiveStream<T>{
    type Error = R::Error;

    fn try_receive(&self) -> Result<Option<T>, Self::Error> {
        self.receive_stream.try_receive()
    }

    fn receive(&self) -> Result<T, Self::Error> {
        self.receive_stream.receive()
    }

    fn receive_slice(&self, buffer: &mut [T]) -> Result<usize, Self::Error> {
        self.receive_stream.receive_slice(buffer)
    }

    fn receive_all(&self, buffer: &mut [T]) -> Result<(), Self::Error> {
        self.receive_stream.receive_all(buffer)
    }

    fn receive_vec(&self, limit: usize) -> Result<Vec<T>, Self::Error> {
        self.receive_stream.receive_vec(limit)
    }
}
impl<T, S, R> ReceiveTimoutStream<T> for ComposedStream<T, S, R> where T: Send, S: SendStream<T>, R: ReceiveTimoutStream<T>{
    fn receive_timeout(&self, timeout: Duration, uf: &impl UniversalFunctions) -> Result<Option<T>, Self::Error> {
        self.receive_stream.receive_timeout(timeout, uf)
    }

    fn receive_slice_timeout(&self, buffer: &mut [T], timeout: Duration, uf: &impl UniversalFunctions) -> Result<usize, Self::Error> {
        self.receive_stream.receive_slice_timeout(buffer, timeout, uf)
    }

    fn receive_all_timeout(&self, buffer: &mut [T], timeout: Duration, uf: &impl UniversalFunctions) -> Result<bool, Self::Error> {
        self.receive_stream.receive_all_timeout(buffer, timeout, uf)
    }

    fn receive_vec_timeout(&self, limit: usize, timeout: Duration, uf: &impl UniversalFunctions) -> Result<Vec<T>, Self::Error> {
        self.receive_stream.receive_vec_timeout(limit, timeout, uf)
    }
}
impl<T, S, R> DuplexStream<T> for ComposedStream<T, S, R> where T: Send, S: SendStream<T>, R: ReceiveStream<T>{}
impl<T, S, R> DuplexTimeoutStream<T> for ComposedStream<T, S, R> where T: Send, S: SendTimeoutStream<T>, R: ReceiveTimoutStream<T>{}
