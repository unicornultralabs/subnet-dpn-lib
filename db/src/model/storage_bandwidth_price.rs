use dpn_types::bandwidth::UserBandwidthPrice;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct StorageUserBandwidthPrice {
    pub user_id: i64,
    pub rate_per_kb: i64,
    pub rate_per_second: i64,
}

impl From<StorageUserBandwidthPrice> for UserBandwidthPrice {
    fn from(model: StorageUserBandwidthPrice) -> Self {
        UserBandwidthPrice {
            user_id: model.user_id,
            rate_per_kb: model.rate_per_kb,
            rate_per_second: model.rate_per_second,
        }
    }
}

impl Into<StorageUserBandwidthPrice> for UserBandwidthPrice {
    fn into(self) -> StorageUserBandwidthPrice {
        StorageUserBandwidthPrice {
            user_id: self.user_id as i64,
            rate_per_kb: self.rate_per_kb as i64,
            rate_per_second: self.rate_per_second as i64,
        }
    }
}
