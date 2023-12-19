use crate::{model::storage_session::StorageSession, StorageProcessor};
use dpn_types::session::SessionStatus;
use sqlx::types::chrono::Utc;

use anyhow::Context as _;

#[derive(Debug)]
pub struct SessionDal<'a, 'c> {
    pub storage: &'a mut StorageProcessor<'c>,
}

impl SessionDal<'_, '_> {
    pub async fn create_session(
        &mut self,
        session_id: i64,
        peer_id: i64,
        client_id: i64,
        rate_per_second: i64,
        rate_per_kb: i64,
    ) -> anyhow::Result<i64> {
        let mut transaction = self
            .storage
            .start_transaction()
            .await
            .context("start_transaction()")?;

        let rec = sqlx::query!(
            r#"
                INSERT INTO sessions (id, provider_id, client_id, status, rate_per_second, rate_per_kb, handshake_at, bandwidth_usage, duration)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                RETURNING id;
                "#,
            session_id,
            peer_id,
            client_id,
            SessionStatus::Active as i32,
            rate_per_second,
            rate_per_kb,
            Utc::now().timestamp(),
            0,
            0,
        )
        .fetch_one(transaction.conn())
        .await?;

        transaction.commit().await.context("commit()")?;
        Ok(rec.id)
    }

    pub async fn modify_session(
        &mut self,
        session_id: i64,
        client_id: i64,
        rate_per_second: Option<i64>,
        rate_per_kb: Option<i64>,
        end_at: Option<i64>,
        duration: Option<i64>,
        bandwidth_usage: Option<i64>,
        duration_fee: Option<i64>,
        bandwidth_fee: Option<i64>,
        total_fee: Option<i64>,
        status: i32,
    ) -> anyhow::Result<StorageSession> {
        let rec = sqlx::query!(
            r#"
                UPDATE sessions 
                SET 
                client_id = $2,
                rate_per_second = $3,
                rate_per_kb = $4,
                end_at = $5,
                duration = $6,
                bandwidth_usage = $7,
                duration_fee = $8,
                bandwidth_fee = $9,
                total_fee = $10,
                status = $11
                WHERE id = $1
                RETURNING 
                    sessions.id,
                    sessions.provider_id,
                    sessions.client_id,
                    sessions.rate_per_second,
                    sessions.rate_per_kb,
                    sessions.handshake_at,
                    sessions.end_at,
                    sessions.duration,
                    sessions.bandwidth_usage,
                    sessions.duration_fee,
                    sessions.bandwidth_fee,
                    sessions.total_fee,
                    sessions.status
            "#,
            session_id,
            client_id,
            rate_per_second,
            rate_per_kb,
            end_at,
            duration,
            bandwidth_usage,
            duration_fee,
            bandwidth_fee,
            total_fee,
            status
        )
        .fetch_one(self.storage.conn())
        .await?;

        Ok(StorageSession {
            id: rec.id,
            provider_id: rec.provider_id,
            client_id: rec.client_id,
            rate_per_second: rec.rate_per_second,
            rate_per_kb: rec.rate_per_kb,
            handshake_at: rec.handshake_at,
            end_at: rec.rate_per_kb,
            duration: rec.rate_per_kb,
            bandwidth_usage: rec.bandwidth_usage,
            duration_fee: rec.duration_fee,
            bandwidth_fee: rec.bandwidth_fee,
            total_fee: rec.total_fee,
            status: rec.status,
        })
    }

    pub async fn update_bandwidth_usage(
        &mut self,
        session_id: i64,
        bandwidth_usage: i64,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
                UPDATE sessions 
                SET bandwidth_usage = bandwidth_usage + $2
                WHERE id = $1;
            "#,
            session_id,
            bandwidth_usage,
        )
        .execute(self.storage.conn())
        .await?;

        Ok(())
    }

    pub async fn get_session_by_id(
        &mut self,
        session_id: i64,
    ) -> anyhow::Result<Option<StorageSession>> {
        let record = sqlx::query!(
            r#"
                SELECT * 
                FROM sessions 
                WHERE id = $1
                LIMIT 1
            "#,
            session_id
        )
        .fetch_optional(self.storage.conn())
        .await?;

        Ok(record.map(|rec| StorageSession {
            id: rec.id,
            provider_id: rec.provider_id,
            client_id: rec.client_id,
            rate_per_second: rec.rate_per_second,
            rate_per_kb: rec.rate_per_kb,
            handshake_at: rec.handshake_at,
            end_at: rec.end_at,
            duration: rec.duration,
            bandwidth_usage: rec.bandwidth_usage,
            duration_fee: rec.duration_fee,
            bandwidth_fee: rec.bandwidth_fee,
            total_fee: rec.total_fee,
            status: rec.status,
        }))
    }

    pub async fn get_active_session_by_provider(
        &mut self,
        provider_id: i64,
    ) -> anyhow::Result<Option<StorageSession>> {
        let record = sqlx::query!(
            r#"
                SELECT * 
                FROM sessions 
                WHERE provider_id = $1 AND status = $2
                LIMIT 1
            "#,
            provider_id,
            SessionStatus::Active as i32
        )
        .fetch_optional(self.storage.conn())
        .await?;

        Ok(record.map(|rec| StorageSession {
            id: rec.id,
            provider_id: rec.provider_id,
            client_id: rec.client_id,
            rate_per_second: rec.rate_per_second,
            rate_per_kb: rec.rate_per_kb,
            handshake_at: rec.handshake_at,
            end_at: rec.end_at,
            duration: rec.duration,
            bandwidth_usage: rec.bandwidth_usage,
            duration_fee: rec.duration_fee,
            bandwidth_fee: rec.bandwidth_fee,
            total_fee: rec.total_fee,
            status: rec.status,
        }))
    }

    pub async fn get_sessions_by_provider(
        &mut self,
        provider_id: i64,
    ) -> anyhow::Result<Vec<StorageSession>> {
        let records = sqlx::query!(
            r#"
                SELECT * 
                FROM sessions 
                WHERE provider_id = $1
            "#,
            provider_id,
        )
        .fetch_all(self.storage.conn())
        .await?;

        Ok(records
            .into_iter()
            .map(|rec| StorageSession {
                id: rec.id,
                provider_id: rec.provider_id,
                client_id: rec.client_id,
                rate_per_second: rec.rate_per_second,
                rate_per_kb: rec.rate_per_kb,
                handshake_at: rec.handshake_at,
                end_at: rec.end_at,
                duration: rec.duration,
                bandwidth_usage: rec.bandwidth_usage,
                duration_fee: rec.duration_fee,
                bandwidth_fee: rec.bandwidth_fee,
                total_fee: rec.total_fee,
                status: rec.status,
            })
            .collect())
    }
}
