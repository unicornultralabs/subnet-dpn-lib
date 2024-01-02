use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use web3::types::U256;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserBandwidthPrice {
    pub user_id: i64,
    pub rate_per_kb: i64,
    pub rate_per_second: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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

#[derive(Debug, Clone, FromPrimitive, Serialize, Deserialize, ToSchema)]
pub enum SessionStatus {
    Active,
    Finished,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Session {
    pub id: i64,
    pub provider_id: i64,
    pub client_id: i64,
    pub rate_per_second: U256,
    pub rate_per_kb: U256,
    pub handshake_at: Option<i64>,
    pub end_at: Option<i64>,
    pub duration: Option<i64>,
    pub bandwidth_usage: Option<i64>,
    pub duration_fee: U256,
    pub bandwidth_fee: U256,
    pub total_fee: U256,
    pub status: SessionStatus,
    pub tx_id: Option<i64>,
}

impl Session {
    pub fn new(
        id: i64,
        provider_id: i64,
        client_id: i64,
        rate_per_second: U256,
        rate_per_kb: U256,
        handshake_at: Option<i64>,
        end_at: Option<i64>,
        duration: Option<i64>,
        bandwidth_usage: Option<i64>,
        duration_fee: U256,
        bandwidth_fee: U256,
        total_fee: U256,
        status: SessionStatus,
        tx_id: Option<i64>,
    ) -> Self {
        Self {
            id: id,
            provider_id: provider_id,
            client_id: client_id,
            rate_per_second: rate_per_second,
            rate_per_kb: rate_per_kb,
            handshake_at: handshake_at,
            end_at: end_at,
            duration: duration,
            bandwidth_usage: bandwidth_usage,
            duration_fee: duration_fee,
            bandwidth_fee: bandwidth_fee,
            total_fee: total_fee,
            status: status,
            tx_id: tx_id,
        }
    }
}
