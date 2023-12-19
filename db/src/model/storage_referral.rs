use dpn_types::referral::Referral;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct StorageReferral {
    pub user_id: i64,
    pub referral_code: Option<String>,
    pub created_at: i64,
    pub referred_by: Option<i64>,
    pub referred_at: Option<i64>,
}


impl From<StorageReferral> for Referral {
    fn from(model: StorageReferral) -> Self {
        Referral {
            referral_code: model.referral_code,
            user_id: model.user_id,
            created_at: model.created_at,
            referred_by: model.referred_by,
            referred_at: model.referred_at,
        }
    }
}

impl Into<StorageReferral> for Referral {
    fn into(self) -> StorageReferral {
        StorageReferral {
            referral_code: self.referral_code,
            user_id: self.user_id,
            created_at: self.created_at,
            referred_by: self.referred_by,
            referred_at: self.referred_at,
        }
    }
}
