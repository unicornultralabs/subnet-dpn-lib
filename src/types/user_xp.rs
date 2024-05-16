use serde::{Deserialize, Serialize};
use web3::types::Address;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct UserXp {
    pub user_addr: Address,
    pub up_time: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl UserXp {
    pub fn new(
        user_addr: Address,
        up_time: i64,
        created_at: i64,
        updated_at: i64,
    ) -> Self {
        Self {
            user_addr,
            up_time,
            created_at,
            updated_at,
        }
    }
}