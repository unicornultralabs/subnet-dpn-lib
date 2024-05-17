#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoTx {
    #[prost(string, tag = "1")]
    pub from_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to_addr: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub amount: i64,
    #[prost(int32, tag = "4")]
    pub tx_type: i32,
    #[prost(int32, tag = "5")]
    pub tx_status: i32,
    #[prost(int64, tag = "6")]
    pub created_at: i64,
}
