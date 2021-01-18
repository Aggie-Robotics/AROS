use v5_traits::stream::{SendStream, DuplexStream, ReceiveStream};
use v5_traits::{UniversalFunctions, EnsureSend, EnsureSync};
use alloc::sync::Arc;
use core::marker::PhantomData;
use serde::__private::Vec;

pub fn split_stream<UF, T, S>(uf: UF, stream: S) -> (SplitSender<UF, T, S>, SplitReceiver<UF, T, S>) where UF: UniversalFunctions, T: 'static + Send, S: DuplexStream<T>{
    let stream = Arc::new(stream);
    (
        SplitSender{ uf: uf.clone(), stream: stream.clone(), phantom_t: Default::default() },
        SplitReceiver{ uf, stream, phantom_t: Default::default() },
    )
}

#[derive(Debug, Clone)]
pub struct SplitSender<UF, T, S> where UF: UniversalFunctions, T: 'static + Send, S: SendStream<T>{
    uf: UF,
    stream: Arc<S>,
    phantom_t: PhantomData<T>,
}
impl<UF, T, S> SendStream<T> for SplitSender<UF, T, S> where UF: UniversalFunctions, T: 'static + Send, S: SendStream<T>{
    type Error = S::Error;

    fn send(&self, val: T) -> Result<(), Self::Error> {
        self.stream.send(val)
    }

    fn send_slice(&self, slice: &[T]) -> Result<(), Self::Error> where T: Copy {
        self.stream.send_slice(slice)
    }

    fn send_vec(&self, data: Vec<T>) -> Result<(), Self::Error> {
        self.stream.send_vec(data)
    }
}
impl<UF, T, S> EnsureSend for SplitSender<UF, T, S> where UF: UniversalFunctions, T: 'static + Send, S: SendStream<T> + Send + Sync{}
impl<UF, T, S> EnsureSync for SplitSender<UF, T, S> where UF: UniversalFunctions, T: 'static + Send, S: SendStream<T> + Send + Sync{}
unsafe impl<UF, T, S> Sync for SplitSender<UF, T, S> where UF: UniversalFunctions + Send + Sync, T: 'static + Send, S: SendStream<T> + Send + Sync{}

#[derive(Debug, Clone)]
pub struct SplitReceiver<UF, T, S> where UF: UniversalFunctions, T: 'static + Send, S: ReceiveStream<T>{
    uf: UF,
    stream: Arc<S>,
    phantom_t: PhantomData<T>,
}
impl<UF, T, S> ReceiveStream<T> for SplitReceiver<UF, T, S> where UF: UniversalFunctions, T: 'static + Send, S: ReceiveStream<T>{
    type Error = S::Error;

    fn try_receive(&self) -> Result<Option<T>, Self::Error> {
        self.stream.try_receive()
    }

    fn receive(&self) -> Result<T, Self::Error> {
        self.stream.receive()
    }

    fn receive_slice(&self, buffer: &mut [T]) -> Result<usize, Self::Error> {
        self.stream.receive_slice(buffer)
    }

    fn receive_all(&self, buffer: &mut [T]) -> Result<(), Self::Error> {
        self.stream.receive_all(buffer)
    }

    fn receive_vec(&self, limit: usize) -> Result<Vec<T>, Self::Error> {
        self.stream.receive_vec(limit)
    }

    fn receive_whole_vec(&self, limit: usize) -> Result<Vec<T>, Self::Error> {
        self.stream.receive_whole_vec(limit)
    }
}
