#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoProxyPayload {
    #[prost(string, tag = "1")]
    pub origin_topic: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub stream_id: u64,
    #[prost(bytes = "vec", tag = "3")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoVpnPayload {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoHealthCheck {
    #[prost(string, tag = "1")]
    pub msg: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoStreamPayload {
    #[prost(oneof = "proto_stream_payload::Payload", tags = "1, 2, 3")]
    pub payload: ::core::option::Option<proto_stream_payload::Payload>,
}
/// Nested message and enum types in `ProtoStreamPayload`.
pub mod proto_stream_payload {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag = "1")]
        ProxyPayload(super::ProtoProxyPayload),
        #[prost(message, tag = "2")]
        VpnPayload(super::ProtoVpnPayload),
        #[prost(message, tag = "3")]
        HealthCheck(super::ProtoHealthCheck),
    }
}
