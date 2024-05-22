use std::{net::IpAddr, sync::Arc};

use anyhow::{anyhow, Result};
use maxminddb::{geoip2, Reader};

use crate::types::geo::{City, Continent, Country, Geo, Location};

#[derive(Debug)]
pub struct GeoService {
    reader: Reader<Vec<u8>>,
}

impl GeoService {
    pub fn new(mmdb_path: String) -> Result<Self> {
        let reader = maxminddb::Reader::open_readfile(mmdb_path)
            .map_err(|e| anyhow!("failed to read mmdb file err={}", e))?;
        Ok(Self { reader })
    }

    pub fn get_geo_from_ip_address(self: Arc<Self>, ip_addr: String) -> Result<Geo> {
        let ip_addr: IpAddr = ip_addr
            .clone()
            .parse()
            .map_err(|e| anyhow!("parse ip addr failed err={}", e))?;

        let geo = self.reader.lookup::<geoip2::City>(ip_addr).map_err(|e| {
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
}
