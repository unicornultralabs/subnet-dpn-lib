use crate::utils::{bytes_to_hex_string, szabo_to_u256, u256_to_szabo};
use dpn_types::user::User;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct StorageUser {
    pub id: i64,
    pub username: Option<String>,
    pub fingerprint: Option<String>,
    pub pincode: Option<String>,
    pub deposit_addr: String,
    pub withdrawal_addr: Option<String>,
    /// In Szabo unit. Example: 6868.123456 U2U is converted to 6868123456
    pub balance: i64,
    pub created_at: i64,
    pub last_login: i64,
}

impl From<StorageUser> for User {
    fn from(model: StorageUser) -> Self {
        User {
            id: model.id,
            username: model.username,
            fingerprint: model.fingerprint,
            pincode: model.pincode,
            deposit_addr: model.deposit_addr.parse().unwrap(),
            withdrawal_addr: model.withdrawal_addr.map(|wa| wa.parse().unwrap()),
            balance: szabo_to_u256(model.balance),
            created_at: model.created_at,
            last_login: model.last_login,
        }
    }
}

impl Into<StorageUser> for User {
    fn into(self) -> StorageUser {
        StorageUser {
            id: self.id,
            username: self.username,
            fingerprint: self.fingerprint.map(|x| x.to_owned()),
            pincode: self.pincode,
            deposit_addr: bytes_to_hex_string(self.deposit_addr.as_bytes()),
            withdrawal_addr: self
                .withdrawal_addr
                .map(|wa| bytes_to_hex_string(wa.as_bytes())),
            balance: u256_to_szabo(self.balance.clone()),
            created_at: self.created_at,
            last_login: self.last_login,
        }
    }
}
