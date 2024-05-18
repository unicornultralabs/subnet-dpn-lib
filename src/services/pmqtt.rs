use anyhow::anyhow;
use log::info;
use std::time::Duration;
use tokio::time::sleep;

pub async fn try_reconnect(cli: &paho_mqtt::Client) -> bool {
    info!("connection lost, waiting to retry connection...");
    for _ in 0..12 {
        sleep(Duration::from_millis(100)).await;
        if cli.reconnect().is_ok() {
            info!("reconnected mqtt");
            return true;
        }
    }
    info!("unable to reconnect after several attempts.");
    false
}

pub async fn get_mqtt_consumer_client(
    host: String,
    client_id: String,
) -> anyhow::Result<(
    paho_mqtt::AsyncClient,
    paho_mqtt::AsyncReceiver<Option<paho_mqtt::Message>>,
)> {
    let create_opts = paho_mqtt::CreateOptionsBuilder::new_v3()
        .server_uri(host)
        .client_id(client_id)
        .finalize();
    let mut cli = paho_mqtt::AsyncClient::new(create_opts)
        .map_err(|e| anyhow!("creating the client failed err={}", e))?;
    let strm = cli.get_stream(25);
    let conn_opts = paho_mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(60))
        .clean_session(false)
        .will_message(paho_mqtt::Message::new(
            "test/lwt",
            "[LWT] Async subscriber lost connection",
            paho_mqtt::QOS_1,
        ))
        .finalize();
    cli.connect(conn_opts)
        .await
        .map_err(|e| anyhow!("cannot connect to mqtt err={}", e))?;
    Ok((cli, strm))
}

pub async fn get_mqtt_publisher_client(host: String) -> anyhow::Result<paho_mqtt::AsyncClient> {
    let cli = paho_mqtt::AsyncClient::new(host)
        .map_err(|e| anyhow!("creating the client failed err={}", e))?;
    cli.connect(None)
        .await
        .map_err(|e| anyhow!("cannot connect to mqtt err={}", e))?;
    Ok(cli)
}
