use serde::{Deserialize, Serialize};
use web3::types::Address;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct UserXp {
    pub user_addr: Address,
    pub minutes_uptime: f64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl UserXp {
    pub fn new(
        user_addr: Address,
        minutes_uptime: f64,
        created_at: i64,
        updated_at: i64,
    ) -> Self {
        Self {
            user_addr,
            minutes_uptime,
            created_at,
            updated_at,
        }
    }
}