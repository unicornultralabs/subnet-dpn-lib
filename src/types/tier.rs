use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserTier {
    pub user_addr: String,
    pub tier: Tier,
}

#[derive(Debug, Clone, FromPrimitive, Serialize, Deserialize, ToSchema)]
pub enum Tier {
    Silver,
    Gold,
    Platinum,
    Diamond,
}
