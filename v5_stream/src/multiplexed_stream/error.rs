use core::fmt::Debug;

use v5_traits::error::Error;
use v5_traits::stream::{DuplexStream, ReceiveStream, SendStream};

use crate::multiplexed_stream::{ChannelIdType, TypeIdType};
use crate::multiplexed_stream::link_state::LinkState;
use crate::multiplexed_stream::multiplex_packet::MultiplexPacket;

#[derive(Debug)]
pub enum MultiplexError<S> where S: DuplexStream<MultiplexPacket>{
    SendStreamError(<S as SendStream<MultiplexPacket>>::Error),
    ReceiveStreamError(<S as ReceiveStream<MultiplexPacket>>::Error),
    SerdeCborError(serde_cbor::Error),
    ManagementChannelNotOpen,
    TriedToConnectInWrongLinkState(LinkState),
    ChannelNotOpened(ChannelIdType),
    ChannelOutOfBounds(ChannelIdType),
    WrongTypeForChannel{ channel_type_id: TypeIdType, packet_type_id: TypeIdType },
    ChannelAlreadyOpened{ channel_id: ChannelIdType, type_id: TypeIdType },
    NoPartnerStreams(ChannelIdType),
}
impl<S> Error for MultiplexError<S> where S: DuplexStream<MultiplexPacket> + Debug{}
impl<S> From<serde_cbor::Error> for MultiplexError<S> where S: DuplexStream<MultiplexPacket>{
    fn from(from: serde_cbor::Error) -> Self {
        Self::SerdeCborError(from)
    }
}
