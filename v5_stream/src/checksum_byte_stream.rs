use v5_traits::UniversalFunctions;
use v5_traits::stream::{SendStream, ReceiveStream, DuplexStream};
use alloc::vec::Vec;
use alloc::{format, vec};
use crc::crc64::checksum_ecma;
use serde::{Serialize, Deserialize};
use v5_traits::mutex::Mutex;
use core::mem::size_of;

const PACKET_START_BYTES: [u8; 6] = [132, 35, 53, 2, 100, 94];
const MAX_PACKET_LEN: usize = 8192;
//TODO: Add packet resending
#[derive(Debug)]
pub struct ChecksumByteStream<UF, S, M> where UF: UniversalFunctions, S: DuplexStream<SData=u8, RData=u8>, M: Mutex<Inner=S>{
    uf: UF,
    stream: M,
}
impl<UF, S, M> ChecksumByteStream<UF, S, M> where UF: UniversalFunctions, S: DuplexStream<SData=u8, RData=u8>, M: Mutex<Inner=S>{
    pub fn new(uf: UF, stream: S) -> Self {
        Self { uf, stream: M::new(stream) }
    }
}
impl<UF, S, M> SendStream for ChecksumByteStream<UF, S, M> where UF: UniversalFunctions, S: DuplexStream<SData=u8, RData=u8>, M: Mutex<Inner=S>{
    type SData = Vec<u8>;

    fn send(&self, val: Vec<u8>){
        let header = ChecksumPacket {
            data_checksum: checksum_ecma(&val),
            data: val,
        };
        let data_out = match serde_cbor::to_vec(&header){
            Ok(val) => val,
            Err(error) => {
                self.uf.log_error(||format!("Serde Cbor Error: {}", error));
                return;
            },
        };
        if data_out.len() > MAX_PACKET_LEN{
            self.uf.log_error(||format!("Max Packet Length exceeded, length: {}, max_length: {}", data_out.len(), MAX_PACKET_LEN));
        }
        else{
            self.stream.lock(|stream|{
                stream.send_slice(&PACKET_START_BYTES);
                stream.send_slice(&data_out.len().to_be_bytes());
                stream.send_vec(data_out);
            });
        }
    }
}
impl<UF, S, M> ReceiveStream for ChecksumByteStream<UF, S, M> where UF: UniversalFunctions, S: DuplexStream<SData=u8, RData=u8>, M: Mutex<Inner=S>{
    type RData = Vec<u8>;

    fn try_receive(&self) -> Option<Vec<u8>> {
        self.stream.lock(|stream|{
            match stream.try_receive(){
                Some(val) => {
                    if val == PACKET_START_BYTES[0] {
                        match receive_packet(&self.uf, stream, 1){
                            Ok(val) => Some(val),
                            Err(_) => None,
                        }
                    }
                    else{
                        None
                    }
                },
                None => None,
            }
        })
    }

    fn receive(&self) -> Self::RData {
        self.stream.lock(|stream| {
            let mut received_last = false;
            loop {
                match receive_packet(&self.uf, stream, if received_last {1} else {0}) {
                    Ok(data) => return data,
                    Err(last) => received_last = last,
                }
            }
        })
    }
}
impl<UF, S, M> DuplexStream for ChecksumByteStream<UF, S, M> where UF: UniversalFunctions, S: DuplexStream<SData=u8, RData=u8>, M: Mutex<Inner=S>{}

///Errs with true if the last byte received match packet start byte
fn receive_packet<S>(uf: &impl UniversalFunctions, stream: &mut S, successfully_read: usize) -> Result<Vec<u8>, bool> where S: DuplexStream<SData=u8, RData=u8>{
    for index in successfully_read..PACKET_START_BYTES.len(){
        let temp = stream.receive();
        if temp != PACKET_START_BYTES[index]{
            return Err(temp == PACKET_START_BYTES[0]);
        }
    }
    //packet start confirmed
    let mut size_bytes = [0; size_of::<u64>()];
    stream.receive_all(&mut size_bytes);
    let size = u64::from_be_bytes(size_bytes);
    let mut packet_bytes = vec![0; size as usize];
    stream.receive_all(&mut packet_bytes);
    let packet: ChecksumPacket = match serde_cbor::from_slice(&packet_bytes){
        Ok(val) => val,
        Err(error) => {
            uf.log_error(||format!("Serde cbor error: {}", error));
            return Err(false);
        },
    };
    if checksum_ecma(&packet.data) != packet.data_checksum{
        uf.log_error(||"Checksum Error");
        Err(false)
    }
    else{
        Ok(packet.data)
    }

}

#[derive(Debug, Serialize, Deserialize)]
struct ChecksumPacket {
    data: Vec<u8>,
    data_checksum: u64,
}
