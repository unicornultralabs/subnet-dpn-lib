use anyhow::{anyhow, Error, Result};
use redis::Commands as _;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::{fmt::Debug, sync::Arc};

#[derive(Debug)]
pub struct RedisService {
    client: redis::Client,
}

impl RedisService {
    pub fn new(redis_uri: String) -> Result<Self> {
        let client = redis::Client::open(redis_uri)
            .map_err(|e| anyhow!("redis: cannot open client err={}", e))?;
        _ = client
            .get_connection()
            .map_err(|e| anyhow!("redis: cannot get connection err={}", e))?;
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

pub fn get_geo_kf(masternode_id: String, login_session_id: String) -> (String, String) {
    (
        "peer_geo".to_owned(),
        format!("{}_{}", masternode_id.clone(), login_session_id.clone()),
    )
}

pub fn get_price_kf(peer_addr: String) -> (String, String) {
    ("peer_price".to_owned(), peer_addr)
}

pub fn get_proxy_acc_kf(id: String) -> (String, String) {
    ("proxy_acc".to_owned(), id)
}
