use dpn_types::session::{Session, SessionStatus};
use num_traits::FromPrimitive;

use crate::utils::{szabo_to_u256, u256_to_szabo};

#[derive(Debug, sqlx::FromRow)]
pub struct StorageSession {
    pub id: i64,
    pub provider_id: i64,
    pub client_id: i64,
    pub rate_per_second: Option<i64>,
    pub rate_per_kb: Option<i64>,
    pub handshake_at: Option<i64>,
    pub end_at: Option<i64>,
    pub duration: Option<i64>,
    pub bandwidth_usage: Option<i64>,
    pub duration_fee: Option<i64>,
    pub bandwidth_fee: Option<i64>,
    pub total_fee: Option<i64>,
    pub status: i32,
}

impl From<StorageSession> for Session {
    fn from(model: StorageSession) -> Self {
        Session {
            id: model.id,
            provider_id: model.provider_id,
            client_id: model.client_id,
            rate_per_second: szabo_to_u256(model.rate_per_second.unwrap_or_default()),
            rate_per_kb: szabo_to_u256(model.rate_per_kb.unwrap_or_default()),
            handshake_at: model.handshake_at,
            end_at: model.end_at,
            duration: model.duration,
            bandwidth_usage: model.bandwidth_usage,
            duration_fee: szabo_to_u256(model.duration_fee.unwrap_or_default()),
            bandwidth_fee: szabo_to_u256(model.bandwidth_fee.unwrap_or_default()),
            total_fee: szabo_to_u256(model.total_fee.unwrap_or_default()),
            status: SessionStatus::from_i32(model.status).unwrap(),
        }
    }
}

impl Into<StorageSession> for Session {
    fn into(self) -> StorageSession {
        StorageSession {
            id: self.id,
            provider_id: self.provider_id,
            client_id: self.client_id,
            rate_per_second: Some(u256_to_szabo(self.rate_per_second.clone())),
            rate_per_kb: Some(u256_to_szabo(self.rate_per_kb.clone())),
            handshake_at: self.handshake_at,
            end_at: self.end_at,
            duration: self.duration,
            bandwidth_usage: self.bandwidth_usage,
            duration_fee: Some(u256_to_szabo(self.duration_fee.clone())),
            bandwidth_fee: Some(u256_to_szabo(self.bandwidth_fee.clone())),
            total_fee: Some(u256_to_szabo(self.total_fee.clone())),
            status: self.status as i32,
        }
    }
}
