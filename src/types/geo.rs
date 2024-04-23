use serde::{Deserialize, Serialize};

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
