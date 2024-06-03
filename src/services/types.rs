use serde::{Deserialize, Serialize};

use crate::types::connection::ProxyAccData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PeerChanged {
    Connected(PeerChangedInfo),
    Disconnected(PeerChangedInfo),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerChangedInfo {
    pub uuid: String,
    pub login_session_id: String,
    pub addr_u128: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProxyAccChanged {
    Created(ProxyAccData),
    Updated(ProxyAccData),
    Deleted(String), // proxy_acc_id
    RefreshAll(),
}
