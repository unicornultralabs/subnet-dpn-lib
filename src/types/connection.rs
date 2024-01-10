use std::{net::IpAddr, time::Duration};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{bandwidth::UserBandwidthPrice, region::UserRegionInfo};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum ConnectionEvent {
    PeerConnected(PeernodeInfo),
    /// peer_id
    PeerDisconnected(String),
    /// peer_id, peer stats
    PeerStats(String, PeerStats),
    /// client_id, assigned_peer_id
    ClientProcessed(String, String),
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PeernodeInfo {
    pub peer_id: String,
    pub ip_addr: String,
    pub throughput: f64,
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


#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct UserSetting
{
    pub proxy_acc_data: ProxyAccData,
    pub user_bandwidth_price: UserBandwidthPrice,
    pub user_region_info: UserRegionInfo,
}
