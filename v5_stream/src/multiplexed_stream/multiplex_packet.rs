use crate::multiplexed_stream::{ChannelIdType, TypeIdType};
use serde::{Serialize, Deserialize};
use alloc::vec::Vec;

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiplexPacket{
    pub channel_id: ChannelIdType,
    pub type_id: TypeIdType,
    pub packet_data: Vec<u8>,
}
