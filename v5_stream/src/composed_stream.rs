use core::fmt::Debug;
use core::marker::PhantomData;

use v5_traits::stream::{DuplexStream, ReceiveStream, SendStream};
use alloc::vec::Vec;

#[derive(Debug)]
pub struct ComposedStream<T, S, R> where T: Send, S: SendStream<T>, R: ReceiveStream<T>{
    pub send_stream: S,
    pub receive_stream: R,
    phantom_t: PhantomData<T>,
}
impl<T, S, R> ComposedStream<T, S, R> where T: Send, S: SendStream<T>, R: ReceiveStream<T>{
    pub fn new(send_stream: S, receive_stream: R) -> Self{
        Self{ send_stream, receive_stream, phantom_t: Default::default() }
    }
}
impl<T, S, R> SendStream<T> for ComposedStream<T, S, R> where T: Send, S: SendStream<T>, R: ReceiveStream<T>{
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
impl<T, S, R> DuplexStream<T> for ComposedStream<T, S, R> where T: Send, S: SendStream<T>, R: ReceiveStream<T>{}