use crate::multiplexed_stream::{ChannelIdType, TypeIdType};
use serde::{Serialize, Deserialize};
use crate::multiplexed_stream::identifiable::Identifiable;

#[derive(Debug, Serialize, Deserialize)]
pub enum ManagementPacket{
    Connect,
    ConnectionReceived,
    BreakConnection,
    BadPacket,
    CreateChannel(ChannelIdType),
    ChannelCreated(ChannelIdType),
    ChannelAlreadyMade(ChannelIdType),
    ChannelOutOfBounds(ChannelIdType),
}
impl Identifiable for ManagementPacket{
    type NameType = &'static str;
    const ID: TypeIdType = 0;
    const NAME: Self::NameType = "ManagementPacket";
}
