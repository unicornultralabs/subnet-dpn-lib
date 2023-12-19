use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, FromPrimitive, Serialize, Deserialize)]

pub enum SSOMethod {
    Google,
    Apple,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct SSOReq {
    pub method: i16,
    pub payload: String,
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
