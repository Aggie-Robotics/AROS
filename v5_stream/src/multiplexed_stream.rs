use alloc::boxed::Box;
use alloc::format;
use alloc::vec::Vec;
use core::marker::PhantomData;
use core::time::Duration;

use serde::{Deserialize, Serialize};

use v5_traits::{EnsureSend, EnsureSync, LogLevel, UniversalFunctions};
use v5_traits::error::Error;
use v5_traits::stream::{DuplexStream, MessageStreamCreator, ReceiveStream, SendStream};
use v5_traits::sync::SyncCell;

use crate::composed_stream::ComposedStream;
use crate::identifiable::{Identifiable, IdentifiableIDType};

pub type ChannelIndexType = u64;

pub struct MultiplexedStream<UF, S, C>
    where UF: UniversalFunctions,
          S: DuplexStream<MultiplexPacket>,
          C: MessageStreamCreator<DataPacket>{
    uf: UF,
    stream: S,
    channels: Vec<Channel<C>>
}
impl<UF, S, C> MultiplexedStream<UF, S, C>
    where UF: UniversalFunctions,
          S: DuplexStream<MultiplexPacket>,
          C: MessageStreamCreator<DataPacket>{
    pub fn new(uf: UF, stream: S, creator: &C, num_channels: ChannelIndexType) -> Self{
        let mut channels = Vec::with_capacity(num_channels as usize);
        for _ in 0..num_channels{
            channels.push(Channel::new(creator));
        }
        Self{ uf, stream, channels }
    }

    pub fn stream(&self) -> &S where S: Sync{
        &self.stream
    }

    pub fn num_channels(&self) -> ChannelIndexType{
        self.channels.len() as ChannelIndexType
    }

    pub fn get_channel<T>(&self, index: ChannelIndexType) -> Option<ClientStream<T, C>> where T: Identifiable + Send{
        Some(ClientStream::new(index, self.channels.get(index as usize)?.client_streams.swap(None)?))
    }

    pub fn handle_outbound(&self, delay: Duration){
        loop {
            self.uf.delay(delay);
            'Iterator: for (channel_id, channel) in self.channels.iter().enumerate() {
                while let Some(outbound) = match channel.outbound_receiver.try_receive() {
                    Ok(outbound) => outbound,
                    Err(error) => {
                        self.uf.log(|| format!("Error receiving outbound on channel {}: {:?}", channel_id, error), LogLevel::ERROR);
                        continue 'Iterator;
                    },
                } {
                    if let Err(error) = self.stream.send(MultiplexPacket::Data(outbound)) {
                        self.uf.log(|| format!("Error sending packet on channel {}: {:?}", channel_id, error), LogLevel::ERROR);
                    }
                }
            }
        }
    }
    pub fn handle_inbound(&self){
        'MainLoop: loop{
            let received = match self.stream.receive(){
                Ok(received) => received,
                Err(error) => {
                    self.uf.log(||format!("Error receiving inbound: {:?}", error), LogLevel::ERROR);
                    continue 'MainLoop;
                },
            };
            let data = match received{
                MultiplexPacket::Data(data) => data,
                MultiplexPacket::ChannelNotAvailable { received_channel, num_channels } => {
                    self.uf.log(||format!("Channel {} is not available, number of channels on other side: {}", received_channel, num_channels), LogLevel::ERROR);
                    continue 'MainLoop;
                }
            };
            match self.channels.get(data.channel_id as usize) {
                None => {
                    self.uf.log(||format!("Channel {} was sent to, channels available: {}", data.channel_id, self.num_channels()), LogLevel::ERROR);
                    if let Err(error) = self.stream.send(MultiplexPacket::ChannelNotAvailable { received_channel: data.channel_id, num_channels: self.num_channels() }){
                        self.uf.log(||format!("Could not tell the other side channel unavailable! {:?}", error), LogLevel::ERROR);
                    }
                }
                Some(channel) => {
                    let channel_id = data.channel_id;
                    if let Err(error) = channel.inbound_sender.send(data){
                        self.uf.log_error(||format!("Error sending to inbound sender on channel {}: {:?}", channel_id, error));
                    }
                }
            }
        }
    }
}
impl<UF, S, C> EnsureSend for MultiplexedStream<UF, S, C>
    where UF: UniversalFunctions + Send,
          S: DuplexStream<MultiplexPacket> + Send,
          C: MessageStreamCreator<DataPacket>{}
impl<UF, S, C> EnsureSync for MultiplexedStream<UF, S, C>
    where UF: UniversalFunctions + Sync,
          S: DuplexStream<MultiplexPacket> + Sync,
          C: MessageStreamCreator<DataPacket>{}

#[derive(Debug, Serialize, Deserialize)]
pub enum MultiplexPacket{
    Data(DataPacket),
    ChannelNotAvailable{ received_channel: ChannelIndexType, num_channels: ChannelIndexType },
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DataPacket{
    channel_id: ChannelIndexType,
    type_id: IdentifiableIDType,
    data: Vec<u8>,
}

struct Channel<C> where C: MessageStreamCreator<DataPacket>{
    inbound_sender: C::Sender,
    client_streams: SyncCell<ComposedStream<DataPacket, C::Sender, C::Receiver>>,
    outbound_receiver: C::Receiver,
}
impl<C> Channel<C> where C: MessageStreamCreator<DataPacket>{
    fn new(creator: &C) -> Self{
        let inbound = creator.create_stream();
        let outbound = creator.create_stream();
        Self{
            inbound_sender: inbound.0,
            client_streams: SyncCell::from(ComposedStream::new(outbound.0, inbound.1)),
            outbound_receiver: outbound.1,
        }
    }
}
impl<C> EnsureSend for Channel<C> where C: MessageStreamCreator<DataPacket>{}
impl<C> EnsureSync for Channel<C> where C: MessageStreamCreator<DataPacket>{}

pub struct ClientStream<T, C>
    where T: Identifiable + Send,
          C: MessageStreamCreator<DataPacket>{
    channel_id: ChannelIndexType,
    streams: Box<ComposedStream<DataPacket, C::Sender, C::Receiver>>,
    phantom_t: PhantomData<T>,
}
impl<T, C> ClientStream<T, C>
    where T: Identifiable + Send,
          C: MessageStreamCreator<DataPacket>{
    fn new(channel_id: ChannelIndexType, streams: Box<ComposedStream<DataPacket, C::Sender, C::Receiver>>) -> Self{
        Self{ channel_id, streams, phantom_t: Default::default() }
    }
}
impl<T, C> SendStream<T> for ClientStream<T, C>
    where T: Identifiable + Send,
          C: MessageStreamCreator<DataPacket>{
    type Error = ClientStreamError<<C::Sender as SendStream<DataPacket>>::Error>;

    fn send(&self, val: T) -> Result<(), Self::Error> {
        let data = match serde_cbor::to_vec(&val){
            Ok(data) => data,
            Err(error) => return Err(ClientStreamError::SerdeCborError(error)),
        };
        Ok(self.streams.send(DataPacket{
            channel_id: self.channel_id,
            type_id: T::ID,
            data
        })?)
    }
}
impl<T, C> ReceiveStream<T> for ClientStream<T, C>
    where T: Identifiable + Send,
          C: MessageStreamCreator<DataPacket>{
    type Error = ClientStreamError<<C::Receiver as ReceiveStream<DataPacket>>::Error>;

    fn try_receive(&self) -> Result<Option<T>, Self::Error> {
        self.streams.try_receive()?.map_or(Ok(None), |val|{
            if val.type_id != T::ID{
                Err(ClientStreamError::TypeIdMismatch { received: val.type_id, expected: T::ID })
            }
            else{
                match serde_cbor::from_slice(&val.data){
                    Ok(val) => Ok(Some(val)),
                    Err(error) => Err(ClientStreamError::SerdeCborError(error)),
                }
            }
        })
    }

    fn receive(&self) -> Result<T, Self::Error> {
        let val = self.streams.receive()?;
        if val.type_id != T::ID{
            Err(ClientStreamError::TypeIdMismatch { received: val.type_id, expected: T::ID })
        }
        else{
            match serde_cbor::from_slice(&val.data){
                Ok(val) => Ok(val),
                Err(error) => Err(ClientStreamError::SerdeCborError(error)),
            }
        }
    }
}

#[derive(Debug)]
pub enum ClientStreamError<E> where E: Error{
    StreamError(E),
    SerdeCborError(serde_cbor::Error),
    TypeIdMismatch{ received: IdentifiableIDType, expected: IdentifiableIDType }
}
impl<E> Error for ClientStreamError<E> where E: Error{
    fn is_recoverable(&self) -> bool {
        match self {
            ClientStreamError::StreamError(error) => error.is_recoverable(),
            ClientStreamError::SerdeCborError(_) => true,
            ClientStreamError::TypeIdMismatch { .. } => true,
        }
    }
}
impl<E> From<E> for ClientStreamError<E> where E: Error{
    fn from(from: E) -> Self {
        Self::StreamError(from)
    }
}
