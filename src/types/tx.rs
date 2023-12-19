use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use web3::types::{Address, H256, U256};

#[derive(Debug, Clone, FromPrimitive, Serialize, Deserialize)]
pub enum TxType {
    Deposit,
    Withdrawal,
}

#[derive(Debug, Clone, FromPrimitive, Serialize, Deserialize)]
pub enum TxStatus {
    Failed,
    Success,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tx {
    pub id: i64,
    pub user_id: Option<i64>,
    pub from_addr: Address,
    pub to_addr: Address,
    pub tx_hash: Option<H256>,
    pub amount: U256,
    pub tx_type: TxType,
    pub tx_status: TxStatus,
    pub created_at: u64,
}

impl Tx {
    pub fn new(
        id: i64,
        user_id: Option<i64>,
        from_addr: Address,
        to_addr: Address,
        tx_hash: Option<H256>,
        amount: U256,
        tx_type: TxType,
        tx_status: TxStatus,
        created_at: u64,
    ) -> Self {
        Self {
            id: id,
            user_id: user_id,
            from_addr: from_addr,
            to_addr: to_addr,
            tx_hash: tx_hash,
            amount: amount,
            tx_type: tx_type,
            tx_status: tx_status,
            created_at: created_at,
        }
    }
}
