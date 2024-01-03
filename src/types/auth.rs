use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::user::User;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserClaims {
    pub user: User,
    pub exp: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(serde::Deserialize, Debug, Clone, ToSchema)]
pub enum SSOInfo {
    Google(GoogleSSOInfo),
    Apple(AppleSSOInfo),
}

#[derive(serde::Deserialize, Debug, Clone, ToSchema)]
pub struct GoogleSSOInfo {
    pub id: String,
    pub email: String,
    pub display_name: String,
    pub photo_url: String,
    pub token: String,
}

#[derive(serde::Deserialize, Debug, Clone, ToSchema)]
pub struct AppleSSOInfo {
    pub email: String,
    pub full_name: String,
    pub code: String,
    pub token: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct SSORes {
    pub code: i16,
    pub user_id: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub error_msg: String,
}
