use std::{net::IpAddr, time::Duration};

use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum ConnectionEvent {
    /// peer_id
    PeerConnected(String),
    /// peer_id
    PeerDisconnected(String),
    /// peer_id, peer stats
    PeerStats(String, PeerStats),
    /// client_id, assigned_peer_id
    ClientProcessed(String, String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerStats {
    pub peer_id: String,
    pub client_id: String,
    pub download: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct ProxyAccData {
    pub client_id: String,
    pub username: String,
    pub password: String,
    pub ip_rotation_period: Duration,
    pub whitelist_ip_list: Vec<IpAddr>,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct VerifyProxyAccData {
    pub username: String,
    pub password: String,
}
