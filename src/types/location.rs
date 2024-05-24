use utoipa::ToSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema)]
pub struct Location {
    pub geoname_id: i32,
    pub city_name: String,
    pub continent_geoname_id: i32,
    pub continent_code: String,
    pub continent_name: String,
    pub country_geoname_id: i32,
    pub country_iso_code: String,
    pub country_name: String,
}

impl Location {
    pub fn new(
        geoname_id: i32,
        city_name: String,
        continent_geoname_id: i32,
        continent_code: String,
        continent_name: String,
        country_geoname_id: i32,
        country_iso_code: String,
        country_name: String,
    ) -> Self {
        Self {
            geoname_id,
            city_name,
            continent_geoname_id,
            continent_code,
            continent_name,
            country_geoname_id,
            country_iso_code,
            country_name,
        }
    }
}
