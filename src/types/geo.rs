use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Result};
use maxminddb::geoip2;
use std::net::IpAddr;

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

pub fn get_geo_from_ip_address(mmdb_path: String, ip_addr: String) -> Result<Geo> {
    let ip_addr: IpAddr = ip_addr
        .clone()
        .parse()
        .map_err(|e| anyhow!("parse ip addr failed err={}", e))?;

    let reader = maxminddb::Reader::open_readfile(mmdb_path)
        .map_err(|e| anyhow!("failed to read mmdb file err={}", e))?;

    let geo = reader.lookup::<geoip2::City>(ip_addr).map_err(|e| {
        anyhow!(
            "no mmdb record for ip_addr={} err={}",
            ip_addr.to_string(),
            e
        )
    })?;

    let mut geo_city: Option<City> = None;
    let mut geo_country: Option<Country> = None;
    let mut geo_continent: Option<Continent> = None;
    let mut geo_location: Option<Location> = None;

    if let Some(city) = geo.city {
        geo_city = Some(City {
            geoname_id: city.geoname_id,
            name: city
                .names
                .unwrap_or_default()
                .get("en")
                .map(|n| n.to_string()),
        });
    }
    if let Some(country) = geo.country {
        geo_country = Some(Country {
            geoname_id: country.geoname_id,
            name: country
                .names
                .unwrap_or_default()
                .get("en")
                .map(|n| n.to_string()),
            is_in_european_union: country.is_in_european_union,
            iso_code: country.iso_code.map(|i| i.to_string()),
        });
    }
    if let Some(continent) = geo.continent {
        geo_continent = Some(Continent {
            geoname_id: continent.geoname_id,
            name: continent
                .names
                .unwrap_or_default()
                .get("en")
                .map(|n| n.to_string()),
            code: continent.code.map(|c| c.to_string()),
        });
    }
    if let Some(location) = geo.location {
        geo_location = Some(Location {
            latitude: location.latitude,
            longitude: location.longitude,
            metro_code: location.metro_code,
            time_zone: location.time_zone.map(|t| t.to_string()),
        });
    }

    Ok(Geo {
        city: geo_city,
        continent: geo_continent,
        country: geo_country,
        location: geo_location,
    })
}
