use dpn_types::{
    internal_tx::{InternalTx, InternalTxType},
    tx::TxStatus,
};
use num_traits::FromPrimitive;

use crate::utils::{szabo_to_u256, u256_to_szabo};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct StorageInternalTx {
    pub id: i64,
    pub session_id: i64,
    pub from_user_id: i64,
    pub to_user_id: i64,
    /// In Szabo unit. Example: 6868.123456 U2U is converted to 6868123456
    pub amount: i64,
    pub tx_type: i32,
    pub tx_status: i32,
    pub rewarded: bool,
    pub created_at: i64,
}

impl From<StorageInternalTx> for InternalTx {
    fn from(model: StorageInternalTx) -> Self {
        InternalTx {
            id: model.id,
            session_id: model.session_id,
            from_user_id: model.from_user_id,
            to_user_id: model.to_user_id,
            amount: szabo_to_u256(model.amount),
            tx_status: TxStatus::from_i32(model.tx_status).unwrap(),
            tx_type: InternalTxType::from_i32(model.tx_type).unwrap(),
            rewarded: model.rewarded,
            created_at: model.created_at,
        }
    }
}

impl Into<StorageInternalTx> for InternalTx {
    fn into(self) -> StorageInternalTx {
        StorageInternalTx {
            id: self.id,
            session_id: self.session_id,
            from_user_id: self.from_user_id,
            to_user_id: self.to_user_id,
            amount: u256_to_szabo(self.amount.clone()),
            tx_status: self.tx_status as i32,
            tx_type: self.tx_type as i32,
            rewarded: self.rewarded,
            created_at: self.created_at,
        }
    }
}
