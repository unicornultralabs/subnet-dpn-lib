use dpn_types::rewards_overview::RewardsOverview;
use sqlx::types::BigDecimal;

#[derive(Debug, sqlx::FromRow)]
pub struct StorageRewardsOverview {
    /// claimed + unclaimed
    pub total_rewards: Option<BigDecimal>,
    /// unclaimed
    pub unclaimed_rewards: Option<BigDecimal>,
    /// claimed + unclaimed
    pub total_network_rewards: Option<BigDecimal>,
    /// claimed + unclaimed
    pub total_task_rewards: Option<BigDecimal>,
    /// claimed + unclaimed
    pub total_referral_rewards: Option<BigDecimal>,
}

impl From<StorageRewardsOverview> for RewardsOverview {
    fn from(model: StorageRewardsOverview) -> Self {
        RewardsOverview {
            total_rewards: model
                .total_rewards
                .map_or("0.0".to_owned(), |r| r.to_string()),
            unclaimed_rewards: model
                .unclaimed_rewards
                .map_or("0.0".to_owned(), |r| r.to_string()),
            total_network_rewards: model
                .total_network_rewards
                .map_or("0.0".to_owned(), |r| r.to_string()),
            total_task_rewards: model
                .total_task_rewards
                .map_or("0.0".to_owned(), |r| r.to_string()),
            total_referral_rewards: model
                .total_referral_rewards
                .map_or("0.0".to_owned(), |r| r.to_string()),
        }
    }
}
