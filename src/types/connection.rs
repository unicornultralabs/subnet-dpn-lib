use dpn_proto::proxy_acc::ProtoProxyAcc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::utils::hash::hash;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum ConnectionEvent {
    PeerConnected(PeernodeInfo),
    /// peer_id
    PeerDisconnected(String),
    /// session_hash, peer stats
    PeerStats(String, PeerStats),
    /// client_identifier, client_addr, peer_addr, handshaked_at timestamp
    ClientProcessed(String, String, String, i64),
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PeernodeInfo {
    pub peer_id: String,
    pub ip_addr: String,
    pub throughput: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerStats {
    pub session_hash: String,
    pub download: u64,
    pub upload: u64,
    pub c_download: u64,
    pub c_upload: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct ProxyAccData {
    pub id: String,
    pub password: String,
    pub ip_rotation_period: i64,
    pub whitelisted_ip: Option<String>,
    pub user_addr: String,
    pub country_geoname_id: i64,
    pub city_geoname_id: i64,
    pub rate_per_kb: i64,
    pub rate_per_second: i64,
    pub created_at: i64,
}

impl ProxyAccData {
    pub fn new(
        password: String,
        ip_rotation_period: i64,
        whitelisted_ip: Option<String>,
        user_addr: String,
        country_geoname_id: i64,
        city_geoname_id: i64,
        rate_per_kb: i64,
        rate_per_second: i64,
        created_at: i64,
    ) -> Self {
        let mut _self = Self {
            id: "".to_string(),
            user_addr,
            password,
            ip_rotation_period,
            whitelisted_ip,
            country_geoname_id,
            city_geoname_id,
            rate_per_kb,
            rate_per_second,
            created_at,
        };

        let proto: ProtoProxyAcc = _self.clone().into();
        let binding = ::prost::Message::encode_to_vec(&proto);
        let bz = binding.as_slice();

        _self.id = hash(bz).to_string();
        _self
    }
}

impl Into<ProtoProxyAcc> for ProxyAccData {
    fn into(self) -> ProtoProxyAcc {
        ProtoProxyAcc {
            user_addr: self.user_addr,
            created_at: self.created_at,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub enum VerifyProxyAccData {
    // ip
    IP(String),
    // username, password
    BasicAuth(String, String),
}
