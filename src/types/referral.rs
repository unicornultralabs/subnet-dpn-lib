use ethers::types::H256;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Referral {
    pub user_id: i64,
    pub referral_code: Option<String>,
    pub created_at: i64,
    pub referred_by: Option<i64>,
    pub referred_at: Option<i64>,
    pub tx_hash: Option<H256>,
}

impl Referral {
    pub fn new(
        user_id: i64,
        referral_code: Option<String>,
        created_at: i64,
        referred_by: Option<i64>,
        referred_at: Option<i64>,
        tx_hash: Option<H256>,
    ) -> Self {
        Self {
            referral_code: referral_code,
            user_id: user_id,
            created_at: created_at,
            referred_by: referred_by,
            referred_at: referred_at,
            tx_hash: tx_hash,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ReferralsOverview {
    pub total_referees: i64,
    // total of txs related to all referees
    pub total_referees_txs: i32,
    // commision earned from referees
    pub total_commision: String,
    // unclaimed commision earned from referees
    pub unclaimed_commission: String,
}
