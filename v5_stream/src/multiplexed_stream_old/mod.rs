use alloc::boxed::Box;
use alloc::format;
use alloc::vec::Vec;
use core::sync::atomic::Ordering;
use core::time::Duration;
use atomic::Atomic;
use channel_state::ChannelState;
use link_state::LinkState;
use management_packet::ManagementPacket;
use multiplex_packet::MultiplexPacket;
use v5_traits::{EnsureSend, EnsureSync, LogLevel, UniversalFunctions};
use v5_traits::stream::{DuplexStream, MessageStreamCreator, ReceiveStream, SendStream};
use v5_traits::sync::Mutex;

use crate::multiplexed_stream::stored_channel::StoredChannel;
use crate::multiplexed_stream::identifiable::Identifiable;
use v5_traits::error::CustomError;
use serde_cbor::Error;

pub mod channel_state;
pub mod client_receiver;
pub mod client_sender;
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
          C: MessageStreamCreator<Vec<u8>> + Send + Sync{
    universal_functions: U,
    stream: S,
    channels: Vec<StoredChannel<M, C>>,
    link_state: Atomic<LinkState>,
    stream_creator: C,
}
impl<U, S, M, C> MultiplexStream<U, S, M, C>
    where U: UniversalFunctions,
          S: DuplexStream<MultiplexPacket> + Send + Sync,
          M: Mutex<Option<(C::Sender, C::Receiver)>>,
          C: MessageStreamCreator<Vec<u8>> + Send + Sync{
    pub fn new(universal_functions: U, stream: S, stream_creator: C, max_channels: ChannelIdType) -> Result<Self, CustomError>{
        universal_functions.log(||format!("MultiplexStream::new({:?}, stream, stream_creator, {:?})", universal_functions, max_channels), LogLevel::TRACE);
        if max_channels <= 0{
            let message = format!("CANNOT HAVE {} CHANNELS!", max_channels);
            universal_functions.log(||message.clone(), LogLevel::FATAL);
            return Err(CustomError::new(false, message));
        }
        let mut channels = Vec::with_capacity(max_channels as usize);
        for index in 0..max_channels{
            channels.push(StoredChannel::new(index));
        }
        Ok(Self{
            universal_functions,
            stream,
            channels,
            link_state: Atomic::new(LinkState::NotConnected),
            stream_creator,
        })
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

    fn send(&self, packet: MultiplexPacket) -> Result<(), CustomError>{
        self.universal_functions.log(||format!("MultiplexStream::try_send({:?})", packet), LogLevel::TRACE);
        match self.stream.send(packet){
            Ok(_) => Ok(()),
            Err(error) => {
                self.universal_functions.log(||format!("Stream error in send: {:?}", error), LogLevel::ERROR);
                Err(CustomError::from_error("MultiplexStream::send", error))
            },
        }
    }
    fn try_receive(&self) -> Result<Option<MultiplexPacket>, CustomError>{
        self.universal_functions.log(||format!("MultiplexStream::try_receive()"), LogLevel::TRACE);
        match self.stream.try_receive() {
            Ok(val) => Ok(val),
            Err(error) => {
                self.universal_functions.log(||format!("Stream error in try_receive: {:?}", error), LogLevel::ERROR);
                Err(CustomError::from_error("MultiplexStream::try_receive", error))
            },
        }
    }
    fn receive(&self) -> Result<MultiplexPacket, CustomError>{
        self.universal_functions.log(||format!("MultiplexStream::receive()"), LogLevel::TRACE);
        match self.stream.receive(){
            Ok(val) => Ok(val),
            Err(error) => {
                self.universal_functions.log(||format!("Stream error in receive: {:?}", error), LogLevel::ERROR);
                Err(CustomError::from_error("MultiplexStream::receive", error))
            },
        }
    }

    pub fn send_connection(&self) -> Result<(), CustomError>{
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
                let message = format!("send_connection called on connection state {:?}", prev_state);
                self.universal_functions.log(||message.clone(), LogLevel::FATAL);
                Err(CustomError::new(false, message))
            }
        }
    }
    fn send_management_packet(&self, packet: &ManagementPacket) -> Result<(), CustomError>{
        self.universal_functions.log(||format!("MultiplexStream::send_management_packet({:?})", packet), LogLevel::TRACE);
        self.send(MultiplexPacket{
            channel_id: MANAGEMENT_CHANNEL,
            type_id: ManagementPacket::ID,
            packet_data: match serde_cbor::to_vec(&packet){
                Ok(packet_data) => packet_data,
                Err(error) => return Err(CustomError::new(true, format!("Serde Cbor Error: {}", error)))
            },
        })
    }
    fn try_receive_sent_connection(&self) -> Result<Option<ManagementPacket>, CustomError>{
        self.universal_functions.log(||format!("MultiplexStream::try_receive_sent_connection()"), LogLevel::TRACE);
        match self.try_receive()?{
            None => Ok(None),
            Some(packet) => {
                if packet.channel_id == MANAGEMENT_CHANNEL {
                    if packet.type_id == ManagementPacket::ID {
                        let packet = match serde_cbor::from_slice(&packet.packet_data){
                            Ok(packet) => packet,
                            Err(error) => return Err(CustomError::new(true, format!("serde_cbor error try_receive_sent_connection {}", error)))
                        };
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
    pub fn receive_connection(&self) -> Result<(), CustomError>{
        self.universal_functions.log(||format!("MultiplexStream::receive_connection()"), LogLevel::TRACE);
        match self.link_state.compare_exchange(LinkState::NotConnected, LinkState::Connecting, Ordering::SeqCst, Ordering::SeqCst){
            Ok(_) => {
                self.universal_functions.log(||"Connecting...", LogLevel::DEBUG);
                loop {
                    let packet = self.receive()?;
                    if packet.channel_id == -1 {
                        if packet.type_id == ManagementPacket::ID {
                            let packet = match serde_cbor::from_slice(&packet.packet_data){
                                Ok(packet) => packet,
                                Err(error) => return Err(CustomError::new(true, format!("serde_cbor error receive_connection {}", error)))
                            };
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
                let message = format!("receive_connection called on connection state {:?}", prev_state)
                self.universal_functions.log(||message, LogLevel::ERROR);
                Err(CustomError::new(true, format!("receive_connection called on connection state {:?}", prev_state)))
            }
        }
    }

    fn send_packet<P>(&self, channel_id: ChannelIdType, packet: P ) -> Result<(), CustomError> where P: Identifiable{
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
    pub fn open_channel<T>(&self, channel_id: ChannelIdType) -> Result<(impl SendStream<Box<T>> + Send + Sync, impl ReceiveStream<Box<T>> + Send + Sync), CustomError> where T: Identifiable{
        self.universal_functions.log(||format!("MultiplexStream::open_channel<T:{}>({:?})", T::NAME, channel_id), LogLevel::TRACE);
        if channel_id < 0 || channel_id as usize >= self.channels.len(){
            return Err(ChannelOutOfBounds(channel_id));
        }
        self.channels[channel_id as usize].open_connection(self)
    }

    pub fn receive_loop(&self, buffer_size: usize){
        match self.stream.receive_vec(buffer_size){
            Ok(received) => {
                for packet in received{
                    if packet.channel_id < 0{
                        if packet.type_id != ManagementPacket::ID{
                            self.universal_functions.log(||format!("Bad management packet received: {:?}", packet), LogLevel::ERROR);
                            continue;
                        }
                        match serde_cbor::from_slice(&packet.packet_data){
                            Ok(packet) => self.handle_management_packet(packet),
                            Err(error) => self.universal_functions.log(||format!("serde_cbor error processing packet({:?})! {}", packet, error), LogLevel::FATAL),
                        }
                    }
                    else{
                        let channel = match self.channels.get(packet.channel_id as usize){
                            None => {
                                self.universal_functions.log(||format!("Channel out of range {}", packet.channel_id), LogLevel::ERROR);
                                continue;
                            }
                            Some(channel) => channel
                        };
                        if channel.type_id() != packet.type_id{
                            self.universal_functions.log(||format!("Packet type id ({}) does not match channel id ({}), packet: {:?}", packet.type_id, channel.type_id(), packet), LogLevel::ERROR);
                            continue;
                        }
                        if let Err(error) = channel.give_packet(packet.packet_data){
                            self.universal_functions.log(||format!("Error giving packet to channel: {}", error))
                        }
                    }
                }
            },
            Err(error) => self.universal_functions.log(||format!("Could not receive packets: {:?}", error), LogLevel::FATAL)

        };
    }
    fn handle_management_packet(&self, packet: ManagementPacket){

    }
}
impl<U, S, M, C> Drop for MultiplexStream<U, S, M, C>
    where U: UniversalFunctions,
          S: DuplexStream<MultiplexPacket> + Send + Sync,
          M: Mutex<Option<(C::Sender, C::Receiver)>>,
          C: MessageStreamCreator<Vec<u8>> + Send + Sync{
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
          C: MessageStreamCreator<Vec<u8>> + Send + Sync{}
impl<U, S, M, C> EnsureSync for MultiplexStream<U, S, M, C>
    where U: UniversalFunctions,
          S: DuplexStream<MultiplexPacket> + Send + Sync,
          M: Mutex<Option<(C::Sender, C::Receiver)>>,
          C: MessageStreamCreator<Vec<u8>> + Send + Sync{}
