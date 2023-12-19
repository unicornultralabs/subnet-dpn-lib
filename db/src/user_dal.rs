use crate::{
    model::storage_rewards_overview::StorageRewardsOverview,
    utils::{bytes_to_hex_string, szabo_to_u256, u256_to_szabo},
};
use anyhow::Context as _;
use dpn_types::{
    internal_tx::InternalTxType,
    rewards_overview::RewardsOverview,
    tx::{Tx, TxStatus},
};
use sqlx::types::chrono::Utc;
use web3::types::{Address, U256};

use crate::{
    model::{storage_tx::StorageTx, storage_user::StorageUser},
    StorageProcessor,
};

#[derive(Debug)]
pub struct UserDal<'a, 'c> {
    pub(crate) storage: &'a mut StorageProcessor<'c>,
}

impl UserDal<'_, '_> {
    pub async fn get_user_by_deposit_addr(
        &mut self,
        address: Address,
    ) -> anyhow::Result<Option<StorageUser>> {
        let record = sqlx::query!(
            r#"
                SELECT * 
                FROM users 
                WHERE deposit_addr = $1
                LIMIT 1
            "#,
            address.to_string()
        )
        .fetch_optional(self.storage.conn())
        .await?;

        Ok(record.map(|rec| StorageUser {
            id: rec.id,
            fingerprint: rec.fingerprint,
            pincode: rec.pincode,
            username: rec.username,
            deposit_addr: rec.deposit_addr,
            withdrawal_addr: rec.withdrawal_addr,
            balance: rec.balance,
            created_at: rec.created_at,
            last_login: rec.last_login,
        }))
    }

    pub async fn get_user_by_id(&mut self, user_id: i64) -> anyhow::Result<Option<StorageUser>> {
        let record = sqlx::query!(
            r#"
                SELECT * 
                FROM users 
                WHERE id = $1
                LIMIT 1
            "#,
            user_id
        )
        .fetch_optional(self.storage.conn())
        .await?;

        Ok(record.map(|rec| StorageUser {
            id: rec.id,
            fingerprint: rec.fingerprint,
            pincode: rec.pincode,
            username: rec.username,
            deposit_addr: rec.deposit_addr,
            withdrawal_addr: rec.withdrawal_addr,
            balance: rec.balance,
            created_at: rec.created_at,
            last_login: rec.last_login,
        }))
    }

    pub async fn get_user_by_email(&mut self, username: String) -> anyhow::Result<Option<StorageUser>> {
        let record = sqlx::query!(
            r#"
                SELECT * 
                FROM users 
                WHERE username = $1
                LIMIT 1
            "#,
            username
        )
        .fetch_optional(self.storage.conn())
        .await?;

        Ok(record.map(|rec| StorageUser {
            id: rec.id,
            fingerprint: rec.fingerprint,
            pincode: rec.pincode,
            username: rec.username,
            deposit_addr: rec.deposit_addr,
            withdrawal_addr: rec.withdrawal_addr,
            balance: rec.balance,
            created_at: rec.created_at,
            last_login: rec.last_login,
        }))
    }

    pub async fn create_user(&mut self, usr: StorageUser) -> anyhow::Result<StorageUser> {
        let rec = sqlx::query!(
            r#"
                INSERT INTO users (
                    fingerprint,
                    pincode,
                    username,
                    deposit_addr,
                    withdrawal_addr,
                    balance,
                    created_at,
                    last_login
                ) VALUES (
                    $1,
                    $2,
                    $3,
                    $4,
                    $5,
                    $6,
                    $7,
                    $8
                )
                ON CONFLICT (deposit_addr) DO UPDATE SET deposit_addr = $4
                RETURNING *
            "#,
            usr.fingerprint,
            usr.pincode,
            usr.username,
            usr.deposit_addr,
            usr.withdrawal_addr,
            usr.balance,
            usr.created_at,
            usr.last_login
        )
        .fetch_one(self.storage.conn())
        .await?;

        Ok(StorageUser {
            id: rec.id,
            fingerprint: rec.fingerprint,
            pincode: rec.pincode,
            username: rec.username,
            deposit_addr: rec.deposit_addr,
            withdrawal_addr: rec.withdrawal_addr,
            balance: rec.balance,
            created_at: rec.created_at,
            last_login: rec.last_login,
        })
    }

    pub async fn get_withdrawal_amount(&mut self, user_id: i64) -> anyhow::Result<U256> {
        let rec = sqlx::query!(
            r#"
                SELECT *
                FROM users 
                WHERE id = $1
            "#,
            user_id,
        )
        .fetch_one(self.storage.conn())
        .await?;

        Ok(szabo_to_u256(rec.balance))
    }

    /// add balance of given user id. Overflow is unchecked !!!
    /// amount is converted and stored as szabo unit in database
    pub async fn add_balance(&mut self, user_id: i64, amount: U256) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
                UPDATE users 
                SET balance = balance + $1 
                WHERE id = $2
            "#,
            u256_to_szabo(amount),
            user_id,
        )
        .execute(self.storage.conn())
        .await?;

        Ok(())
    }

    /// subtract balance of given user id. Overflow is unchecked !!!
    /// amount is converted and stored as szabo unit in database
    pub async fn sub_balance(&mut self, user_id: i64, amount: U256) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
                UPDATE users 
                SET balance = balance - $1 
                WHERE id = $2;
            "#,
            u256_to_szabo(amount),
            user_id,
        )
        .execute(self.storage.conn())
        .await?;

        Ok(())
    }

    pub async fn update_on_deposit(&mut self, tx: Tx) -> anyhow::Result<()> {
        let mut transaction = self
            .storage
            .start_transaction()
            .await
            .context("start_transaction()")?;
        let storage_tx: StorageTx = tx.into();

        sqlx::query!(
            r#"
                INSERT INTO users (
                    deposit_addr,
                    created_at,
                    last_login
                ) VALUES (
                    $1,
                    $2,
                    $3
                )
                ON CONFLICT (deposit_addr) DO NOTHING;
            "#,
            storage_tx.to_addr,
            Utc::now().timestamp(),
            Utc::now().timestamp()
        )
        .execute(transaction.conn())
        .await?;

        sqlx::query!(
            r#"
                UPDATE users 
                SET balance = balance + $1 
                WHERE id = (
                    SELECT id 
                    FROM users 
                    WHERE deposit_addr = $2
                    LIMIT 1
                    FOR UPDATE
                );
            "#,
            storage_tx.amount,
            storage_tx.to_addr
        )
        .execute(transaction.conn())
        .await?;

        sqlx::query!(
            r#"
                UPDATE transactions
                SET tx_status = $1
                WHERE tx_hash = $2
            "#,
            TxStatus::Success as i32,
            storage_tx.tx_hash,
        )
        .execute(transaction.conn())
        .await?;

        transaction.commit().await.context("commit()")?;
        Ok(())
    }

    pub async fn update_withdrawal_addr_of(
        &mut self,
        user_id: i64,
        withdrawal_addr: Address,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
                UPDATE users 
                SET withdrawal_addr = $1
                WHERE id = $2;
            "#,
            bytes_to_hex_string(withdrawal_addr.as_bytes()),
            user_id,
        )
        .execute(self.storage.conn())
        .await?;

        Ok(())
    }

    pub async fn get_rewards_overview_of(
        &mut self,
        user_id: i64,
    ) -> anyhow::Result<RewardsOverview> {
        let rec = sqlx::query!(
            r#" 
                SELECT
                    (SELECT sum(amount) FROM internal_transactions
                    WHERE to_user_id = $1 AND tx_status = $2) AS total_rewards,
                    (SELECT sum(amount) FROM internal_transactions
                    WHERE to_user_id = $1 AND tx_status = $2 AND rewarded = FALSE ) AS unclaimed_rewards,
                    (SELECT sum(amount) FROM internal_transactions
                    WHERE to_user_id = $1 AND tx_status = $2 AND tx_type = $3) AS total_network_rewards,
                    (SELECT sum(amount) FROM internal_transactions
                    WHERE to_user_id = $1 AND tx_status = $2 AND tx_type = $4) AS total_task_rewards,
                    (SELECT sum(amount) FROM internal_transactions
                    WHERE to_user_id = $1 AND tx_status = $2 AND tx_type = $5) AS total_referral_rewards
            "#,
            user_id,
            TxStatus::Success as i32,
            InternalTxType::Network as i32,
            InternalTxType::Task as i32,
            InternalTxType::Referral as i32,
        ).fetch_one(self.storage.conn()).await?;

        Ok(StorageRewardsOverview {
            total_rewards: rec.total_rewards,
            unclaimed_rewards: rec.unclaimed_rewards,
            total_network_rewards: rec.total_network_rewards,
            total_task_rewards: rec.total_task_rewards,
            total_referral_rewards: rec.total_referral_rewards,
        }
        .into())
    }
}
