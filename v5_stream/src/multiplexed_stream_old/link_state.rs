#[derive(Copy, Clone, Debug)]
pub enum LinkState{
    NotConnected,
    Connecting,
    Connected,
    ConnectionBroken,
}
