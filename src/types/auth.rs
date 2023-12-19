#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct VerifyAuthTokenReq {
    pub access_token: String,
    pub refresh_token: String,
}
