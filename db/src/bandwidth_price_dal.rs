use anyhow::{Context, Error};

use crate::{model::storage_bandwidth_price::StorageUserBandwidthPrice, StorageProcessor};

#[derive(Debug)]
pub struct BandWidthPriceDal<'a, 'c> {
    pub(crate) storage: &'a mut StorageProcessor<'c>,
}

impl BandWidthPriceDal<'_, '_> {
    pub async fn create_bandwidth_price(&mut self, user_id: i64) -> anyhow::Result<()> {
        let mut transaction = self
            .storage
            .start_transaction()
            .await
            .context("start_transaction()")?;

        match sqlx::query!(
            r#"
                INSERT INTO user_bandwidth_price (user_id, rate_per_kb, rate_per_second)
                VALUES ($1, $2, $3)
                ON CONFLICT (user_id) DO NOTHING;
            "#,
            user_id as i64,
            0,
            0,
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
                    return Err(Error::msg(format!("failed to create bandwidth")));
                }
            },
            Err(_) => {
                transaction.commit().await.context("rollback()")?;
                return Err(Error::msg(format!("failed to create bandwidth")));
            }
        }
    }

    pub async fn update_bandwidth_price(
        &mut self,
        user_id: i64,
        rate_per_kb: i64,
        rate_per_second: i64,
    ) -> anyhow::Result<()> {
        let mut transaction = self
            .storage
            .start_transaction()
            .await
            .context("start_transaction()")?;

        match sqlx::query!(
            r#"
                UPDATE user_bandwidth_price 
                SET rate_per_kb = $2, rate_per_second = $3
                WHERE user_id = $1
            "#,
            user_id,
            rate_per_kb,
            rate_per_second,
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
                    return Err(Error::msg(format!("failed to update bandwidth")));
                }
            },
            Err(_) => {
                transaction.commit().await.context("rollback()")?;
                return Err(Error::msg(format!("failed to update bandwidth")));
            }
        }
    }

    pub async fn get_detail_bandwidth_price(
        &mut self,
        user_id: i64,
    ) -> anyhow::Result<StorageUserBandwidthPrice> {
        let mut transaction = self
            .storage
            .start_transaction()
            .await
            .context("start_transaction()")?;

        match sqlx::query!(
            r#"
                SELECT * FROM user_bandwidth_price
                WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_one(transaction.conn())
        .await
        {
            Ok(rs) => {
                return Ok(StorageUserBandwidthPrice {
                    user_id: rs.user_id,
                    rate_per_kb: rs.rate_per_kb,
                    rate_per_second: rs.rate_per_second,
                })
            }
            Err(_) => {
                return Err(Error::msg(format!("failed to get user bandwidth price")));
            }
        }
    }
}
