use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RewardsOverview {
    /// claimed + unclaimed
    pub total_rewards: i64,
    /// unclaimed
    pub unclaimed_rewards: i64,
    /// claimed + unclaimed
    pub total_network_rewards: i64,
    /// claimed + unclaimed
    pub total_task_rewards: i64,
    /// claimed + unclaimed
    pub total_referral_rewards: i64,
    /// claimed + unclaimed
    pub total_commission_rewards: i64,
}
