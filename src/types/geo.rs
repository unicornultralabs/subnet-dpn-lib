use serde::{Deserialize, Serialize};

pub const DEFAULT_CONTINENTAL_CODE: &str = "DEFAULT";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Geo {
    pub city: Option<City>,
    pub continent: Option<Continent>,
    pub country: Option<Country>,
    pub location: Option<Location>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Continent {
    pub code: Option<String>,
    pub geoname_id: Option<u32>,
    pub name: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Country {
    pub geoname_id: Option<u32>,
    pub is_in_european_union: Option<bool>,
    pub iso_code: Option<String>,
    pub name: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct City {
    pub geoname_id: Option<u32>,
    pub name: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Location {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub metro_code: Option<u16>,
    pub time_zone: Option<String>,
}

impl Default for Geo {
    fn default() -> Self {
        Self {
            city: Some(City::default()),
            continent: Some(Continent::default()),
            country: Some(Country::default()),
            location: Some(Location::default()),
        }
    }
}

impl Default for Continent {
    fn default() -> Self {
        Self {
            code: Some("Default code".to_string()),
            geoname_id: Some(0),
            name: Some("Default continent name".to_string()),
        }
    }
}

impl Default for Country {
    fn default() -> Self {
        Self {
            geoname_id: Some(0),
            is_in_european_union: Some(false),
            iso_code: Some("Default iso_code".to_string()),
            name: Some("Default country name".to_string()),
        }
    }
}

impl Default for City {
    fn default() -> Self {
        Self {
            geoname_id: Some(0),
            name: Some("Default city".to_string()),
        }
    }
}

impl Default for Location {
    fn default() -> Self {
        Self {
            latitude: Some(0.0),
            longitude: Some(0.0),
            metro_code: Some(0),
            time_zone: Some("Default timezone".to_string()),
        }
    }
}
