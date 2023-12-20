use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RewardsOverview {
    /// claimed + unclaimed
    pub total_rewards: String,
    /// unclaimed
    pub unclaimed_rewards: String,
    /// claimed + unclaimed
    pub total_network_rewards: String,
    /// claimed + unclaimed
    pub total_task_rewards: String,
    /// claimed + unclaimed
    pub total_referral_rewards: String,
}
