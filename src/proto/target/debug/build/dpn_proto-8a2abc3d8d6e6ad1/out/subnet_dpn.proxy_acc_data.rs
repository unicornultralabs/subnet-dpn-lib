#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoProxyAccData {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub ip_rotation_period: i32,
    #[prost(string, tag = "4")]
    pub whitelist_ip_list: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub city_geoname_id: i64,
    #[prost(int64, tag = "6")]
    pub country_geoname_id: i64,
    #[prost(int64, tag = "7")]
    pub rate_per_kb: i64,
    #[prost(int64, tag = "8")]
    pub rate_per_second: i64,
}
