#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoSession {
    #[prost(string, tag = "1")]
    pub provider_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub client_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub client_identifier: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub handshaked_at: i64,
}
