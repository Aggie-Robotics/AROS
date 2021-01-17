use v5_traits::stream::{DuplexStream, SendStream, ReceiveStream};
use alloc::vec::Vec;
use v5_traits::UniversalFunctions;
use alloc::vec;
use core::mem::size_of;

#[derive(Debug)]
pub struct SimpleByteStream<UF, S>
    where UF: UniversalFunctions,
          S: DuplexStream<u8>{
    uf: UF,
    stream: S,

}
impl<UF, S> SimpleByteStream<UF, S>
    where UF: UniversalFunctions,
          S: DuplexStream<u8>{
    pub fn new(uf: UF, stream: S) -> Self{
        Self{
            uf,
            stream,
        }
    }
}
impl<UF, S> SendStream<Vec<u8>> for SimpleByteStream<UF, S>
    where UF: UniversalFunctions,
          S: DuplexStream<u8>{
    type Error = <S as SendStream<u8>>::Error;

    fn send(&self, val: Vec<u8>) -> Result<(), Self::Error> {
        self.stream.send_slice(&(val.len() as u64).to_be_bytes())?;
        self.stream.send_vec(val)
    }
}
impl<UF, S> ReceiveStream<Vec<u8>> for SimpleByteStream<UF, S>
    where UF: UniversalFunctions,
          S: DuplexStream<u8>{
    type Error = <S as ReceiveStream<u8>>::Error;

    fn try_receive(&self) -> Result<Option<Vec<u8>>, Self::Error> {
        let byte = match self.stream.try_receive()?{
            None => return Ok(None),
            Some(byte) => byte,
        };
        let mut len_bytes = [0; size_of::<u64>()];
        len_bytes[0] = byte;
        self.stream.receive_all(&mut len_bytes[1..])?;
        let len = u64::from_be_bytes(len_bytes);
        let mut data = vec![0; len as usize];
        self.stream.receive_all(&mut data)?;
        Ok(Some(data))
    }

    fn receive(&self) -> Result<Vec<u8>, Self::Error> {
        let mut len_bytes = [0; size_of::<u64>()];
        self.stream.receive_all(&mut len_bytes)?;
        let len = u64::from_be_bytes(len_bytes);
        let mut data = vec![0; len as usize];
        self.stream.receive_all(&mut data)?;
        Ok(data)
    }
}
impl<UF, S> DuplexStream<Vec<u8>> for SimpleByteStream<UF, S>
    where UF: UniversalFunctions,
          S: DuplexStream<u8>{}
