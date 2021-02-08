use v5_traits::stream::{SendStream, DuplexStream, ReceiveStream, SendTimeoutStream, ReceiveTimoutStream};
use v5_traits::{UniversalFunctions, EnsureSend, EnsureSync};
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::time::Duration;

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

    fn send(&self, val: Self::SData)  {
        self.stream.send(val)
    }

    fn send_slice(&self, slice: &[Self::SData]) where Self::SData: Copy {
        self.stream.send_slice(slice)
    }

    fn send_vec(&self, data: Vec<Self::SData>) {
        self.stream.send_vec(data)
    }
}
impl<UF, S> SendTimeoutStream for SplitSender<UF, S> where UF: UniversalFunctions, S: SendTimeoutStream{
    fn send_timeout(&self, val: Self::SData, timeout: Duration, uf: &impl UniversalFunctions) -> Option<Self::SData> {
        self.stream.send_timeout(val, timeout, uf)
    }

    fn send_slice_timeout(&self, slice: &[Self::SData], timeout: Duration, uf: &impl UniversalFunctions) -> usize where Self::SData: Copy {
        self.stream.send_slice_timeout(slice, timeout, uf)
    }

    fn send_vec_timeout(&self, data: Vec<Self::SData>, timeout: Duration, uf: &impl UniversalFunctions) -> Option<Vec<Self::SData>> {
        self.stream.send_vec_timeout(data, timeout, uf)
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

    fn try_receive(&self) -> Option<Self::RData> {
        self.stream.try_receive()
    }

    fn receive(&self) -> Self::RData {
        self.stream.receive()
    }

    fn receive_slice(&self, buffer: &mut [Self::RData]) -> usize {
        self.stream.receive_slice(buffer)
    }

    fn receive_all(&self, buffer: &mut [Self::RData]) {
        self.stream.receive_all(buffer)
    }

    fn receive_vec(&self, limit: usize) -> Vec<Self::RData>{
        self.stream.receive_vec(limit)
    }

    fn receive_whole_vec(&self, limit: usize) -> Vec<Self::RData> {
        self.stream.receive_whole_vec(limit)
    }
}
impl<UF, S> ReceiveTimoutStream for SplitReceiver<UF, S> where UF: UniversalFunctions, S: ReceiveTimoutStream{
    fn receive_timeout(&self, timeout: Duration, uf: &impl UniversalFunctions) -> Option<Self::RData> {
        self.stream.receive_timeout(timeout, uf)
    }

    fn receive_slice_timeout(&self, buffer: &mut [Self::RData], timeout: Duration, uf: &impl UniversalFunctions) -> usize {
        self.stream.receive_slice_timeout(buffer, timeout, uf)
    }

    fn receive_vec_timeout(&self, limit: usize, timeout: Duration, uf: &impl UniversalFunctions) -> Vec<Self::RData> {
        self.stream.receive_vec_timeout(limit, timeout, uf)
    }
}
impl<UF, S> EnsureSend for SplitReceiver<UF, S> where UF: UniversalFunctions, S: ReceiveStream + Send + Sync{}
impl<UF, S> EnsureSync for SplitReceiver<UF, S> where UF: UniversalFunctions, S: ReceiveStream + Send + Sync{}
