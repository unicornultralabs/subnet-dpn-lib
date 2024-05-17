#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoProxyAcc {
    #[prost(string, tag = "1")]
    pub user_addr: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub created_at: i64,
}
