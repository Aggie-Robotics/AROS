use serde::{Serialize, Deserialize};
use alloc::vec::Vec;
use v5_traits::stream::{DuplexStream, SendStream, ReceiveStream, MessageStreamCreator};
use crate::serialize_stream::DuplexSerializeStream;
use core::sync::atomic::{AtomicU8, Ordering, AtomicU64};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use v5_traits::task::TaskRunner;
use alloc::sync::Arc;
use v5_traits::error::Error;
use core::fmt::Debug;

pub type TypeIdType = u8;
/// The id of a type to unify the type system
/// Ids 0-1000 are reserved for system use
pub trait Identifiable<'a>: 'static + Serialize + Deserialize<'a>{
    const ID: TypeIdType;
}

pub type ChannelsList = Vec<AtomicU64>;

/// Connection Process:
/// 1. MultiplexStream::send_connection on one device, MultiplexStream::receive_connection on the other
///
pub struct MultiplexStream<S> where S: DuplexStream<MultiplexPacket> + Send + Sync{
    stream: S,
    channels: ChannelsList,
    link_state: AtomicU8,
}
impl<S> MultiplexStream<DuplexSerializeStream<MultiplexPacket, S>> where S: DuplexStream<u8> + Send + Sync{
    pub fn from_byte_stream(byte_stream: S, max_channels: usize) -> Self{
        Self::new(DuplexSerializeStream::new(byte_stream), max_channels)
    }
}
impl<S> MultiplexStream<S> where S: DuplexStream<MultiplexPacket> + Send + Sync{
    pub fn new(stream: S, max_channels: usize) -> Self{
        let mut channels = Vec::with_capacity(max_channels);
        for _ in 0..max_channels{
            channels.push(AtomicU64::new(EMPTY_CHANNEL));
        }
        channels[0].store(ChannelInfo{ channel_id: 0, type_id: ManagementPacket::ID }.to_u64(), Ordering::SeqCst);
        Self{
            stream,
            channels,
            link_state: AtomicU8::new(LinkState::NotConnected as u8),
        }
    }

    pub fn channels(&self) -> Vec<Option<ChannelInfo>> {
        self.channels.iter()
            .map(|state|ChannelInfo::from_u64(state.load(Ordering::SeqCst)))
            .collect()
    }
    pub fn link_state(&self) -> LinkState{
        FromPrimitive::from_u8(self.link_state.load(Ordering::SeqCst)).expect("Link State had bad value")
    }
    fn set_link_state(&self, state: LinkState){
        self.link_state.store(state as u8, Ordering::SeqCst);
    }

    pub fn send_connection<SC, TR>(self: Arc<Self>, stream_creator: &mut SC, task_runner: &TR) -> Result<(), MultiplexError<S>>
        where SC: MessageStreamCreator<SendConnectionMessage>,
              TR: TaskRunner{
        self.send_management_packet(&ManagementPacket::Connect);
        let received = self.stream.try_receive()?;
        loop{

        }
        Ok(())
    }

    fn send_management_packet(&self, packet: &ManagementPacket) -> Result<(), MultiplexError<S>>{
        let channel_info = match ChannelInfo::from_u64(self.channels[0].load(Ordering::SeqCst)){
            Some(info) => info,
            None => return Err(MultiplexError::ManagementChannelNotOpen),
        };
        self.stream.send(MultiplexPacket{
            channel_info,
            packet_data: serde_cbor::to_vec(&packet)?,
        })
    }

    fn send_packet<'a>(&self, channel: u8, packet: impl Identifiable<'a> ) -> Result<(), MultiplexError<S>>{
        unimplemented!()
    }
    // pub fn open_channel<'a, T>(&self) -> Result<(impl SendStream<T> + Send + Sync, impl ReceiveStream<T> + Send + Sync, ChannelInfo), MultiplexError<<S as SendStream<MultiplexPacket>>::Error>>
    //     where T: Identifiable<'a>{
    //     unimplemented!()
    // }
}
struct SendConnectionArgs<S, SS, RS>
    where S: DuplexStream<MultiplexPacket> + Send + Sync,
          SS: SendStream<SendConnectionMessage>,
          RS: ReceiveStream<SendConnectionMessage>{
    multiplex_stream: Arc<MultiplexStream<S>>,
    message_sender: SS,
    message_receiver: RS,
}
enum SendConnectionMessage{

}

#[derive(Debug, FromPrimitive)]
pub enum LinkState{
    NotConnected,
    Connecting,
    Connected,
    ConnectionBroken,
}

#[derive(Debug)]
pub enum MultiplexError<S> where S: DuplexStream<MultiplexPacket>{
    SendStreamError(<S as SendStream<MultiplexPacket>>::Error),
    ReceiveStreamError(<S as SendStream<MultiplexPacket>>::Error),
    SerdeCborError(serde_cbor::Error),
    ManagementChannelNotOpen,
}
impl<S> Error for MultiplexError<S> where S: DuplexStream<MultiplexPacket> + Debug{}
impl<S> From<serde_cbor::Error> for MultiplexError<S> where S: DuplexStream<MultiplexPacket>{
    fn from(from: serde_cbor::Error) -> Self {
        Self::SerdeCborError(from)
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct ChannelInfo {
    pub channel_id: u32,
    pub type_id: TypeIdType,
}
const CHANNEL_OPEN_FLAG: u64        = 0x80_00_00_00_00_00_00_00;
const CHANNEL_ID_BITS: u64          = 0x00_00_00_00_FF_FF_FF_FF;
const CHANNEL_ID_OFFSET: u8         = 0;
const TYPE_ID_BITS: u64             = 0x00_00_FF_FF_00_00_00_00;
const TYPE_ID_OFFSET: u8            = 32;
const EMPTY_CHANNEL: u64            = 0x00_00_00_00_00_00_00_00;
impl ChannelInfo{
    const fn from_u64(val: u64) -> Option<Self> {
        if val & CHANNEL_OPEN_FLAG == CHANNEL_OPEN_FLAG {
            let channel_id = ((val & CHANNEL_ID_BITS) >> CHANNEL_ID_OFFSET) as u32;
            let type_id = ((val & TYPE_ID_BITS) >> TYPE_ID_OFFSET) as TypeIdType;
            Some(ChannelInfo{ channel_id, type_id })
        }
        else{
            None
        }
    }
    const fn to_u64(self) -> u64{
        CHANNEL_OPEN_FLAG & ((self.channel_id as u64) << CHANNEL_ID_OFFSET) & ((self.type_id as u64) << TYPE_ID_OFFSET)
    }
}

#[derive(Serialize, Deserialize)]
pub struct MultiplexPacket{
    channel_info: ChannelInfo,
    packet_data: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
enum ManagementPacket{
    Connect,
    ConnectionReceived,
    CreateChannel(u64),
    ChannelCreated,
    ChannelAlreadyMade,
    ChannelOutOfBounds,
}
impl<'a> Identifiable<'a> for ManagementPacket{
    const ID: TypeIdType = 0;
}
