use v5_traits::stream::{DuplexStream, SendStream, ReceiveStream};
use alloc::vec::Vec;
use v5_traits::{UniversalFunctions, LogLevel, EnsureSync, EnsureSend};
use core::marker::PhantomData;
use serde::{Serialize, Deserialize};
use v5_traits::error::Error;
use alloc::format;

pub struct SerializeStream<UF, T, S>
    where UF: UniversalFunctions,
          T: 'static + Send + Serialize + for<'de> Deserialize<'de>,
          S: DuplexStream<SData=Vec<u8>, RData=Vec<u8>>{
    uf: UF,
    stream: S,
    phantom_t: PhantomData<T>,
}
impl<UF, T, S> SerializeStream<UF, T, S>
    where UF: UniversalFunctions,
          T: 'static + Send + Serialize + for<'de> Deserialize<'de>,
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
          T: 'static + Send + Serialize + for<'de> Deserialize<'de>,
          S: DuplexStream<SData=Vec<u8>, RData=Vec<u8>> + Sync{}
impl<UF, T, S> SendStream for SerializeStream<UF, T, S>
    where UF: UniversalFunctions,
          T: 'static + Send + Serialize + for<'de> Deserialize<'de>,
          S: DuplexStream<SData=Vec<u8>, RData=Vec<u8>>{
    type SData = T;
    type Error = SerializeStreamError<<S as SendStream>::Error>;

    fn send(&self, val: T) -> Result<(), Self::Error> {
        Ok(self.stream.send(match serde_cbor::to_vec(&val){
            Ok(data) => data,
            Err(error) => {
                self.uf.log(||format!("Serde cbor error while serializing: {}", error), LogLevel::ERROR);
                return Err(SerializeStreamError::SerdeCborError(error))
            },
        })?)
    }
}
impl<UF, T, S> ReceiveStream for SerializeStream<UF, T, S>
    where UF: UniversalFunctions,
          T: 'static + Send + Serialize + for<'de> Deserialize<'de>,
          S: DuplexStream<SData=Vec<u8>, RData=Vec<u8>>{
    type RData = T;
    type Error = SerializeStreamError<<S as ReceiveStream>::Error>;

    fn try_receive(&self) -> Result<Option<T>, Self::Error> {
        self.stream.try_receive()?.map_or(Ok(None), |val|{
            match serde_cbor::from_slice(&val){
                Ok(val) => Ok(Some(val)),
                Err(error) => Err(SerializeStreamError::SerdeCborError(error)),
            }
        })
    }

    fn receive(&self) -> Result<T, Self::Error> {
        match serde_cbor::from_slice(&self.stream.receive()?){
            Ok(val) => Ok(val),
            Err(error) => Err(SerializeStreamError::SerdeCborError(error)),
        }
    }
}
impl<UF, T, S> DuplexStream for SerializeStream<UF, T, S>
    where UF: UniversalFunctions,
          T: 'static + Send + Serialize + for<'de> Deserialize<'de>,
          S: DuplexStream<SData=Vec<u8>, RData=Vec<u8>>{

}
impl<UF, T, S> EnsureSync for SerializeStream<UF, T, S>
    where UF: UniversalFunctions + Sync,
          T: Send + Serialize + for<'de> Deserialize<'de>,
          S: DuplexStream<SData=Vec<u8>, RData=Vec<u8>> + Sync{}
impl<UF, T, S> EnsureSend for SerializeStream<UF, T, S>
    where UF: UniversalFunctions + Send,
          T: Send + Serialize + for<'de> Deserialize<'de>,
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
