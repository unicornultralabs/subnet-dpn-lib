use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PeerStatus {
    Connected(PeerStatusConnected),
    Disconnected(PeerStatusDisconnected),
    ClientRemoved(PeerStatusClientRemoved),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerStatusConnected {
    pub ip_u32: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerStatusDisconnected {
    pub ip_u32: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerStatusClientRemoved {
    pub ip_u32: u32,
}
