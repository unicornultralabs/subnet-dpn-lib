use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PeerStatus {
    Connected(PeerStatusConnected),
    Disconnected(PeerStatusDisconnected),
    ClientRemoved(PeerStatusClientRemoved),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerStatusConnected {
    pub uuid: String,
    pub login_session_id: String,
    pub ip_u32: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerStatusDisconnected {
    pub uuid: String,
    pub login_session_id: String,
    pub ip_u32: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerStatusClientRemoved {
    pub uuid: String,
    pub login_session_id: String,
    pub ip_u32: u32,
}
