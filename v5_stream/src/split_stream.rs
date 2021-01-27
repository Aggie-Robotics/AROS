use v5_traits::stream::{SendStream, DuplexStream, ReceiveStream};
use v5_traits::{UniversalFunctions, EnsureSend, EnsureSync};
use alloc::sync::Arc;
use alloc::vec::Vec;

pub fn split_stream<UF, S>(uf: UF, stream: S) -> (SplitSender<UF, S>, SplitReceiver<UF, S>) where UF: UniversalFunctions, S: DuplexStream{
    let stream = Arc::new(stream);
    (
        SplitSender{ uf: uf.clone(), stream: stream.clone() },
        SplitReceiver{ uf, stream },
    )
}

#[derive(Debug, Clone)]
pub struct SplitSender<UF, S> where UF: UniversalFunctions, S: SendStream{
    uf: UF,
    stream: Arc<S>,
}
impl<UF, S> SendStream for SplitSender<UF, S> where UF: UniversalFunctions, S: SendStream{
    type SData = S::SData;
    type Error = S::Error;

    fn send(&self, val: Self::SData) -> Result<(), Self::Error> {
        self.stream.send(val)
    }

    fn send_slice(&self, slice: &[Self::SData]) -> Result<(), Self::Error> where Self::SData: Copy {
        self.stream.send_slice(slice)
    }

    fn send_vec(&self, data: Vec<Self::SData>) -> Result<(), Self::Error> {
        self.stream.send_vec(data)
    }
}
impl<UF, S> EnsureSend for SplitSender<UF, S> where UF: UniversalFunctions, S: SendStream + Send + Sync{}
impl<UF, S> EnsureSync for SplitSender<UF, S> where UF: UniversalFunctions, S: SendStream + Send + Sync{}

#[derive(Debug, Clone)]
pub struct SplitReceiver<UF, S> where UF: UniversalFunctions, S: ReceiveStream{
    uf: UF,
    stream: Arc<S>,
}
impl<UF, S> ReceiveStream for SplitReceiver<UF, S> where UF: UniversalFunctions, S: ReceiveStream{
    type RData = S::RData;
    type Error = S::Error;

    fn try_receive(&self) -> Result<Option<Self::RData>, Self::Error> {
        self.stream.try_receive()
    }

    fn receive(&self) -> Result<Self::RData, Self::Error> {
        self.stream.receive()
    }

    fn receive_slice(&self, buffer: &mut [Self::RData]) -> Result<usize, Self::Error> {
        self.stream.receive_slice(buffer)
    }

    fn receive_all(&self, buffer: &mut [Self::RData]) -> Result<(), Self::Error> {
        self.stream.receive_all(buffer)
    }

    fn receive_vec(&self, limit: usize) -> Result<Vec<Self::RData>, Self::Error> {
        self.stream.receive_vec(limit)
    }

    fn receive_whole_vec(&self, limit: usize) -> Result<Vec<Self::RData>, Self::Error> {
        self.stream.receive_whole_vec(limit)
    }
}
impl<UF, S> EnsureSend for SplitReceiver<UF, S> where UF: UniversalFunctions, S: ReceiveStream + Send + Sync{}
impl<UF, S> EnsureSync for SplitReceiver<UF, S> where UF: UniversalFunctions, S: ReceiveStream + Send + Sync{}
