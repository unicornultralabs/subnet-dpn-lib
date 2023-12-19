use dpn_types::internal_tx::InternalTx;

use crate::{model::storage_internal_tx::StorageInternalTx, StorageProcessor};

#[derive(Debug)]
pub struct InternalTxDal<'a, 'c> {
    pub storage: &'a mut StorageProcessor<'c>,
}

impl InternalTxDal<'_, '_> {
    pub async fn insert_tx(&mut self, tx: InternalTx) -> anyhow::Result<()> {
        let _tx: StorageInternalTx = tx.into();

        sqlx::query!(
            r#"
            INSERT INTO internal_transactions (
                session_id,
                from_user_id,
                to_user_id,
                amount,
                tx_type,
                tx_status,
                rewarded,
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
        "#,
            _tx.session_id,
            _tx.from_user_id,
            _tx.to_user_id,
            _tx.amount,
            _tx.tx_type,
            _tx.tx_status,
            _tx.rewarded,
            _tx.created_at,
        )
        .execute(self.storage.conn())
        .await?;

        Ok(())
    }
}
