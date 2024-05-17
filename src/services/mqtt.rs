use anyhow::anyhow;
use rumqttc::v5::{AsyncClient, EventLoop, MqttOptions};
use std::time::Duration;

pub struct MqttService {}

impl MqttService {
    pub async fn new(url: String) -> anyhow::Result<(AsyncClient, EventLoop)> {
        let mut options = MqttOptions::parse_url(url.clone())
            .map_err(|e| anyhow!("cannot get parse url={} err={}", url, e))?;
        options.set_keep_alive(Duration::from_secs(10));
        Ok(AsyncClient::new(options, 10))
    }
}
