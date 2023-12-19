use dpn_types::tx::{Tx, TxStatus, TxType};
use num_traits::FromPrimitive;

use crate::utils::{bytes_to_hex_string, szabo_to_u256, u256_to_szabo};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct StorageTx {
    pub id: i64,
    pub user_id: Option<i64>,
    pub from_addr: String,
    pub to_addr: String,
    pub tx_hash: Option<String>,
    /// In Szabo unit. Example: 6868.123456 U2U is converted to 6868123456
    pub amount: i64,
    pub tx_type: i32,
    pub tx_status: i32,
    pub attempts: i16,
    pub created_at: i64,
}

impl From<StorageTx> for Tx {
    fn from(model: StorageTx) -> Self {
        Tx {
            id: model.id,
            user_id: model.user_id,
            from_addr: model.from_addr.parse().unwrap(),
            to_addr: model.to_addr.parse().unwrap(),
            tx_hash: model.tx_hash.map(|txh| txh.parse().unwrap()),
            amount: szabo_to_u256(model.amount),
            tx_type: TxType::from_i32(model.tx_type).unwrap(),
            tx_status: TxStatus::from_i32(model.tx_status).unwrap(),
            created_at: model.created_at as u64,
        }
    }
}

impl Into<StorageTx> for Tx {
    fn into(self) -> StorageTx {
        StorageTx {
            id: self.id,
            user_id: self.user_id,
            from_addr: bytes_to_hex_string(self.from_addr.as_bytes()),
            to_addr: bytes_to_hex_string(self.to_addr.as_bytes()),
            tx_hash: self.tx_hash.map(|x| bytes_to_hex_string(x.as_bytes())),
            amount: u256_to_szabo(self.amount.clone()),
            tx_type: self.tx_type as i32,
            tx_status: self.tx_status as i32,
            attempts: 0,
            created_at: self.created_at as i64,
        }
    }
}
