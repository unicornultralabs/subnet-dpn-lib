use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use mockall::automock;
use redis::Commands as _;
use std::{fmt::Debug, sync::Arc};

use crate::types::bandwidth::UserBandwidthPrice;
use crate::types::connection::ProxyAccData;
use crate::types::geo::Geo;

const PROVIDER_PRICE_KEY: &str = "provider.price";
const PROVIDER_GEO_KEY: &str = "provider.geo";
const PROXY_ACC_KEY: &str = "proxyacc";

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

    async fn set_provider_geo(self: Arc<Self>, provider_addr: String, geo: Geo) -> Result<()>;
    async fn get_provider_geo(self: Arc<Self>, provider_addr: String) -> Result<Geo>;

    async fn set_proxy_acc(self: Arc<Self>, proxy_acc: ProxyAccData) -> Result<(), Error>;
    async fn get_proxy_acc(self: Arc<Self>, proxy_acc_id: String) -> Result<ProxyAccData, Error>;
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

        match conn.hset::<String, String, String, usize>(
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

    async fn set_provider_geo(
        self: Arc<Self>,
        login_session_id: String,
        geo: Geo,
    ) -> Result<(), Error> {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| anyhow!("cannot get connection err={}", e))?;

        match conn.hset::<String, String, String, usize>(
            String::from(PROVIDER_GEO_KEY),
            login_session_id,
            serde_json::to_string(&geo).unwrap(),
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!("redis failed to insert geography err={}", e)),
        }
    }

    async fn get_provider_geo(self: Arc<Self>, login_session_id: String) -> Result<Geo, Error> {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| anyhow!("cannot get connection err={}", e))?;
        let geo_str: String = conn
            .hget(PROVIDER_GEO_KEY, login_session_id.clone())
            .map_err(|e| {
                anyhow!(
                    "redis cannot get key={}:{} err={}",
                    PROVIDER_PRICE_KEY,
                    login_session_id,
                    e
                )
            })?;
        let geo = serde_json::from_str::<Geo>(&geo_str)
            .map_err(|e| anyhow!("redis failed to decode geography err={}", e))?;
        Ok(geo)
    }

    async fn set_proxy_acc(self: Arc<Self>, proxy_acc: ProxyAccData) -> Result<(), Error> {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| anyhow!("cannot get connection err={}", e))?;

        match conn.hset::<String, String, String, usize>(
            String::from(PROXY_ACC_KEY),
            proxy_acc.id.clone(),
            serde_json::to_string(&proxy_acc).unwrap(),
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!("redis failed to insert proxy acc err={}", e)),
        }
    }

    async fn get_proxy_acc(self: Arc<Self>, proxy_acc_id: String) -> Result<ProxyAccData, Error> {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| anyhow!("cannot get connection err={}", e))?;
        let proxy_acc_str: String =
            conn.hget(PROXY_ACC_KEY, proxy_acc_id.clone())
                .map_err(|e| {
                    anyhow!(
                        "redis cannot get key={}:{} err={}",
                        PROXY_ACC_KEY,
                        proxy_acc_id,
                        e
                    )
                })?;
        let proxy_acc = serde_json::from_str::<ProxyAccData>(&proxy_acc_str)
            .map_err(|e| anyhow!("redis failed to decode geography err={}", e))?;
        Ok(proxy_acc)
    }
}

impl RedisServiceImpl {
    pub fn new(redis_uri: String) -> Result<Self> {
        let client = redis::Client::open(redis_uri)?;
        Ok(Self { client })
    }
}
