use serde::{Serialize, Deserialize};
use core::marker::PhantomData;
use core::mem::size_of;
use alloc::vec;
use v5_traits::stream::{SendStream, ReceiveStream, DuplexStream};
use v5_traits::error::Error;

pub struct SerializeSendStream<T, S> where T: 'static + Serialize, S: SendStream<u8>{
    stream: S,
    phantom_t: PhantomData<T>,
}
impl<T, S> SerializeSendStream<T, S> where T: 'static + Serialize, S: SendStream<u8>{
    pub fn new(stream: S) -> Self{
        Self{ stream, phantom_t: Default::default() }
    }
}
impl<T, S> SendStream<T> for SerializeSendStream<T, S> where T: 'static + Serialize, S: SendStream<u8>{
    type Error = SerializeStreamError<S::Error>;

    fn try_send(&self, val: T) -> Result<Option<T>, Self::Error> {
        unimplemented!()
    }

    fn send(&self, val: T) -> Result<(), Self::Error> {
        send(&self.stream, &val)
    }
}

pub struct SerializeReceiveStream<T, S> where T: 'static + for<'a> Deserialize<'a>, S: ReceiveStream<u8>{
    stream: S,
    phantom_t: PhantomData<T>,
}
impl<T, S> SerializeReceiveStream<T, S> where T: 'static + for<'a> Deserialize<'a>, S: ReceiveStream<u8>{
    pub fn new(stream: S) -> Self{
        Self{ stream, phantom_t: Default::default() }
    }
}
impl<T, S> ReceiveStream<T> for SerializeReceiveStream<T, S> where T: 'static + for<'a> Deserialize<'a>, S: ReceiveStream<u8>{
    type Error = SerializeStreamError<S::Error>;

    fn receive(&self) -> Result<T, Self::Error> {
        receive(&self.stream)
    }
}

pub struct DuplexSerializeStream<T, S> where T: 'static + Serialize + for<'a> Deserialize<'a>, S: DuplexStream<u8>{
    stream: S,
    phantom_t: PhantomData<T>,
}
impl<T, S> DuplexSerializeStream<T, S> where T: 'static + Serialize + for<'a> Deserialize<'a>, S: DuplexStream<u8>{
    pub fn new(stream: S) -> Self{
        Self{ stream, phantom_t: Default::default() }
    }
}
impl<T, S> SendStream<T> for DuplexSerializeStream<T, S> where T: 'static + Serialize + for<'a> Deserialize<'a>, S: DuplexStream<u8>{
    type Error = SerializeStreamError<<S as SendStream<u8>>::Error>;

    fn send(&self, val: T) -> Result<(), Self::Error> {
        send(&self.stream, &val)
    }
}
impl<T, S> ReceiveStream<T> for DuplexSerializeStream<T, S> where T: 'static + Serialize + for<'a> Deserialize<'a>, S: DuplexStream<u8>{
    type Error = SerializeStreamError<<S as ReceiveStream<u8>>::Error>;

    fn receive(&self) -> Result<T, Self::Error> {
        unimplemented!()
    }
}
impl<T, S> DuplexStream<T> for DuplexSerializeStream<T, S> where T: 'static + Serialize + for<'a> Deserialize<'a>, S: DuplexStream<u8>{}

#[derive(Debug)]
pub enum SerializeStreamError<E> where E: Error{
    StreamError(E),
    SerdeCborError(serde_cbor::Error),
}
impl<E> From<serde_cbor::Error> for SerializeStreamError<E> where E: Error{
    fn from(from: serde_cbor::Error) -> Self {
        Self::SerdeCborError(from)
    }
}
impl<E> Error for SerializeStreamError<E> where E: Error{}

fn try_send<T, S>(stream: &S, data: T) -> Result<Option<T>, SerializeStreamError<S::Error>> where T: 'static + Serialize, S: SendStream<u8>{

}
fn send<T, S>(stream: &S, data: &T) -> Result<(), SerializeStreamError<S::Error>> where T: 'static + Serialize, S: SendStream<u8>{
    let bytes = serde_cbor::to_vec(data)?;

    if let Err(error) = stream.send_all(&bytes.len().to_be_bytes()){
        return Err(SerializeStreamError::StreamError(error));
    }
    if let Err(error) = stream.send_all(&bytes){
        return Err(SerializeStreamError::StreamError(error));
    }
    Ok(())
}
fn receive<T, S>(stream: &S) -> Result<T, SerializeStreamError<S::Error>> where T: 'static + for<'a> Deserialize<'a>, S: ReceiveStream<u8>{
    let mut len_bytes = [0; size_of::<usize>()];
    if let Err(error) = stream.receive_all(&mut len_bytes){
        return Err(SerializeStreamError::StreamError(error));
    }
    let len = usize::from_be_bytes(len_bytes);
    let mut out_bytes = vec![0; len];
    if let Err(error) = stream.receive_all(&mut out_bytes){
        return Err(SerializeStreamError::StreamError(error));
    }
    Ok(serde_cbor::from_slice(&out_bytes)?)
}
