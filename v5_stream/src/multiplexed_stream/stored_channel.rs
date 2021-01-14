use alloc::boxed::Box;
use alloc::vec::Vec;
use core::any::Any;
use core::default::Default;
use core::fmt::Debug;
use core::marker::{PhantomData, Send};
use core::option::Option;
use core::option::Option::None;
use core::sync::atomic::Ordering;

use atomic::Atomic;

use v5_traits::error::Error;
use v5_traits::stream::{DuplexStream, MessageStreamCreator, ReceiveStream, SendStream};
use v5_traits::sync::{Mutex, SyncCell};
use v5_traits::UniversalFunctions;

use crate::multiplexed_stream::{ChannelIdType, MultiplexStream, TypeIdType};
use crate::multiplexed_stream::channel_state::ChannelState;
use crate::multiplexed_stream::error::MultiplexError::{ChannelAlreadyOpened, WrongTypeForChannel};
use crate::multiplexed_stream::error::MultiplexError;
use crate::multiplexed_stream::management_packet::ManagementPacket;
use crate::multiplexed_stream::multiplex_packet::MultiplexPacket;
use crate::multiplexed_stream::stored_channel::ClientError::DowncastError;
use crate::multiplexed_stream::identifiable::Identifiable;

pub struct StoredChannel<M, C>
    where M: Mutex<Option<(C::Sender, C::Receiver)>>,
          C: MessageStreamCreator<Box<dyn Any + Send>> + Send + Sync{
    channel_state: Atomic<ChannelState>,
    type_id: Atomic<TypeIdType>,
    own_streams: M,
    partner_streams: SyncCell<(C::Sender, C::Receiver)>,
}
impl<M, C> StoredChannel<M, C>
    where M: Mutex<Option<(C::Sender, C::Receiver)>>,
          C: MessageStreamCreator<Box<dyn Any + Send>> + Send + Sync{
    /// Does not create streams, only changes state to open
    pub fn make_open(&self, type_id: TypeIdType){
        self.channel_state.store(ChannelState::Open, Ordering::SeqCst);
        self.type_id.store(type_id, Ordering::SeqCst);
    }
    pub fn open_connection<U, S, T>(&self, multiplex_stream: &MultiplexStream<U, S, M, C>, channel_id: ChannelIdType) -> Result<(impl SendStream<Box<T>> + Send + Sync, impl ReceiveStream<Box<T>> + Send + Sync), MultiplexError<S>>
        where U: UniversalFunctions,
              S: DuplexStream<MultiplexPacket> + Send + Sync,
              T: Identifiable{
        if let Ok(_) = self.channel_state.compare_exchange(ChannelState::StreamsAvailable, ChannelState::Open, Ordering::SeqCst, Ordering::SeqCst){
            let type_id = self.type_id.load(Ordering::SeqCst);
            if type_id != T::ID{
                return Err(WrongTypeForChannel { channel_type_id: type_id, packet_type_id: T::ID });
            }
            let streams = self.partner_streams.swap(None);
            match streams{
                None => Err(MultiplexError::NoPartnerStreams(channel_id)),
                Some(streams) => {
                    multiplex_stream.send_management_packet(&ManagementPacket::ChannelCreated(channel_id))?;
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
            multiplex_stream.send_management_packet(&ManagementPacket::CreateChannel(channel_id))?;
            Ok((ClientSender::new(out_streams.0), ClientReceiver::new(out_streams.1)))
        }
        else{
            Err(ChannelAlreadyOpened { channel_id, type_id: T::ID })
        }
    }

    pub fn channel_state(&self) -> ChannelState{
        self.channel_state.load(Ordering::SeqCst)
    }
    pub fn type_id(&self) -> TypeIdType{
        self.type_id.load(Ordering::SeqCst)
    }
}
impl<M, C> Default for StoredChannel<M, C>
    where M: Mutex<Option<(C::Sender, C::Receiver)>>,
          C: MessageStreamCreator<Box<dyn Any + Send>> + Send + Sync{
    fn default() -> Self {
        Self{
            channel_state: Atomic::new(ChannelState::Closed),
            type_id: Atomic::new(0),
            own_streams: M::new(None),
            partner_streams: Default::default()
        }
    }
}

#[derive(Debug)]
pub struct ClientSender<T, C> where T: Identifiable + Send + Sync, C: MessageStreamCreator<Box<dyn Any + Send>> + Send + Sync{
    sender: C::Sender,
    phantom_t: PhantomData<T>,
}
impl<T, C> ClientSender<T, C> where T: Identifiable + Send + Sync, C: MessageStreamCreator<Box<dyn Any + Send>> + Send + Sync{
    pub fn new(sender: C::Sender) -> Self{
        Self{ sender, phantom_t: Default::default() }
    }
}
impl<T, C> SendStream<Box<T>> for ClientSender<T, C> where T: Identifiable + Send + Sync, C: MessageStreamCreator<Box<dyn Any + Send>> + Send + Sync{
    type Error = ClientError<<C::Sender as SendStream<Box<dyn Any + Send>>>::Error>;

    fn try_send(&self, val: Box<T>) -> Result<Result<(), Box<T>>, Self::Error> {
        match self.sender.try_send(val)?{
            Ok(_) => Ok(Ok(())),
            Err(back) => match back.downcast(){
                Ok(val) => Ok(Err(val)),
                Err(error) => Err(DowncastError(error)),
            }
        }
    }

    fn send(&self, val: Box<T>) -> Result<(), Self::Error> {
        Ok(self.sender.send(val)?)
    }

    fn send_vec(&self, data: Vec<Box<T>>) -> Result<(), Self::Error> {
        let data = data.into_iter().map(|val|val as Box<dyn Any + Send>).collect();
        Ok(self.sender.send_vec(data)?)
    }
}
#[derive(Debug)]
pub struct ClientReceiver<T, C> where T: Identifiable + Send + Sync, C: MessageStreamCreator<Box<dyn Any + Send>> + Send + Sync{
    receiver: C::Receiver,
    phantom_t: PhantomData<T>
}
impl<T, C> ClientReceiver<T, C> where T: Identifiable + Send + Sync, C: MessageStreamCreator<Box<dyn Any + Send>> + Send + Sync{
    pub fn new(receiver: C::Receiver) -> Self{
        Self{ receiver, phantom_t: Default::default() }
    }
}
impl<T, C> ReceiveStream<Box<T>> for ClientReceiver<T, C> where T: Identifiable + Send + Sync, C: MessageStreamCreator<Box<dyn Any + Send>> + Send + Sync{
    type Error = ClientError<<C::Receiver as ReceiveStream<Box<dyn Any + Send>>>::Error>;

    fn try_receive(&self) -> Result<Option<Box<T>>, Self::Error> {
        match self.receiver.try_receive()?{
            None => Ok(None),
            Some(value) => Ok(Some(match value.downcast(){
                Ok(value) => value,
                Err(error) => return Err(DowncastError(error)),
            }))
        }
    }

    fn receive(&self) -> Result<Box<T>, Self::Error> {
        Ok(match self.receiver.receive()?.downcast(){
            Ok(value) => value,
            Err(error) => return Err(DowncastError(error)),
        })
    }

    fn receive_vec(&self, limit: usize) -> Result<Vec<Box<T>>, Self::Error> {
        self.receiver.receive_vec(limit)?.into_iter().map(|val|
            match val.downcast() {
                Ok(value) => Ok(value),
                Err(error) => Err(DowncastError(error)),
            }
        ).collect()
    }
}
#[derive(Debug)]
pub enum ClientError<E: Error>{
    StreamError(E),
    DowncastError(Box<dyn Any + Send>),
}
impl<E: Error> From<E> for ClientError<E>{
    fn from(from: E) -> Self {
        Self::StreamError(from)
    }
}
impl<E: Error> Error for ClientError<E>{}
