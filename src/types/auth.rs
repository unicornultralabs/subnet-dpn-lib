use serde::{Deserialize, Serialize};

use super::user::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserClaims {
    pub user_id: i64,
    pub user: User,
    pub exp: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VerifyAuthTokenReq {
    pub access_token: String,
    pub refresh_token: String,
}
