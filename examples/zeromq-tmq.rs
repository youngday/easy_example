use log::{debug, error, info, trace, warn};
use log4rs;
mod settings;
use settings::Settings;
use serde::{Deserialize, Serialize};

use futures::{SinkExt, StreamExt};
use tmq::{publish, subscribe, Context};
// use std::collections::HashMap;
use std::{error::Error,  time::Duration};


use tokio::time::sleep;

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


#[tokio::main] 
async fn main() -> Result<(), Box<dyn Error>> {
    log4rs::init_file("examples/config/log.yaml", Default::default()).unwrap();
    trace!("some trace log");
    debug!("some debug log");
    info!("some information log");
    warn!("some warning log");
    error!("some error log");

    let settings = Settings::new();

    // Print out our settings
    info!("{:?}", settings);

    info!("Start your app.");

    let send_task = tokio::spawn(async {
        send().await;
    });
    let recv_udp_task = tokio::spawn(async {
        received_udp().await;
    });

    let recv_serial_task = tokio::spawn(async {
        recv_serial().await;
    });
    let recv_http_task = tokio::spawn(async {
        recv_http().await;
    });
    let recv_test_task = tokio::spawn(async {
        recv_test().await;
    });

    let result=send_task.await.unwrap();
    info!("result:{:?}",result);
    let result=recv_udp_task.await.unwrap();
    info!("result:{:?}",result);
    let result=recv_serial_task.await.unwrap();
    info!("result:{:?}",result);
    let result=recv_http_task.await.unwrap();
    info!("result:{:?}",result);
    let result=recv_test_task.await.unwrap();
    info!("result:{:?}",result);

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

            sleep(Duration::from_millis(200)).await;
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
