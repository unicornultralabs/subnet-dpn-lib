use std::{net::IpAddr, time::Duration};

use dpn_proto::proxy_acc_data::ProtoProxyAccData;
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
    pub client_id: String,
    pub username: String,
    pub password: String,
    pub ip_rotation_period: Duration,
    pub whitelist_ip_list: Vec<IpAddr>,
    pub city_geoname_id: i64,
    pub country_geoname_id: i64,
    pub rate_per_kb: i64,
    pub rate_per_second: i64,
}

impl ProxyAccData {
    pub fn new(
        client_id: String,
        username: String,
        password: String,
        ip_rotation_period: Duration,
        whitelist_ip_list: Vec<IpAddr>,
        city_geoname_id: i64,
        country_geoname_id: i64,
        rate_per_kb: i64,
        rate_per_second: i64,
    ) -> Self {
        let mut _self = Self {
            client_id,
            username,
            password,
            ip_rotation_period,
            whitelist_ip_list,
            city_geoname_id,
            country_geoname_id,
            rate_per_kb,
            rate_per_second,
        };
        
        _self
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct VerifyProxyAccData {
    pub username: String,
    pub password: String,
}
