use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct UserRegionInfo {
    pub user_addr: String,
    pub city_geoname_id: Option<u32>,
    pub country_geoname_id: u32,
}

impl UserRegionInfo {
    pub fn new(
        user_addr: String,
        city_geoname_id: Option<u32>,
        country_geoname_id: u32,
    ) -> Self {
        Self {
            user_addr: user_addr,
            city_geoname_id: city_geoname_id,
            country_geoname_id: country_geoname_id,
        }
    }
}
