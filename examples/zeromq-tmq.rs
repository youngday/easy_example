use async_std::task;
use config;
use config::Config;
use futures::{SinkExt, StreamExt};
use lazy_static;
use log::{debug, error, info, trace, warn};
use log4rs;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{error::Error, sync::RwLock, time::Duration};
use tmq::{publish, subscribe, Context};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Application {
    application: Data,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Data {
    build: String,
    container_name: String,
    environment2: Data2,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Data2 {
    one_env2: String,
    sec_env2: String,
}

lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new(Config::default());
}

#[async_std::main] //async-std = { features = ["attributes"] }
async fn main() -> Result<(), Box<dyn Error>> {
    log4rs::init_file("examples/config/log.yaml", Default::default()).unwrap();
    trace!("demo trace");
    debug!("demo debug");
    info!("demo info");
    warn!("demo warn");
    error!("demo error");
    //global config from local
    SETTINGS.write()?.set("version", "0.2.0910".to_string())?;
    info!("version: {}", SETTINGS.read()?.get::<String>("version")?);
    //global config from file
    let mut settings = Config::default();
    settings
        // File::with_name(..) is shorthand for File::from(Path::new(..))
        .merge(config::File::with_name("conf/00-default.toml"))
        .unwrap()
        .merge(config::File::with_name("conf/05-some.yaml"))
        .unwrap()
        .merge(config::File::with_name("conf/99-extra.json"))
        .unwrap();
    //decode from toml yaml json ,NOTE:need flat layer,no struct en/decode,simple is good.
    let file_set = settings.try_into::<HashMap<String, String>>().unwrap();
    let test_str = file_set.get("debug").unwrap();
    // Print out our settings (as a HashMap)
    info!("\n{:?}", file_set);
    info!("\n{:?}", test_str);

    info!("Start your app.");

    let send_task = task::spawn(async {
        task::Builder::new()
            .name("send_task".to_string())
            .spawn(send())
            .unwrap()
            .await;
    });
    let recv_udp_task = task::spawn(async {
        task::Builder::new()
            .name("recv_udp_task".to_string())
            .spawn(received_udp())
            .unwrap()
            .await;
    });

    let recv_serial_task = task::spawn(async {
        task::Builder::new()
            .name("recv_serial_task".to_string())
            .spawn(recv_serial())
            .unwrap()
            .await;
    });
    let recv_http_task = task::spawn(async {
        task::Builder::new()
            .name("recv_http_task".to_string())
            .spawn(recv_http())
            .unwrap()
            .await;
    });
    let recv_test_task = task::spawn(async {
        task::Builder::new()
            .name("recv_test_task".to_string())
            .spawn(recv_test())
            .unwrap()
            .await;
    });

    send_task.await;
    recv_udp_task.await;
    recv_serial_task.await;
    recv_http_task.await;
    recv_test_task.await;

    Ok(())
}

async fn send() {
    let bind_ip = "tcp://127.0.0.1:7898";
    let mut socket = publish(&Context::new()).bind(bind_ip).unwrap();
    let mut i = 0;
    loop {
        i += 1;
        let message = format!("Broadcast #{}", i);
        info!("Publish: {}", message);
        socket
            .send(vec![b"UDP1" as &[u8], message.as_bytes()])
            .await
            .unwrap();
        socket
            .send(vec![b"SERIAL1" as &[u8], message.as_bytes()])
            .await
            .unwrap();
        socket
            .send(vec![b"HTTP1" as &[u8], message.as_bytes()])
            .await
            .unwrap();
        socket
            .send(vec![b"AAA" as &[u8], message.as_bytes()])
            .await
            .unwrap();

        task::sleep(Duration::from_millis(2000)).await;
    }
}

async fn received_udp() {
    let bind_ip = "tcp://127.0.0.1:7898";
    let mut socket = subscribe(&Context::new())
        .connect(bind_ip)
        .unwrap()
        .subscribe(b"UDP1")
        .unwrap();

    while let Some(msg) = socket.next().await {
        info!(
            "Subscribe: {:?}",
            msg.unwrap()
                .iter()
                .map(|item| item.as_str().unwrap_or("invalid text"))
                .collect::<Vec<&str>>()
        );
    }
}

async fn recv_serial() {
    let bind_ip = "tcp://127.0.0.1:7898";
    let mut socket = subscribe(&Context::new())
        .connect(bind_ip)
        .unwrap()
        .subscribe(b"SERIAL1")
        .unwrap();

    while let Some(msg) = socket.next().await {
        info!(
            "Subscribe: {:?}",
            msg.unwrap()
                .iter()
                .map(|item| item.as_str().unwrap_or("invalid text"))
                .collect::<Vec<&str>>()
        );
    }
}

async fn recv_http() {
    let bind_ip = "tcp://127.0.0.1:7898";
    let mut socket = subscribe(&Context::new())
        .connect(bind_ip)
        .unwrap()
        .subscribe(b"HTTP1")
        .unwrap();

    while let Some(msg) = socket.next().await {
        info!(
            "Subscribe: {:?}",
            msg.unwrap()
                .iter()
                .map(|item| item.as_str().unwrap_or("invalid text"))
                .collect::<Vec<&str>>()
        );
    }
}

async fn recv_test() {
    let bind_ip = "tcp://127.0.0.1:7898";
    let mut socket = subscribe(&Context::new())
        .connect(bind_ip)
        .unwrap()
        .subscribe(b"AAA")
        .unwrap();

    while let Some(msg) = socket.next().await {
        info!(
            "Subscribe: {:?}",
            msg.unwrap()
                .iter()
                .map(|item| item.as_str().unwrap_or("invalid text"))
                .collect::<Vec<&str>>()
        );
    }
}
