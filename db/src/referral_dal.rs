use anyhow::{Context, Error};
use sqlx::types::chrono::Utc;

use crate::{
    model::{
        storage_referral::StorageReferral, storage_referrals_overview::StorageReferralsOverview,
    },
    StorageProcessor,
};

#[derive(Debug)]
pub struct ReferralDal<'a, 'c> {
    pub(crate) storage: &'a mut StorageProcessor<'c>,
}

impl ReferralDal<'_, '_> {
    pub async fn create_empty_referral(&mut self, user_id: i64) -> anyhow::Result<()> {
        _ = sqlx::query!(
            r#"
                INSERT INTO referrals (user_id, created_at)
                VALUES ($1, $2)
                ON CONFLICT (user_id) DO NOTHING;
            "#,
            user_id,
            Utc::now().timestamp(),
        )
        .execute(self.storage.conn())
        .await?;
        Ok(())
    }

    pub async fn create_referral_code(
        &mut self,
        referral_code: String,
        user_id: i64,
    ) -> anyhow::Result<()> {
        let mut transaction = self
            .storage
            .start_transaction()
            .await
            .context("start_transaction()")?;

        _ = sqlx::query!(
            r#"
                INSERT INTO referrals (user_id, created_at)
                VALUES ($1, $2)
                ON CONFLICT (user_id) DO NOTHING;
            "#,
            user_id,
            Utc::now().timestamp(),
        )
        .execute(transaction.conn())
        .await?;

        match sqlx::query!(
            r#"
                UPDATE referrals
                SET referral_code = $2
                WHERE user_id = $1;
            "#,
            user_id,
            referral_code,
        )
        .execute(transaction.conn())
        .await
        {
            Ok(rs) => match rs.rows_affected() {
                1 => {
                    transaction.commit().await.context("commit()")?;
                    return Ok(());
                }
                0 => {
                    transaction.commit().await.context("rollback()")?;
                    return Err(Error::msg(format!("failed to update referral code")));
                }
                _ => {
                    transaction.commit().await.context("rollback()")?;
                    return Err(Error::msg(format!("multiple referrals were updated")));
                }
            },
            Err(_) => {
                transaction.commit().await.context("rollback()")?;
                return Err(Error::msg(format!("failed to update referral code")));
            }
        }
    }

    pub async fn import_referral_code(
        &mut self,
        friend_referral_code: String,
        user_id: i64,
    ) -> anyhow::Result<()> {
        let mut transaction = self
            .storage
            .start_transaction()
            .await
            .context("start_transaction()")?;

        let rec = sqlx::query!(
            r#"
                SELECT user_id
                FROM referrals
                WHERE referral_code = $1
            "#,
            friend_referral_code,
        )
        .fetch_one(transaction.conn())
        .await;
        if let Err(_) = rec {
            return Err(Error::msg(format!("referral code not found")));
        }
        let friend_user_id = rec.unwrap().user_id;

        _ = sqlx::query!(
            r#"
                INSERT INTO referrals (user_id, created_at)
                VALUES ($1, $2)
                ON CONFLICT (user_id) DO NOTHING;
            "#,
            user_id,
            Utc::now().timestamp(),
        )
        .execute(transaction.conn())
        .await?;

        match sqlx::query!(
            r#"
                UPDATE referrals
                SET referred_by = $2, referred_at = $3
                WHERE user_id = $1 AND referred_by IS NULL;
            "#,
            user_id,
            friend_user_id,
            Utc::now().timestamp(),
        )
        .execute(transaction.conn())
        .await
        {
            Ok(rs) => match rs.rows_affected() {
                1 => {
                    transaction.commit().await.context("commit()")?;
                    return Ok(());
                }
                0 => {
                    transaction.commit().await.context("rollback()")?;
                    return Err(Error::msg(format!("failed to link referral")));
                }
                _ => {
                    transaction.commit().await.context("rollback()")?;
                    return Err(Error::msg(format!("multiple referrals were updated")));
                }
            },
            Err(_) => {
                transaction.commit().await.context("rollback()")?;
                return Err(Error::msg(format!("failed to link referral")));
            }
        }
    }

    pub async fn get_referral_of(&mut self, user_id: i64) -> anyhow::Result<StorageReferral> {
        let record = sqlx::query!(
            r#"
                SELECT * 
                FROM referrals 
                WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(self.storage.conn())
        .await?;

        match record {
            Some(rec) => Ok(StorageReferral {
                referral_code: rec.referral_code,
                user_id: rec.user_id,
                created_at: rec.created_at,
                referred_by: rec.referred_by,
                referred_at: rec.referred_at,
            }),
            None => Err(anyhow::anyhow!("Referral not found")),
        }
    }

    pub async fn get_referrals_by_id(
        &mut self,
        user_id: i64,
    ) -> anyhow::Result<Vec<StorageReferral>> {
        let records = sqlx::query!(
            r#"
                SELECT * 
                FROM referrals 
                WHERE referred_by = $1
                ORDER BY referred_at DESC
            "#,
            user_id
        )
        .fetch_all(self.storage.conn())
        .await?;

        let mut result = Vec::new();

        for rec in records {
            result.push(StorageReferral {
                referral_code: rec.referral_code,
                user_id: rec.user_id,
                created_at: rec.created_at,
                referred_by: rec.referred_by,
                referred_at: rec.referred_at,
            });
        }

        Ok(result)
    }

    pub async fn get_referrals_overview_of(
        &mut self,
        user_id: i64,
    ) -> anyhow::Result<StorageReferralsOverview> {
        let rec = sqlx::query!(
            r#" 
                SELECT
                    (SELECT count(*) FROM referrals 
                        WHERE referred_by = $1) AS total_referees,
                    (SELECT 0) AS total_referees_txs,
                    (SELECT 0.0) AS total_commision,
                    (SELECT 0.0) AS unclaimed_commission
            "#,
            user_id,
        )
        .fetch_one(self.storage.conn())
        .await?;

        Ok(StorageReferralsOverview {
            total_referees: rec.total_referees,
            total_referees_txs: rec.total_referees_txs,
            total_commision: rec.total_commision,
            unclaimed_commission: rec.unclaimed_commission,
        }
        .into())
    }
}
