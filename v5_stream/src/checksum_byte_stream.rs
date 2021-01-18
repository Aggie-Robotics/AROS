use v5_traits::UniversalFunctions;
use v5_traits::stream::{MessageStreamCreator, SendStream, ReceiveStream, DuplexTimeoutStream, ReceiveTimoutStream, DuplexStream};
use core::time::Duration;
use alloc::vec::Vec;
use alloc::{format, vec};
use core::mem::size_of;
use crc::crc64::checksum_ecma;
use serde::{Serialize, Deserialize};
use core::sync::atomic::AtomicU64;
use atomic::Ordering;

const PACKET_START_BYTES: [u8; 6] = [132, 35, 53, 2, 100, 94];

#[derive(Debug)]
pub struct ChecksumByteStream<UF, S, C> where UF: UniversalFunctions, S: DuplexTimeoutStream<u8>, C: MessageStreamCreator<Vec<u8>>{
    uf: UF,
    stream: S,
    outbound_stream: (C::Sender, C::Receiver),
    inbound_stream: (C::Sender, C::Receiver),
    send_sequence_counter: AtomicU64,
    received_sequence_counter: AtomicU64,
}
impl<UF, S, C> ChecksumByteStream<UF, S, C> where UF: UniversalFunctions, S: DuplexTimeoutStream<u8>, C: MessageStreamCreator<Vec<u8>> {
    pub fn new(uf: UF, stream: S, creator: &C) -> Self {
        Self {
            uf,
            stream,
            outbound_stream: creator.create_stream(),
            inbound_stream: creator.create_stream(),
            send_sequence_counter: AtomicU64::new(0),
            received_sequence_counter: AtomicU64::new(0),
        }
    }

    pub fn stream(&self) -> &S where S: Sync{
        &self.stream
    }

    pub fn management_loop(&self, timeouts: ChecksumByteTimeouts){
        let mut packet_start_buffer = [0; PACKET_START_BYTES.len()];
        let mut found = 0;
        loop {
            while let Some(received_packet) = self.receive_packet(&timeouts, &mut packet_start_buffer, &mut found){
                match received_packet.0.packet_type{
                    ChecksumPacketType::DataPacket { .. } => {
                        let expected_seq = self.received_sequence_counter.swap(received_packet.0.sequence + 1, Ordering::SeqCst);
                        if expected_seq != received_packet.0.sequence{
                            self.uf.log_debug(||format!("Skipped received seq, expected: {}, received: {}", expected_seq, received_packet.0.sequence));
                        }
                        if let Err(error) = self.inbound_stream.0.send(received_packet.1.expect("DataPacket had no data with it")){
                            self.uf.log_error(||format!("Error sending to own inbound stream: {:?}", error));
                        }
                    }
                }
            }

            'SendWhile: while let Some(outbound) = match self.outbound_stream.1.try_receive(){
                Ok(outbound) => outbound,
                Err(error) => {
                    self.uf.log_error(||format!("Error receiving outbound: {:?}", error));
                    None
                }
            }{
                let header = ChecksumHeader{
                    packet_type: ChecksumPacketType::DataPacket {
                        data_length: outbound.len() as u64,
                        data_checksum: checksum_ecma(&outbound),
                    },
                    sequence: self.send_sequence_counter.fetch_add(1, Ordering::SeqCst),
                };
                let header_bytes = match serde_cbor::to_vec(&header){
                    Ok(bytes) => bytes,
                    Err(error) => {
                        self.uf.log_error(||format!("Serde cbor error on header({:?}): {}", header, error));
                        continue 'SendWhile;
                    }
                };
                let header_crc = checksum_ecma(&header_bytes);
                let bytes_out: Vec<_> = PACKET_START_BYTES.iter().cloned()
                    .chain((header_bytes.len() as u64).to_be_bytes().iter().cloned())
                    .chain(header_bytes.into_iter())
                    .chain(header_crc.to_be_bytes().iter().cloned())
                    .chain(outbound.into_iter())
                    .collect();
                match self.stream.send_vec_timeout(bytes_out, timeouts.send_packet_timeout, &self.uf) {
                    Err(error) => {
                        self.uf.log_error(||format!("Error sending data packet: {:?}", error));
                    }
                    Ok(sent) => if sent.is_some(){
                        self.uf.log_debug(||format!("Timeout sending data packet"));
                    }
                }
            }
        }
    }

    fn receive_packet(&self, timeouts: &ChecksumByteTimeouts, packet_start_buffer: &mut [u8; PACKET_START_BYTES.len()], found: &mut usize) -> Option<(ChecksumHeader, Option<Vec<u8>>)>{
        let receive_start_end_time = self.uf.system_time() + timeouts.receive_packet_start_timeout;
        while *found < packet_start_buffer.len() && self.uf.system_time() < receive_start_end_time{
            match self.stream.receive_slice_timeout(&mut packet_start_buffer[*found..], receive_start_end_time - self.uf.system_time(), &self.uf){
                Ok(num_received) => {
                    for index in 0..num_received{
                        if packet_start_buffer[*found] == PACKET_START_BYTES[*found]{
                            *found += 1;
                        }
                        else{
                            let mut buffer = [0; PACKET_START_BYTES.len()];
                            let num_unchecked = num_received - index - 1;
                            buffer[0..num_unchecked].copy_from_slice(&packet_start_buffer[*found..*found + num_unchecked]);
                            packet_start_buffer[0..num_unchecked].copy_from_slice(&buffer[0..num_unchecked]);
                            *found = 0;
                        }
                    }
                }
                Err(error) => self.uf.log_error(||format!("Error receiving slice timeout: {:?}", error)),
            }
        }
        if *found == PACKET_START_BYTES.len() {
            *found = 0;
            ChecksumHeader::get_packet(&self.stream, timeouts.receive_packet_timeout, &self.uf)
        }
        else {
            None
        }
    }
}
impl<UF, S, C> SendStream<Vec<u8>> for ChecksumByteStream<UF, S, C> where UF: UniversalFunctions, S: DuplexTimeoutStream<u8>, C: MessageStreamCreator<Vec<u8>>{
    type Error = <C::Sender as SendStream<Vec<u8>>>::Error;

    fn send(&self, val: Vec<u8>) -> Result<(), Self::Error> {
        self.outbound_stream.0.send(val)
    }

    fn send_vec(&self, data: Vec<Vec<u8>>) -> Result<(), Self::Error> {
        self.outbound_stream.0.send_vec(data)
    }
}
impl<UF, S, C> ReceiveStream<Vec<u8>> for ChecksumByteStream<UF, S, C> where UF: UniversalFunctions, S: DuplexTimeoutStream<u8>, C: MessageStreamCreator<Vec<u8>>{
    type Error = <C::Receiver as ReceiveStream<Vec<u8>>>::Error;

    fn try_receive(&self) -> Result<Option<Vec<u8>>, Self::Error> {
        self.inbound_stream.1.try_receive()
    }

    fn receive(&self) -> Result<Vec<u8>, Self::Error> {
        self.inbound_stream.1.receive()
    }

    fn receive_slice(&self, buffer: &mut [Vec<u8>]) -> Result<usize, Self::Error> {
        self.inbound_stream.1.receive_slice(buffer)
    }

    fn receive_all(&self, buffer: &mut [Vec<u8>]) -> Result<(), Self::Error> {
        self.inbound_stream.1.receive_all(buffer)
    }

    fn receive_vec(&self, limit: usize) -> Result<Vec<Vec<u8>>, Self::Error> {
        self.inbound_stream.1.receive_vec(limit)
    }

    fn receive_whole_vec(&self, vec: &mut Vec<Vec<u8>>, limit: usize) -> Result<(), Self::Error> {
        self.inbound_stream.1.receive_whole_vec(vec, limit)
    }
}
impl<UF, S, C> DuplexStream<Vec<u8>> for ChecksumByteStream<UF, S, C> where UF: UniversalFunctions, S: DuplexTimeoutStream<u8>, C: MessageStreamCreator<Vec<u8>>{}

#[derive(Copy, Clone, Debug)]
pub struct ChecksumByteTimeouts{
    pub receive_packet_start_timeout: Duration,
    pub receive_packet_timeout: Duration,
    pub send_packet_timeout: Duration,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChecksumHeader{
    packet_type: ChecksumPacketType,
    sequence: u64,
}
impl ChecksumHeader{
    fn get_packet(stream: &impl ReceiveTimoutStream<u8>, timeout: Duration, uf: &impl UniversalFunctions) -> Option<(ChecksumHeader, Option<Vec<u8>>)> {
        let end_time = uf.system_time() + timeout;
        let mut length_bytes = [0; size_of::<u64>()];
        let header: ChecksumHeader = match stream.receive_all_timeout(&mut length_bytes, end_time - uf.system_time(), uf) {
            Ok(result) => if result {
                let length = u64::from_be_bytes(length_bytes) as usize;
                let mut header_bytes = vec![0; length + size_of::<u64>()];
                match stream.receive_all_timeout(&mut header_bytes, end_time - uf.system_time(), uf) {
                    Ok(result) => if result {
                        let crc_calculated = checksum_ecma(&header_bytes[0..length]);
                        let mut crc_received_bytes = [0; size_of::<u64>()];
                        crc_received_bytes.copy_from_slice(&header_bytes[length..]);
                        let crc_received = u64::from_be_bytes(crc_received_bytes);
                        if crc_received != crc_calculated {
                            uf.log_debug(|| format!("Checksum mismatch, received: {}, calculated: {}", crc_received, crc_calculated));
                            return None;
                        }
                        match serde_cbor::from_slice(&header_bytes[0..length]) {
                            Ok(header) => header,
                            Err(error) => {
                                uf.log_error(|| format!("Error deserializing header: {}", error));
                                return None;
                            },
                        }
                    } else {
                        uf.log_debug(|| format!("Timed out receiving header of length {}", length));
                        return None;
                    },
                    Err(error) => {
                        uf.log_error(|| format!("Error receiving packet of length {}: {:?}", length, error));
                        return None;
                    }
                }
            } else {
                uf.log_debug(|| "Timed out receiving header length");
                return None;
            },
            Err(error) => {
                uf.log_error(|| format!("Error receiving header: {:?}", error));
                return None;
            }
        };
        match &header.packet_type {
            ChecksumPacketType::DataPacket { data_length, data_checksum } => {
                let mut data_bytes = vec![0; *data_length as usize];
                match stream.receive_all_timeout(&mut data_bytes, end_time - uf.system_time(), uf) {
                    Ok(got_values) => if got_values {
                        let calculated_crc = checksum_ecma(&data_bytes);
                        if calculated_crc != *data_checksum {
                            uf.log_debug(|| format!("Checksum mismatch for data, calculated: {}, received: {}", calculated_crc, data_checksum));
                            None
                        } else {
                            Some((header, Some(data_bytes)))
                        }
                    } else {
                        uf.log_debug(|| format!("Timeout recieving packet data"));
                        None
                    }
                    Err(error) => {
                        uf.log_error(|| format!("Error receiving packet data: {:?}", error));
                        None
                    }
                }
            }
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
enum ChecksumPacketType {
    DataPacket{ data_length: u64, data_checksum: u64 },
}
