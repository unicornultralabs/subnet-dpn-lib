use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::geo::Geo;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MasternodeInfo {
    pub peer_bind: String,
    pub client_bind: String,
    pub control_bind: String,
    pub web_bind: String,
    pub root_ca: Option<String>,
    pub geo: Geo,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AssignMasternodeRes {
    pub masternode: Option<MasternodeInfo>,
}
