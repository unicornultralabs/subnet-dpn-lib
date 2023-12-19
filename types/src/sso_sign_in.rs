use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, FromPrimitive, Serialize, Deserialize)]

pub enum SSOMethod {
    Google,
    Apple,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct SSOReq {
    method: i16,
    payload: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub enum SSOInfo {
    Google(GoogleSSOInfo),
    Apple(AppleSSOInfo),
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct GoogleSSOInfo {
    pub id: String,
    pub email: String,
    pub display_name: String,
    pub photo_url: String,
    pub token: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct AppleSSOInfo {
    pub email: String,
    pub full_name: String,
    pub code: String,
    pub token: String,
}

pub fn parse_sso_sign_in_req(req: SSOReq) -> Result<SSOInfo, String> {
    match SSOMethod::from_i16(req.method) {
        Some(method) => match method {
            SSOMethod::Google => match serde_json::from_str::<GoogleSSOInfo>(&req.payload)
            {
                Ok(info) => Ok(SSOInfo::Google(info)),
                Err(_) => Err(format!("invalid sign in payload")),
            },
            SSOMethod::Apple => match serde_json::from_str::<AppleSSOInfo>(&req.payload) {
                Ok(info) => Ok(SSOInfo::Apple(info)),
                Err(_) => Err(format!("invalid sign in payload")),
            },
        },
        None => Err(format!("unknown sign in method")),
    }
}

pub fn verify_sso_info(_: SSOInfo) -> Result<(), String> {
    // TODO(rameight): verify sso info
    Ok(())
}