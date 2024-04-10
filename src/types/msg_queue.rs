use super::{
    bandwidth::{EphemeralSession, SessionTerminationReason},
    connection::PeernodeInfo,
};

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

#[derive(Debug, Clone)]
pub struct PeerConnectedExtra {
    pub peer_addr: String,
    pub login_session_id: String,
    pub info: PeernodeInfo,
}

#[derive(Debug, Clone)]
pub struct PeerDisconnectedExtra {
    pub peer_addr: String,
    pub login_session_id: String,
}

#[derive(Debug, Clone)]
pub struct SessionCreatedExtra {
    pub session: EphemeralSession,
}

#[derive(Debug, Clone)]
pub struct SessionTerminatedExtra {
    pub session: EphemeralSession,
    pub reason: SessionTerminationReason,
}

#[derive(Debug, Clone)]
pub struct DepositExtra {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub tx_hash: String,
}

#[derive(Debug, Clone)]
pub struct WithdrawalExtra {
    pub user_addr: String,
    pub withdrawal_addr: String,
}

#[derive(Debug, Clone)]
pub struct ReferralExtra {
    pub referrer_addr: String,
    pub referee_addr: String,
}
