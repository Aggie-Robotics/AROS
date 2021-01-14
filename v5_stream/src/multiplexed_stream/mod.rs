use alloc::boxed::Box;
use alloc::format;
use alloc::vec::Vec;
use core::any::Any;
use core::marker::PhantomData;
use core::sync::atomic::Ordering;
use core::time::Duration;
use atomic::Atomic;
use channel_state::ChannelState;
use error::MultiplexError::*;
use error::MultiplexError;
use link_state::LinkState;
use management_packet::ManagementPacket;
use multiplex_packet::MultiplexPacket;
use v5_traits::{EnsureSend, EnsureSync, LogLevel, UniversalFunctions};
use v5_traits::stream::{DuplexStream, MessageStreamCreator, ReceiveStream, SendStream};
use v5_traits::sync::Mutex;

use crate::multiplexed_stream::stored_channel::StoredChannel;
use crate::multiplexed_stream::identifiable::Identifiable;

pub mod channel_state;
pub mod error;
pub mod identifiable;
pub mod link_state;
mod management_packet;
pub mod multiplex_packet;
pub mod stored_channel;

pub type TypeIdType = u8;
pub type ChannelIdType = i32;
pub const MANAGEMENT_CHANNEL: ChannelIdType = -1;
//TODO: Make receive channel creation and receive loop
/// Connection Process:
/// 1. MultiplexStream::send_connection on one device, MultiplexStream::receive_connection on the other
pub struct MultiplexStream<U, S, M, C>
    where U: UniversalFunctions,
          S: DuplexStream<MultiplexPacket> + Send + Sync,
          M: Mutex<Option<(C::Sender, C::Receiver)>>,
          C: MessageStreamCreator<Box<dyn Any + Send>> + Send + Sync{
    universal_functions: U,
    stream: S,
    channels: Vec<StoredChannel<M, C>>,
    link_state: Atomic<LinkState>,
    stream_creator: C,
    phantom_u: PhantomData<U>,
}
impl<U, S, M, C> MultiplexStream<U, S, M, C>
    where U: UniversalFunctions,
          S: DuplexStream<MultiplexPacket> + Send + Sync,
          M: Mutex<Option<(C::Sender, C::Receiver)>>,
          C: MessageStreamCreator<Box<dyn Any + Send>> + Send + Sync{
    pub fn new(universal_functions: U, stream: S, stream_creator: C, max_channels: ChannelIdType) -> Self{
        universal_functions.log(||format!("MultiplexStream::new({:?}, {:?}, {:?}, {:?})", universal_functions, stream, stream_creator, max_channels), LogLevel::TRACE);
        if max_channels <= 0{
            panic!("CANNOT HAVE {} CHANNELS!", max_channels);
        }
        let mut channels = Vec::with_capacity(max_channels as usize);
        for _ in 0..max_channels{
            channels.push(Default::default());
        }
        Self{
            universal_functions,
            stream,
            channels,
            link_state: Atomic::new(LinkState::NotConnected),
            stream_creator,
            phantom_u: Default::default(),
        }
    }

    pub fn channels(&self) -> Vec<ChannelState> {
        self.universal_functions.log(||format!("MultiplexStream::channels()"), LogLevel::TRACE);
        self.channels.iter()
            .map(|state|state.channel_state())
            .collect()
    }
    pub fn link_state(&self) -> LinkState{
        self.universal_functions.log(||format!("MultiplexStream::link_state()"), LogLevel::TRACE);
        self.link_state.load(Ordering::SeqCst)
    }

    fn try_send(&self, packet: MultiplexPacket) -> Result<Result<(), MultiplexPacket>, MultiplexError<S>>{
        self.universal_functions.log(||format!("MultiplexStream::try_send({:?})", packet), LogLevel::TRACE);
        match self.stream.try_send(packet) {
            Ok(val) => Ok(val),
            Err(error) => Err(SendStreamError(error)),
        }
    }
    fn send(&self, packet: MultiplexPacket) -> Result<(), MultiplexError<S>>{
        self.universal_functions.log(||format!("MultiplexStream::try_send({:?})", packet), LogLevel::TRACE);
        match self.stream.send(packet){
            Ok(_) => Ok(()),
            Err(error) => Err(SendStreamError(error)),
        }
    }
    fn try_receive(&self) -> Result<Option<MultiplexPacket>, MultiplexError<S>>{
        self.universal_functions.log(||format!("MultiplexStream::try_receive()"), LogLevel::TRACE);
        match self.stream.try_receive() {
            Ok(val) => Ok(val),
            Err(error) => Err(ReceiveStreamError(error)),
        }
    }
    fn receive(&self) -> Result<MultiplexPacket, MultiplexError<S>>{
        self.universal_functions.log(||format!("MultiplexStream::receive()"), LogLevel::TRACE);
        match self.stream.receive(){
            Ok(val) => Ok(val),
            Err(error) => Err(ReceiveStreamError(error)),
        }
    }

    pub fn send_connection(&self) -> Result<(), MultiplexError<S>>{
        self.universal_functions.log(||format!("MultiplexStream::send_connection()"), LogLevel::TRACE);
        const PACKET: ManagementPacket = ManagementPacket::Connect;

        match self.link_state.compare_exchange(LinkState::NotConnected, LinkState::Connecting, Ordering::SeqCst, Ordering::SeqCst){
            Ok(_) => {
                self.universal_functions.log(||format!("Connecting..."), LogLevel::DEBUG);
                self.send_management_packet(&PACKET)?;
                let mut received = self.try_receive_sent_connection()?;
                loop{
                    if received.is_some(){ break; }
                    self.universal_functions.delay(Duration::from_secs(1));
                    received = self.try_receive_sent_connection()?;
                    if received.is_some(){ break; }
                    self.send_management_packet(&PACKET)?;
                    received = self.try_receive_sent_connection()?;
                }
                self.link_state.store(LinkState::Connected, Ordering::SeqCst);
                self.universal_functions.log(||"Connected", LogLevel::DEBUG);
                Ok(())
            }
            Err(prev_state) => {
                self.universal_functions.log(||format!("send_connection called on connection state {:?}", prev_state), LogLevel::ERROR);
                Err(TriedToConnectInWrongLinkState(prev_state))
            }
        }
    }
    fn send_management_packet(&self, packet: &ManagementPacket) -> Result<(), MultiplexError<S>>{
        self.universal_functions.log(||format!("MultiplexStream::send_management_packet({:?})", packet), LogLevel::TRACE);
        self.send(MultiplexPacket{
            channel_id: MANAGEMENT_CHANNEL,
            type_id: ManagementPacket::ID,
            packet_data: serde_cbor::to_vec(&packet)?,
        })
    }
    fn try_receive_sent_connection(&self) -> Result<Option<ManagementPacket>, MultiplexError<S>>{
        self.universal_functions.log(||format!("MultiplexStream::try_receive_sent_connection()"), LogLevel::TRACE);
        match self.try_receive()?{
            None => Ok(None),
            Some(packet) => {
                if packet.channel_id == MANAGEMENT_CHANNEL {
                    if packet.type_id == ManagementPacket::ID {
                        let packet = serde_cbor::from_slice(&packet.packet_data)?;
                        if let ManagementPacket::ConnectionReceived = packet {
                            return Ok(Some(packet));
                        }
                        else{
                            self.universal_functions.log(move||format!("Wrong management packet received on connect: {:?}", packet), LogLevel::ERROR);
                        }
                    }
                    else{
                        self.universal_functions.log(move||format!("Wrong packet type received on connect: {:?}", packet), LogLevel::ERROR);
                    }
                }
                else{
                    self.universal_functions.log(move||format!("Wrong packet channel received on connect: {:?}", packet), LogLevel::ERROR);
                }
                Ok(None)
            }
        }
    }

    //TODO: Log within this
    pub fn receive_connection(&self) -> Result<(), MultiplexError<S>>{
        self.universal_functions.log(||format!("MultiplexStream::receive_connection()"), LogLevel::TRACE);
        match self.link_state.compare_exchange(LinkState::NotConnected, LinkState::Connecting, Ordering::SeqCst, Ordering::SeqCst){
            Ok(_) => {
                self.universal_functions.log(||"Connecting...", LogLevel::DEBUG);
                loop {
                    let packet = self.receive()?;
                    if packet.channel_id == -1 {
                        if packet.type_id == ManagementPacket::ID {
                            let packet = serde_cbor::from_slice(&packet.packet_data)?;
                            if let ManagementPacket::Connect = packet {
                                self.send_management_packet(&ManagementPacket::ConnectionReceived)?;
                                self.universal_functions.log(||"Connected", LogLevel::DEBUG);
                                return Ok(());
                            } else {
                                self.universal_functions.log(move || format!("Wrong management packet received on connect_received: {:?}", packet), LogLevel::ERROR);
                            }
                        }
                        else {
                            self.universal_functions.log(move||format!("Wrong packet type received on connect_received: {:?}", packet), LogLevel::ERROR);
                        }
                    }
                    else{
                        self.universal_functions.log(move||format!("Wrong packet channel received on connect_received: {:?}", packet), LogLevel::ERROR);
                    }
                }
            }
            Err(prev_state) => {
                self.universal_functions.log(||format!("receive_connection called on connection state {:?}", prev_state), LogLevel::ERROR);
                Err(TriedToConnectInWrongLinkState(prev_state))
            }
        }
    }

    fn send_packet<P>(&self, channel_id: ChannelIdType, packet: P ) -> Result<(), MultiplexError<S>> where P: Identifiable{
        self.universal_functions.log(||format!("MultiplexStream::send_packet<P:{}>({:?}, {:?})", P::NAME, channel_id, packet), LogLevel::TRACE);
        let channel_state = match self.channels.get(channel_id as usize){
            None => return Err(ChannelOutOfBounds(channel_id)),
            Some(info) => info,
        };
        let type_id = channel_state.type_id();
        if type_id != P::ID{
            return Err(WrongTypeForChannel { channel_type_id: type_id, packet_type_id: P::ID });
        }
        self.send(MultiplexPacket{ channel_id, type_id, packet_data: serde_cbor::to_vec(&packet)? })
    }
    pub fn open_channel<T>(&self, channel_id: ChannelIdType) -> Result<(impl SendStream<Box<T>> + Send + Sync, impl ReceiveStream<Box<T>> + Send + Sync), MultiplexError<S>> where T: Identifiable{
        self.universal_functions.log(||format!("MultiplexStream::open_channel<T:{}>({:?})", T::NAME, channel_id), LogLevel::TRACE);
        if channel_id < 0 || channel_id as usize >= self.channels.len(){
            return Err(ChannelOutOfBounds(channel_id));
        }
        self.channels[channel_id as usize].open_connection(self, channel_id)
    }
}
impl<U, S, M, C> Drop for MultiplexStream<U, S, M, C>
    where U: UniversalFunctions,
          S: DuplexStream<MultiplexPacket> + Send + Sync,
          M: Mutex<Option<(C::Sender, C::Receiver)>>,
          C: MessageStreamCreator<Box<dyn Any + Send>> + Send + Sync{
    fn drop(&mut self) {
        match self.send_management_packet(&ManagementPacket::BreakConnection){
            Ok(_) => {}
            Err(error) => self.universal_functions.log(move||format!("Error sending management packet on drop! {:?}", error), LogLevel::FATAL),
        }
    }
}
impl<U, S, M, C> EnsureSend for MultiplexStream<U, S, M, C>
    where U: UniversalFunctions,
          S: DuplexStream<MultiplexPacket> + Send + Sync,
          M: Mutex<Option<(C::Sender, C::Receiver)>>,
          C: MessageStreamCreator<Box<dyn Any + Send>> + Send + Sync{}
impl<U, S, M, C> EnsureSync for MultiplexStream<U, S, M, C>
    where U: UniversalFunctions,
          S: DuplexStream<MultiplexPacket> + Send + Sync,
          M: Mutex<Option<(C::Sender, C::Receiver)>>,
          C: MessageStreamCreator<Box<dyn Any + Send>> + Send + Sync{}
