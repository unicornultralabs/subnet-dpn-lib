use std::str::FromStr;

use dpn_types::referrals_overview::ReferralsOverview;
use sqlx::types::BigDecimal;

#[derive(Debug, sqlx::FromRow)]
pub struct StorageReferralsOverview {
    pub total_referees: Option<i64>,
    // total of txs related to all referees
    pub total_referees_txs: Option<i32>,
    // commision earned from referees
    pub total_commision: Option<BigDecimal>,
    // unclaimed commision earned from referees
    pub unclaimed_commission: Option<BigDecimal>,
}

impl From<StorageReferralsOverview> for ReferralsOverview {
    fn from(model: StorageReferralsOverview) -> Self {
        ReferralsOverview {
            total_referees: model.total_referees.unwrap_or(0i64),
            total_referees_txs: model.total_referees_txs.unwrap_or(0i32),
            total_commision: model
                .total_commision
                .map_or("0.0".to_owned(), |r| r.to_string()),
            unclaimed_commission: model
                .unclaimed_commission
                .map_or("0.0".to_owned(), |r| r.to_string()),
        }
    }
}

impl Into<StorageReferralsOverview> for ReferralsOverview {
    fn into(self) -> StorageReferralsOverview {
        StorageReferralsOverview {
            total_referees: Some(self.total_referees),
            total_referees_txs: Some(self.total_referees_txs),
            total_commision: Some(BigDecimal::from_str(&self.total_commision).unwrap()),
            unclaimed_commission: Some(BigDecimal::from_str(&self.unclaimed_commission).unwrap()),
        }
    }
}
