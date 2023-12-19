use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasternodeInfo {
    pub region: String,
    pub peer_bind: String,
    pub client_bind: String,
    pub control_bind: String,
    pub web_bind: String,
    pub root_ca: Option<String>,
}
