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

    pub fn set_sorted_set(
        self: Arc<Self>,
        key: String,
        score: u32,
        value: u32,
    ) -> Result<(), Error> {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| anyhow!("cannot get connection err={}", e))?;
        match conn.zadd::<String, u32, u32, ()>(key, value, score) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!(
                "redis failed to insert peer into peer queue err={}",
                e
            )),
        }
    }

    pub fn remove_sorted_set_item(
        self: Arc<Self>,
        key: String,
        value: u32,
    ) -> Result<(), anyhow::Error> {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| anyhow!("cannot get connection err={}", e))?;

        match conn.zrem::<String, u32, usize>(key, value) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!(
                "redis failed to remove peer in peer queue err={}",
                e
            )),
        }
    }

    pub fn set_score_for_all(
        self: Arc<Self>,
        key: String,
        score: u32,
    ) -> Result<(), anyhow::Error> {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| anyhow!("cannot get connection err={}", e))?;

        let elements: Vec<(u32, u32)> = conn
            .zrange_withscores(key.clone(), 0, -1)
            .map_err(|e| anyhow!("redis failed to get sorted set err={}", e))?;

        for (value, _) in elements {
            conn.zadd::<String, u32, u32, ()>(key.clone(), value, score)
                .map_err(|e| anyhow!("redis failed to set scores err={}", e))?;
        }

        Ok(())
    }

    pub fn get_all_sorted_set(self: Arc<Self>, key: String) -> Result<Vec<(u32, u32)>, Error> {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| anyhow!("cannot get connection err={}", e))?;

        let elements: Vec<(u32, u32)> = conn
            .zrange_withscores(key.clone(), 0, -1)
            .map_err(|e| anyhow!("redis failed to get peer queue err={}", e))?;

        let mut result: Vec<(u32, u32)> = elements
            .into_iter()
            .map(|(value, score)| (value, score))
            .collect();

        result.sort_by_key(|(_value, score)| *score);

        Ok(result)
    }

    /// this function is used to delete data of given key
    pub fn delete(self: Arc<Self>, key: String) -> Result<(), Error> {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| anyhow!("cannot get connection err={}", e))?;

        conn.del(key.clone())
            .map_err(|e| anyhow!("redis failed to delete key={} err={}", key, e))
    }
}

pub fn get_geo_kf(masternode_id: String, login_session_id: String) -> (String, String) {
    (
        "peer_geo".to_owned(),
        format!("{}_{}", masternode_id.clone(), login_session_id.clone()),
    )
}

pub fn get_peer_queue_k() -> String {
    "peer_queue".to_owned()
}

pub fn get_price_kf(peer_addr: String) -> (String, String) {
    ("peer_price".to_owned(), peer_addr)
}

pub fn get_proxy_acc_kf(id: String) -> (String, String) {
    ("proxy_acc".to_owned(), id)
}
