use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBandwidthPrice {
    pub user_id: i64,
    pub rate_per_kb: i64,
    pub rate_per_second: i64,
}
