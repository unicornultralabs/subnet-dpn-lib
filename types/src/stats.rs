use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stat {
    pub peer_id: u32,
    pub client_id: String,
    pub session_id: u32,
}

impl Stat {
    pub fn new(peer_id: u32, client_id: String, session_id: u32) -> Self {
        Self {
            peer_id: peer_id,
            client_id: client_id,
            session_id: session_id,
        }
    }
}
