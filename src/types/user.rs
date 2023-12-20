use serde::{Deserialize, Serialize};
use web3::types::{Address, U256};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub username: Option<String>,
    pub fingerprint: Option<String>,
    pub pincode: Option<String>,
    pub deposit_addr: Address,
    pub withdrawal_addr: Option<Address>,
    pub balance: U256,
    pub created_at: i64,
    pub last_login: i64,
}

impl User {
    pub fn new(
        id: i64,
        email: Option<String>,
        fingerprint: Option<String>,
        pincode: Option<String>,
        deposit_addr: Address,
        withdrawal_addr: Option<Address>,
        created_at: i64,
        last_login: i64,
    ) -> Self {
        Self {
            id: id,
            username: email,
            fingerprint: fingerprint,
            pincode: pincode,
            deposit_addr: deposit_addr,
            withdrawal_addr: withdrawal_addr,
            balance: U256::zero(),
            created_at,
            last_login,
        }
    }
}