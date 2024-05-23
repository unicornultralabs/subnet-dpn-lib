#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Continent {
    pub code: String,
    pub geoname_id: i64,
    pub name: String,
}
