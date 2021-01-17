use alloc::boxed::Box;
use alloc::format;
use alloc::vec::Vec;
use core::any::Any;
use core::default::Default;
use core::fmt::Debug;
use core::marker::{PhantomData, Send};
use core::ops::Deref;
use core::option::Option;
use core::option::Option::None;
use core::sync::atomic::Ordering;

use atomic::Atomic;

use v5_traits::{LogLevel, UniversalFunctions};
use v5_traits::error::CustomError;
use v5_traits::stream::{DuplexStream, MessageStreamCreator, ReceiveStream, SendStream};
use v5_traits::sync::{Mutex, SyncCell};

use crate::multiplexed_stream::{ChannelIdType, MultiplexStream, TypeIdType};
use crate::multiplexed_stream::channel_state::ChannelState;
use crate::multiplexed_stream::client_receiver::ClientReceiver;
use crate::multiplexed_stream::client_sender::ClientSender;
use crate::multiplexed_stream::identifiable::Identifiable;
use crate::multiplexed_stream::management_packet::ManagementPacket;
use crate::multiplexed_stream::multiplex_packet::MultiplexPacket;

pub struct StoredChannel<M, C>
    where M: Mutex<Option<(C::Sender, C::Receiver)>>,
          C: MessageStreamCreator<Vec<u8>> + Send + Sync{
    channel_id: ChannelIdType,
    channel_state: Atomic<ChannelState>,
    type_id: Atomic<TypeIdType>,
    own_streams: M,
    partner_streams: SyncCell<(C::Sender, C::Receiver)>,
}
impl<M, C> StoredChannel<M, C>
    where M: Mutex<Option<(C::Sender, C::Receiver)>>,
          C: MessageStreamCreator<Vec<u8>> + Send + Sync{
    pub fn new(channel_id: ChannelIdType) -> Self{
        Self{
            channel_id,
            channel_state: Atomic::new(ChannelState::Closed),
            type_id: Atomic::new(0),
            own_streams: M::new(None),
            partner_streams: Default::default()
        }
    }

    /// Does not create streams, only changes state to open
    pub fn make_open(&self, type_id: TypeIdType){
        self.channel_state.store(ChannelState::Open, Ordering::SeqCst);
        self.type_id.store(type_id, Ordering::SeqCst);
    }
    pub fn open_connection<U, S, T>(&self, multiplex_stream: &MultiplexStream<U, S, M, C>) -> Result<(impl SendStream<T> + Send + Sync, impl ReceiveStream<T> + Send + Sync), CustomError>
        where U: UniversalFunctions,
              S: DuplexStream<MultiplexPacket> + Send + Sync,
              T: Identifiable{
        if let Ok(_) = self.channel_state.compare_exchange(ChannelState::StreamsAvailable, ChannelState::Open, Ordering::SeqCst, Ordering::SeqCst){
            let type_id = self.type_id.load(Ordering::SeqCst);
            if type_id != T::ID{
                let message = format!("Wrong type for channel {}! Expected type: {}, type given: {}:{}", self.channel_id, type_id, T::ID, T::NAME);
                multiplex_stream.universal_functions.log(||message, LogLevel::ERROR);
                return Err(CustomError::new(true, message));
            }
            let streams = self.partner_streams.swap(None);
            match streams{
                None => {
                    let message = format!("No streams available with streams available state for channel {}!", self.channel_id);
                    multiplex_stream.universal_functions.log(||message, LogLevel::FATAL);
                    Err(CustomError::new(false, message))
                },
                Some(streams) => {
                    multiplex_stream.send_management_packet(&ManagementPacket::ChannelCreated(self.channel_id))?;
                    let (send_stream, receiver_stream) = *streams;
                    Ok((ClientSender::<T, C>::new(send_stream), ClientReceiver::<T, C>::new(receiver_stream)))
                }
            }
        }
        else if let Ok(_) = self.channel_state.compare_exchange(ChannelState::Closed, ChannelState::PartnerNotOpen, Ordering::SeqCst, Ordering::SeqCst){
            self.type_id.store(T::ID, Ordering::SeqCst);
            let (self_streams, out_streams) = multiplex_stream.stream_creator.create_bidirectional_stream();
            let mut guard = self.own_streams.lock();
            *guard = Some(self_streams);
            multiplex_stream.send_management_packet(&ManagementPacket::CreateChannel(self.channel_id))?;
            Ok((ClientSender::new(out_streams.0), ClientReceiver::new(out_streams.1)))
        }
        else{
            let message = format!("Channel {} already opened!", self.channel_id);
            multiplex_stream.universal_functions.log(||message, LogLevel::ERROR);
            Err(CustomError::new(true, message))
        }
    }

    pub fn receive_loop(&self, universal_functions: &impl UniversalFunctions) -> Result<Vec<Vec<u8>>, CustomError>{
        let guard = self.own_streams.lock();
        if let Some(streams) = guard.deref(){
            let mut out = Vec::new();
            let mut received = match streams.1.try_receive(){
                Ok(received) => received,
                Err(error) => return Err(CustomError::from_error("StoredChannel::receive_loop", error))
            };
            while let Some(packet) = received{
                out.push(packet);
                received = match streams.1.try_receive(){
                    Ok(received) => received,
                    Err(error) => return Err(CustomError::from_error("StoredChannel::receive_loop", error))
                };
            }
            Ok(out)
        }
        else{
            let message = format!("Channel {} not opened while receive looping", self.channel_id);
            universal_functions.log(||message, LogLevel::ERROR);
            Err(CustomError::new(true, message))
        }
    }
    pub fn give_packet<S>(&self, packets: Vec<u8>, universal_functions: &impl UniversalFunctions) -> Result<(), CustomError> where S: DuplexStream<MultiplexPacket> + Send + Sync{
        let guard = self.own_streams.lock();
        if let Some(streams) = guard.deref(){
            match streams.0.send_whole_vec(packets){
                Ok(_) => Ok(()),
                Err(error) => Err(CustomError::from_error("StoredChannel::give_packet", error))
            }
        }
        else{
            let message = format!("Channel {} not opened while giving packet!", self.channel_id);
            universal_functions.log(||message, LogLevel::ERROR);
            Err(CustomError::new(true, message))
        }
    }

    pub fn channel_state(&self) -> ChannelState{
        self.channel_state.load(Ordering::SeqCst)
    }
    pub fn type_id(&self) -> TypeIdType{
        self.type_id.load(Ordering::SeqCst)
    }
}
