use serde::{Deserialize, Serialize};

use super::{
    bandwidth::{EphemeralSession, SessionTerminationReason},
    connection::PeernodeInfo,
    internal_tx::InternalTx,
    tx::Tx,
};

pub const DEPOSIT_ROUTING_KEY: &str = "deposit";
pub const WITHDRAWAL_ROUTING_KEY: &str = "withdrawal";
pub const CONNECTION_ROUTING_KEY: &str = "connection";
pub const REFERRAL_ROUTING_KEY: &str = "referral";
pub const SESSION_ROUTING_KEY: &str = "session";
pub const TXS_ROUTING_KEY: &str = "txs";
pub const INTERNAL_TXS_ROUTING_KEY: &str = "internal_txs";

#[derive(Debug, Clone, Serialize, Deserialize)]

pub enum DPNEvent {
    // connection
    PeerConnected(PeerConnectedExtra),
    PeerDisconnected(PeerDisconnectedExtra),

    // session
    SessionCreated(SessionCreatedExtra),
    SessionTerminated(SessionTerminatedExtra),

    // admin
    Deposit(DepositExtra),
    Withdrawal(WithdrawalExtra),
    Referral(ReferralExtra),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerConnectedExtra {
    pub peer_addr: String,
    pub login_session_id: String,
    pub info: PeernodeInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerDisconnectedExtra {
    pub peer_addr: String,
    pub login_session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionCreatedExtra {
    pub session: EphemeralSession,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionTerminatedExtra {
    pub session: EphemeralSession,
    pub reason: SessionTerminationReason,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositExtra {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub tx_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawalExtra {
    pub user_addr: String,
    pub withdrawal_addr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralExtra {
    pub referrer_addr: String,
    pub referee_addr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DPNTx {
    Tx(Tx),
    InternalTx(InternalTx),
}
