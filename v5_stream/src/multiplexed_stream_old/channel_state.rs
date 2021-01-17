use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ChannelState {
    Open,
    StreamsAvailable,
    PartnerEstablishing,
    PartnerNotOpen,
    Closed,
}
