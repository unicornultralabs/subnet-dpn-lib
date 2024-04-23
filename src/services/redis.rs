use anyhow::anyhow;
use anyhow::{Error, Result};
use async_trait::async_trait;
use mockall::automock;
use redis::Commands;
use std::{fmt::Debug, sync::Arc};

use crate::types::bandwidth::UserBandwidthPrice;
use crate::types::region::UserRegionInfo;

const PROVIDER_PRICE_KEY: &str = "provider.price";
const PROVIDER_REGION_KEY: &str = "provider.region";

#[automock]
#[async_trait]
pub trait RedisService: Debug + Send + Sync + 'static {
    async fn set_provider_price(
        self: Arc<Self>,
        provider_addr: String,
        price: UserBandwidthPrice,
    ) -> Result<(), Error>;
    async fn get_provider_price(
        self: Arc<Self>,
        provider_addr: String,
    ) -> Result<UserBandwidthPrice, Error>;
    async fn set_provider_region(
        self: Arc<Self>,
        provider_addr: String,
        region: UserRegionInfo,
    ) -> Result<()>;
    async fn get_provider_region(self: Arc<Self>, provider_addr: String) -> Result<UserRegionInfo>;
}

#[derive(Debug)]
pub struct RedisServiceImpl {
    client: redis::Client,
}

#[async_trait]
impl RedisService for RedisServiceImpl {
    async fn set_provider_price(
        self: Arc<Self>,
        provider_addr: String,
        price: UserBandwidthPrice,
    ) -> Result<(), Error> {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| anyhow!("cannot get connection err={}", e))?;

        match conn.hset::<String, String, String, String>(
            String::from(PROVIDER_PRICE_KEY),
            provider_addr,
            serde_json::to_string(&price).unwrap(),
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!("redis failed to insert user price err={}", e)),
        }
    }

    async fn get_provider_price(
        self: Arc<Self>,
        provider_addr: String,
    ) -> Result<UserBandwidthPrice, Error> {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| anyhow!("cannot get connection err={}", e))?;
        let price_str: String = conn
            .hget(PROVIDER_PRICE_KEY, provider_addr.clone())
            .map_err(|e| {
                anyhow!(
                    "redis cannot get key={}:{} err={}",
                    PROVIDER_PRICE_KEY,
                    provider_addr,
                    e
                )
            })?;
        let price = serde_json::from_str::<UserBandwidthPrice>(&price_str)
            .map_err(|e| anyhow!("redis failed to decode user price err={}", e))?;
        Ok(price)
    }

    async fn set_provider_region(
        self: Arc<Self>,
        provider_addr: String,
        region: UserRegionInfo,
    ) -> Result<(), Error> {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| anyhow!("cannot get connection err={}", e))?;

        match conn.hset::<String, String, String, String>(
            String::from(PROVIDER_REGION_KEY),
            provider_addr,
            serde_json::to_string(&region).unwrap(),
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!("redis failed to insert user region err={}", e)),
        }
    }

    async fn get_provider_region(
        self: Arc<Self>,
        provider_addr: String,
    ) -> Result<UserRegionInfo, Error> {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| anyhow!("cannot get connection err={}", e))?;
        let price_str: String = conn
            .hget(PROVIDER_REGION_KEY, provider_addr.clone())
            .map_err(|e| {
                anyhow!(
                    "redis cannot get key={}:{} err={}",
                    PROVIDER_PRICE_KEY,
                    provider_addr,
                    e
                )
            })?;
        let region = serde_json::from_str::<UserRegionInfo>(&price_str)
            .map_err(|e| anyhow!("redis failed to decode user region err={}", e))?;
        Ok(region)
    }
}

impl RedisServiceImpl {
    pub fn new(redis_uri: String) -> Result<Self> {
        let client = redis::Client::open(redis_uri)?;
        Ok(Self { client })
    }
}
