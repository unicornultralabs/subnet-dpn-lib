use anyhow::{anyhow, Error, Result};
use redis::Commands as _;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::{fmt::Debug, sync::Arc};

pub const PROVIDER_PRICE_KEY: &str = "provider.price";
pub const PROVIDER_GEO_KEY: &str = "provider.geo";
pub const PROXY_ACC_KEY: &str = "proxyacc";

#[derive(Debug)]
pub struct RedisService {
    client: redis::Client,
}

impl RedisService {
    pub fn new(redis_uri: String) -> Result<Self> {
        let client = redis::Client::open(redis_uri)?;
        Ok(Self { client })
    }

    pub fn set_hash<T>(self: Arc<Self>, key: String, field: String, obj: T) -> Result<(), Error>
    where
        T: Serialize,
    {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| anyhow!("cannot get connection err={}", e))?;
        match conn.hset::<String, String, String, usize>(
            key,
            field,
            serde_json::to_string(&obj).unwrap(),
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!("redis failed to insert err={}", e)),
        }
    }

    pub fn get_hash<T>(self: Arc<Self>, key: String, field: String) -> Result<T, Error>
    where
        T: Clone + DeserializeOwned,
    {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| anyhow!("cannot get connection err={}", e))?;
        let obj_str: String = conn
            .hget(key.clone(), field.clone())
            .map_err(|e| anyhow!("redis cannot get key={}:{} err={}", key, field, e))?;
        let proxy_acc = serde_json::from_str::<T>(&obj_str)
            .map_err(|e| anyhow!("redis failed to decode err={}", e))?;
        Ok(proxy_acc)
    }
}
