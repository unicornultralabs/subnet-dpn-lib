use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use web3::types::U256;
use utoipa::ToSchema;

use super::tx::TxStatus;

#[derive(Debug, Clone, Serialize, Deserialize, FromPrimitive)]
pub enum InternalTxType {
    Network,
    Task,
    Referral,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct InternalTx {
    pub id: i64,
    pub session_id: i64,
    pub from_user_id: i64,
    pub to_user_id: i64,
    pub amount: U256,
    pub tx_type: InternalTxType,
    pub tx_status: TxStatus,
    pub rewarded: bool,
    pub created_at: i64,
}

impl InternalTx {
    pub fn new(
        id: i64,
        session_id: i64,
        from_user_id: i64,
        to_user_id: i64,
        amount: U256,
        tx_type: InternalTxType,
        tx_status: TxStatus,
        rewarded: bool,
        created_at: i64,
    ) -> Self {
        Self {
            id,
            session_id,
            from_user_id,
            to_user_id,
            amount,
            tx_type,
            tx_status,
            rewarded,
            created_at,
        }
    }
}
