use dpn_types::tier::{Tier, UserTier};
use num_traits::FromPrimitive;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct StorageUserTier {
    pub user_id: i64,
    pub tier: i32,
}

impl From<StorageUserTier> for UserTier {
    fn from(model: StorageUserTier) -> Self {
        UserTier {
            user_id: model.user_id,
            tier: Tier::from_i32(model.tier).unwrap(),
        }
    }
}

impl Into<StorageUserTier> for UserTier {
    fn into(self) -> StorageUserTier {
        StorageUserTier {
            user_id: self.user_id,
            tier: self.tier as i32,
        }
    }
}
