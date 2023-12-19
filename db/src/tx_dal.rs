use dpn_types::tx::{Tx, TxStatus, TxType};

use crate::{model::storage_tx::StorageTx, StorageProcessor};

const MAX_TX_ATTEMPTS: i16 = 10;

#[derive(Debug)]
pub struct TxDal<'a, 'c> {
    pub storage: &'a mut StorageProcessor<'c>,
}

impl TxDal<'_, '_> {
    pub async fn insert_tx(&mut self, tx: Tx) -> anyhow::Result<()> {
        let _tx: StorageTx = tx.into();

        sqlx::query!(
            r#"
                INSERT INTO transactions (
                    user_id,
                    from_addr,
                    to_addr,
                    tx_hash,
                    amount,
                    tx_type,
                    tx_status,
                    created_at
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
                ON CONFLICT (tx_hash) DO NOTHING;
            "#,
            _tx.user_id,
            _tx.from_addr,
            _tx.to_addr,
            _tx.tx_hash,
            _tx.amount,
            _tx.tx_type,
            _tx.tx_status,
            _tx.created_at,
        )
        .execute(self.storage.conn())
        .await?;

        Ok(())
    }

    /// Update a tx.
    /// Specify `no_retry` as `true` to skip retrying when it is failed.
    pub async fn update_tx(
        &mut self,
        tx_id: i64,
        user_id: Option<i64>,
        tx_hash: Option<String>,
        amount: i64,
        tx_status: i32,
        no_retry: bool,
    ) -> anyhow::Result<StorageTx> {
        let rec = sqlx::query!(
            r#"
                UPDATE transactions 
                SET user_id = $2, tx_hash = $3, amount = $4, tx_status = $5, no_retry = $6
                WHERE id = $1
                RETURNING 
                    transactions.id,
                    transactions.user_id,
                    transactions.from_addr,
                    transactions.to_addr,
                    transactions.tx_hash,
                    transactions.amount,
                    transactions.tx_type,
                    transactions.tx_status,
                    transactions.attempts,
                    transactions.created_at
            "#,
            tx_id,
            user_id,
            tx_hash,
            amount,
            tx_status as i32,
            no_retry
        )
        .fetch_one(self.storage.conn())
        .await?;

        Ok(StorageTx {
            id: rec.id,
            user_id: rec.user_id,
            from_addr: rec.from_addr,
            to_addr: rec.to_addr,
            tx_hash: rec.tx_hash,
            amount: rec.amount,
            tx_type: rec.tx_type,
            tx_status: rec.tx_status,
            attempts: rec.attempts,
            created_at: rec.created_at,
        })
    }

    pub async fn get_next_pending_tx(&mut self) -> anyhow::Result<Option<StorageTx>> {
        let rec = sqlx::query!(
            r#"
                UPDATE transactions
                SET attempts = attempts + 1
                WHERE id = (
                    SELECT id
                    FROM transactions 
                    WHERE tx_status = $1 OR 
                        (tx_status = $2 AND attempts <= $3 AND no_retry = false)
                    ORDER BY id ASC
                    LIMIT 1
                )
                RETURNING 
                    transactions.id,
                    transactions.user_id,
                    transactions.from_addr,
                    transactions.to_addr,
                    transactions.tx_hash,
                    transactions.amount,
                    transactions.tx_type,
                    transactions.tx_status,
                    transactions.attempts,
                    transactions.created_at
            "#,
            TxStatus::Pending as i32,
            TxStatus::Failed as i32,
            MAX_TX_ATTEMPTS
        )
        .fetch_optional(self.storage.conn())
        .await?;

        Ok(rec.map(|rec| StorageTx {
            id: rec.id,
            user_id: rec.user_id,
            from_addr: rec.from_addr,
            to_addr: rec.to_addr,
            tx_hash: rec.tx_hash,
            amount: rec.amount,
            tx_type: rec.tx_type,
            tx_status: rec.tx_status,
            attempts: rec.attempts,
            created_at: rec.created_at,
        }))
    }

    pub async fn get_withdrawal_txs_of(&mut self, user_id: i64) -> anyhow::Result<Vec<StorageTx>> {
        let rec = sqlx::query!(
            r#"
                SELECT * 
                FROM transactions
                WHERE user_id = $1 AND tx_type = $2
                ORDER BY id DESC
            "#,
            user_id,
            TxType::Withdrawal as i32,
        )
        .fetch_all(self.storage.conn())
        .await?;

        Ok(rec
            .iter()
            .map(|t| StorageTx {
                id: t.id,
                user_id: t.user_id,
                from_addr: t.from_addr.clone(),
                to_addr: t.to_addr.clone(),
                tx_hash: t.tx_hash.clone(),
                amount: t.amount,
                tx_type: t.tx_type,
                tx_status: t.tx_status,
                attempts: t.attempts,
                created_at: t.created_at,
            })
            .collect())
    }

    pub async fn get_total_pending_withdrawal_txs_of(
        &mut self,
        user_id: i64,
    ) -> anyhow::Result<Option<i64>> {
        let rec = sqlx::query!(
            r#"
                SELECT COUNT(*) as total_pending_withdrawals
                FROM transactions
                WHERE user_id = $1 AND tx_type = $2 AND tx_status = $3
            "#,
            user_id,
            TxType::Withdrawal as i32,
            TxStatus::Pending as i32,
        )
        .fetch_one(self.storage.conn())
        .await?;

        Ok(rec.total_pending_withdrawals)
    }
}
