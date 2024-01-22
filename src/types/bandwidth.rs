use crate::utils::{bytes_to_hex_string, hash::hash};
use dpn_proto::session::ProtoSession;
use ethers::types::H256;
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use web3::types::{Address, U256};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserBandwidthPrice {
    pub user_addr: String,
    pub rate_per_kb: i64,
    pub rate_per_second: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EphemeralSession {
    pub hash: String,
    pub client_identifier: String,
    pub client_addr: String,
    pub peer_addr: String,
    pub bandwidth_usage: u64,
    pub handshaked_at: i64,
    pub last_active_at: i64,
}

impl EphemeralSession {
    pub fn new(
        client_identifier: String,
        client_addr: String,
        peer_addr: String,
        handshaked_at: i64,
    ) -> Self {
        let mut _self = Self {
            hash: "".to_string(),
            client_identifier,
            client_addr,
            peer_addr,
            bandwidth_usage: 0,
            handshaked_at,
            last_active_at: handshaked_at,
        };

        let proto: ProtoSession = _self.clone().into();
        let binding = ::prost::Message::encode_to_vec(&proto);
        let bz = binding.as_slice();
        let session_hash = hash(bz);

        _self.hash = bytes_to_hex_string(session_hash.as_bytes());
        _self
    }
}

impl Into<ProtoSession> for EphemeralSession {
    fn into(self) -> ProtoSession {
        ProtoSession {
            provider_addr: self.peer_addr,
            client_addr: self.client_addr,
            client_identifier: self.client_identifier,
            handshaked_at: self.handshaked_at,
        }
    }
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

#[derive(Debug, Clone, FromPrimitive, Serialize, Deserialize, ToSchema)]
pub enum SessionTerminationReason {
    ClientInactive,
    PeerDisconnected,
    SystemShutdown,
    ClientLowBalance,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Session {
    pub session_hash: H256,
    pub provider_addr: Address,
    pub client_addr: Address,
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
    pub reason: Option<SessionTerminationReason>,
    pub tx_hash: Option<H256>,
}

impl Session {
    pub fn new(
        session_hash: H256,
        provider_addr: Address,
        client_addr: Address,
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
        reason: Option<SessionTerminationReason>,
        tx_hash: Option<H256>,
    ) -> Self {
        Self {
            session_hash,
            provider_addr,
            client_addr,
            rate_per_second,
            rate_per_kb,
            handshake_at,
            end_at,
            duration,
            bandwidth_usage,
            duration_fee,
            bandwidth_fee,
            total_fee,
            status,
            reason,
            tx_hash,
        }
    }
}
