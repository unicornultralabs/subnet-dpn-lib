use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTier {
    pub user_id: i64,
    pub tier: Tier,
}

#[derive(Debug, Clone, FromPrimitive, Serialize, Deserialize)]
pub enum Tier {
    Silver,
    Gold,
    Platinum,
    Diamond,
}
