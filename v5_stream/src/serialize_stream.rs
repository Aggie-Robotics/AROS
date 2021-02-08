use v5_traits::stream::{DuplexStream, SendStream, ReceiveStream};
use alloc::vec::Vec;
use v5_traits::{UniversalFunctions, LogLevel, EnsureSync, EnsureSend};
use core::marker::PhantomData;
use serde::Serialize;
use v5_traits::error::Error;
use serde::de::DeserializeOwned;

pub struct SerializeStream<UF, T, S>
    where UF: UniversalFunctions,
          T: 'static + Send + Serialize + DeserializeOwned,
          S: DuplexStream<SData=Vec<u8>, RData=Vec<u8>>{
    uf: UF,
    stream: S,
    phantom_t: PhantomData<T>,
}
impl<UF, T, S> SerializeStream<UF, T, S>
    where UF: UniversalFunctions,
          T: 'static + Send + Serialize + DeserializeOwned,
          S: DuplexStream<SData=Vec<u8>, RData=Vec<u8>>{
    pub fn new(uf: UF, stream: S) -> Self{
        Self{ uf, stream, phantom_t: Default::default() }
    }

    pub fn stream(&self) -> &S where S: Sync{
        &self.stream
    }
}
/// This ensures that this is sync if possible because no T is actually stored
unsafe impl<UF, T, S> Sync for SerializeStream<UF, T, S>
    where UF: UniversalFunctions + Sync,
          T: 'static + Send + Serialize + DeserializeOwned,
          S: DuplexStream<SData=Vec<u8>, RData=Vec<u8>> + Sync{}
impl<UF, T, S> SendStream for SerializeStream<UF, T, S>
    where UF: UniversalFunctions,
          T: 'static + Send + Serialize + DeserializeOwned,
          S: DuplexStream<SData=Vec<u8>, RData=Vec<u8>>{
    type SData = T;

    fn send(&self, val: T) {
        self.stream.send(match serde_cbor::to_vec(&val){
            Ok(data) => data,
            Err(error) => {
                self.uf.log(||format!("Serde cbor error while serializing: {}", error), LogLevel::ERROR);
                return;
            },
        })
    }
}
impl<UF, T, S> ReceiveStream for SerializeStream<UF, T, S>
    where UF: UniversalFunctions,
          T: 'static + Send + Serialize + DeserializeOwned,
          S: DuplexStream<SData=Vec<u8>, RData=Vec<u8>>{
    type RData = T;

    fn try_receive(&self) -> Option<Self::RData> {
        self.stream.try_receive().and_then(|val|{
            match serde_cbor::from_slice(&val){
                Ok(val) => Some(val),
                Err(error) => {
                    self.uf.log_error(||format!("Serde cbor error: {}", error));
                    None
                },
            }
        })
    }

    fn receive(&self) -> T {
        loop {
            match serde_cbor::from_slice(&self.stream.receive()) {
                Ok(val) => return val,
                Err(error) => {
                    self.uf.log_error(|| format!("Serde cbor error: {}", error));
                },
            }
        }
    }
}
impl<UF, T, S> DuplexStream for SerializeStream<UF, T, S>
    where UF: UniversalFunctions,
          T: 'static + Send + Serialize + DeserializeOwned,
          S: DuplexStream<SData=Vec<u8>, RData=Vec<u8>>{}
impl<UF, T, S> EnsureSync for SerializeStream<UF, T, S>
    where UF: UniversalFunctions + Sync,
          T: Send + Serialize + DeserializeOwned,
          S: DuplexStream<SData=Vec<u8>, RData=Vec<u8>> + Sync{}
impl<UF, T, S> EnsureSend for SerializeStream<UF, T, S>
    where UF: UniversalFunctions + Send,
          T: Send + Serialize + DeserializeOwned,
          S: DuplexStream<SData=Vec<u8>, RData=Vec<u8>> + Send{}

#[derive(Debug)]
pub enum SerializeStreamError<E> where E: Error{
    StreamError(E),
    SerdeCborError(serde_cbor::Error),
}
impl<E> Error for SerializeStreamError<E> where E: Error{
    fn is_recoverable(&self) -> bool {
        match self {
            SerializeStreamError::StreamError(error) => error.is_recoverable(),
            SerializeStreamError::SerdeCborError(_) => true,
        }
    }
}
impl<E> From<E> for SerializeStreamError<E> where E: Error{
    fn from(from: E) -> Self {
        Self::StreamError(from)
    }
}
