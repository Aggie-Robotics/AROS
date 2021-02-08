use v5_traits::stream::{DuplexStream, SendStream, ReceiveStream};
use alloc::vec::Vec;
use v5_traits::UniversalFunctions;
use alloc::vec;
use core::mem::size_of;
use v5_traits::mutex::Mutex;

#[derive(Debug)]
pub struct SimpleByteStream<UF, S, M> where UF: UniversalFunctions, S: DuplexStream<SData=u8, RData=u8>, M: Mutex<Inner=S>{
    uf: UF,
    stream: M,
}
impl<UF, S, M> SimpleByteStream<UF, S, M> where UF: UniversalFunctions, S: DuplexStream<SData=u8, RData=u8>, M: Mutex<Inner=S>{
    pub fn new(uf: UF, stream: S) -> Self{
        Self{ uf, stream: M::new(stream) }
    }
}
impl<UF, S, M> SendStream for SimpleByteStream<UF, S, M> where UF: UniversalFunctions, S: DuplexStream<SData=u8, RData=u8>, M: Mutex<Inner=S>{
    type SData = Vec<u8>;

    fn send(&self, val: Vec<u8>) {
        self.stream.lock(|stream|{
            stream.send_slice(&(val.len() as u64).to_be_bytes());
            stream.send_vec(val)
        })
    }
}
impl<UF, S, M> ReceiveStream for SimpleByteStream<UF, S, M> where UF: UniversalFunctions, S: DuplexStream<SData=u8, RData=u8>, M: Mutex<Inner=S>{
    type RData = Vec<u8>;

    fn try_receive(&self) -> Option<Vec<u8>> {
        match self.stream.try_lock(|stream|{
            let byte = match stream.try_receive(){
                None => return None,
                Some(byte) => byte,
            };
            let mut len_bytes = [0; size_of::<u64>()];
            len_bytes[0] = byte;
            stream.receive_all(&mut len_bytes[1..]);
            let len = u64::from_be_bytes(len_bytes);
            let mut data = vec![0; len as usize];
            stream.receive_all(&mut data);
            Some(data)
        }){
            Ok(val) => val,
            Err(_) => None,
        }
    }

    fn receive(&self) -> Vec<u8> {
        self.stream.lock(|stream|{
            let mut len_bytes = [0; size_of::<u64>()];
            stream.receive_all(&mut len_bytes);
            let len = u64::from_be_bytes(len_bytes);
            let mut data = vec![0; len as usize];
            stream.receive_all(&mut data);
            data
        })
    }
}
impl<UF, S, M> DuplexStream for SimpleByteStream<UF, S, M> where UF: UniversalFunctions, S: DuplexStream<SData=u8, RData=u8>, M: Mutex<Inner=S> {}
