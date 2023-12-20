use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Referral {
    pub user_id: i64,
    pub referral_code: Option<String>,
    pub created_at: i64,
    pub referred_by: Option<i64>,
    pub referred_at: Option<i64>,
}

impl Referral {
    pub fn new(
        user_id: i64,
        referral_code: Option<String>,
        created_at: i64,
        referred_by: Option<i64>,
        referred_at: Option<i64>,
    ) -> Self {
        Self {
            referral_code: referral_code,
            user_id: user_id,
            created_at: created_at,
            referred_by: referred_by,
            referred_at: referred_at,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ReferralsOverview {
    pub total_referees: i64,
    // total of txs related to all referees
    pub total_referees_txs: i32,
    // commision earned from referees
    pub total_commision: String,
    // unclaimed commision earned from referees
    pub unclaimed_commission: String,
}
