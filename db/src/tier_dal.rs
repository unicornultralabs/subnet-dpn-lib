use anyhow::{Context, Error};

use crate::{model::storage_tier::StorageUserTier, StorageProcessor};

#[derive(Debug)]
pub struct TierDal<'a, 'c> {
    pub(crate) storage: &'a mut StorageProcessor<'c>,
}

impl TierDal<'_, '_> {
    pub async fn create_tier(&mut self, user_id: i64, tier: i32) -> anyhow::Result<()> {
        let mut transaction = self
            .storage
            .start_transaction()
            .await
            .context("start_transaction()")?;

        match sqlx::query!(
            r#"
            INSERT INTO user_tiers (user_id, tier)
            VALUES ($1, $2)
            ON CONFLICT (user_id) DO NOTHING;
            "#,
            user_id,
            tier,
        )
        .execute(transaction.conn())
        .await
        {
            Ok(rs) => match rs.rows_affected() {
                1 => {
                    transaction.commit().await.context("commit()")?;
                    return Ok(());
                }
                _ => {
                    transaction.commit().await.context("rollback()")?;
                    return Err(Error::msg(format!("failed to create tier")));
                }
            },
            Err(_) => {
                transaction.commit().await.context("rollback()")?;
                return Err(Error::msg(format!("failed to create tier")));
            }
        }
    }

    pub async fn get_user_tier_by(&mut self, user_id: i64) -> anyhow::Result<StorageUserTier> {
        match sqlx::query!(
            r#"
                SELECT *
                FROM user_tiers
                WHERE user_id = $1
            "#,
            user_id,
        )
        .fetch_one(self.storage.conn())
        .await
        {
            Ok(rs) => {
                return Ok(StorageUserTier {
                    user_id: rs.user_id,
                    tier: rs.tier,
                })
            }
            Err(_) => {
                return Err(Error::msg(format!("failed to get user tier")));
            }
        }
    }
}
