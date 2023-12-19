use serde::{Deserialize, Serialize};

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
