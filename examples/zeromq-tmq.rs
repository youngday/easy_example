use log::{debug, error, info, trace, warn};
use log4rs;
mod settings;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::collections::HashMap;

use futures::{SinkExt, StreamExt};
use tmq::{publish, subscribe, Context};

use std::{error::Error, fs::File, io::Read, time::Duration};

use tokio::time::sleep;

static HASHMAP: Lazy<HashMap<i32, String>> = Lazy::new(|| {
    let app_data: Application;
    let filename = "examples/config/config.yaml";
    let mut m = HashMap::new();
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();

            app_data = serde_yaml::from_str(&content).unwrap();
            info!("{:?}", app_data.app.build);
            info!("{:?}", app_data.app.environment);

            let _send_ip = app_data.app.net_cfg.send_ip;
            let _recv_ip = app_data.app.net_cfg.recv_ip;
            let _udp_pub_topic = app_data.app.net_cfg.udp_pub_topic;
            let _serial_pub_topic = app_data.app.net_cfg.serial_pub_topic;
            let _http_pub_topic = app_data.app.net_cfg.http_pub_topic;
            let _udp_sub_topic = app_data.app.net_cfg.udp_sub_topic;
            let _serial_sub_topic = app_data.app.net_cfg.serial_sub_topic;
            let _http_sub_topic = app_data.app.net_cfg.http_sub_topic;

            let _tcp_pub_topic = app_data.app.net_cfg.tcp_pub_topic;
            let _tcp_sub_topic = app_data.app.net_cfg.tcp_sub_topic;

            info!("_send_ip:{:?}", _send_ip);
            info!("_recv_ip:{:?}", _recv_ip);
            info!("_udp_pub_topic:{:?}", _udp_pub_topic);
            info!("_serial_pub_topic:{:?}", _serial_pub_topic);
            info!("_http_pub_topic:{:?}", _http_pub_topic);
            info!("_udp_sub_topic:{:?}", _udp_sub_topic);
            info!("_serial_sub_topic:{:?}", _serial_sub_topic);
            info!("_http_sub_topic:{:?}", _http_sub_topic);
            info!("_tcp_pub_topic:{:?}", _tcp_pub_topic);
            info!("_tcp_sub_topic:{:?}", _tcp_sub_topic);

            m.insert(0, _send_ip.to_string());
            m.insert(1, _recv_ip.to_string());
            m.insert(2, _udp_pub_topic.to_string());
            m.insert(3, _serial_pub_topic.to_string());
            m.insert(4, _http_pub_topic.to_string());
            m.insert(5, _udp_sub_topic.to_string());
            m.insert(6, _serial_sub_topic.to_string());
            m.insert(7, _http_sub_topic.to_string());
            m.insert(8, _tcp_pub_topic.to_string());
            m.insert(9, _tcp_sub_topic.to_string());
        }
        Err(error) => {
            info!("There is an error {}: {}", filename, error);
        }
    }
    m
});

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Application {
    app: Data,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Data {
    build: String,
    container_name: String,
    environment2: Data2,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<Vec<String>>,
    net_cfg: NetCfg,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Data2 {
    one_env2: String,
    sec_env2: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct NetCfg {
    send_ip: String,
    recv_ip: String,
    udp_pub_topic: String,
    serial_pub_topic: String,
    http_pub_topic: String,
    udp_sub_topic: String,
    serial_sub_topic: String,
    http_sub_topic: String,
    tcp_pub_topic: String,
    tcp_sub_topic: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    log4rs::init_file("examples/config/log.yaml", Default::default()).unwrap();
    let version: String = "0.3.1102".to_string();
    trace!("some trace log");
    debug!("some debug log");
    info!("some information log");
    warn!("some warning log");
    error!("some error log");

    let settings = Settings::new();

    // Print out our settings
    info!("{:?}", settings);

    info!("version:{:}", version);

    info!("_send_ip:{:?}", HASHMAP.get(&0));
    info!("_recv_ip:{:?}", HASHMAP.get(&1));

    info!("Start your app.");

    let send_task = tokio::spawn(async {
        send().await;
    });
    let recv_serial_task = tokio::spawn(async {
        recv_serial().await;
    });
    let recv_udp_task = tokio::spawn(async {
        received_udp().await;
    });
    let recv_tcp_task = tokio::spawn(async {
        received_tcp().await;
    });

    let recv_http_task = tokio::spawn(async {
        recv_http().await;
    });

    let result = send_task.await.unwrap();
    info!("result:{:?}", result);

    let result = recv_serial_task.await.unwrap();
    info!("result:{:?}", result);
    let result = recv_udp_task.await.unwrap();
    info!("result:{:?}", result);
    let result = recv_tcp_task.await.unwrap();
    info!("result:{:?}", result);
    let result = recv_http_task.await.unwrap();
    info!("result:{:?}", result);

    Ok(())
}

async fn send() {
    let _bindip = HASHMAP.get(&1).unwrap().to_string(); // _recv_ip;
    let _udp_pub_topic = HASHMAP.get(&2).unwrap().to_string(); // _udp_pub_topic;
    let _serial_pub_topic = HASHMAP.get(&3).unwrap().to_string(); // _serial_pub_topic;
    let _http_pub_topic = HASHMAP.get(&4).unwrap().to_string(); // _http_pub_topic;
    let _tcp_pub_topic = HASHMAP.get(&8).unwrap().to_string(); // _tcp_pub_topic;

    let mut socket = publish(&Context::new()).bind(&_bindip).unwrap();
    let mut i = 0;
    loop {
        i += 1;
        let message = format!("Broadcast #{}", i);
        info!("Publish: {}", message);
        socket.send(vec![&_udp_pub_topic, &message]).await.unwrap();
        socket
            .send(vec![&_serial_pub_topic, &message])
            .await
            .unwrap();
        socket.send(vec![&_http_pub_topic, &message]).await.unwrap();
        socket.send(vec![&_tcp_pub_topic, &message]).await.unwrap();
        socket.send(vec!["AAA", &message]).await.unwrap();

        sleep(Duration::from_millis(2000)).await;
    }
}

async fn received_udp() {
    let _bindip = HASHMAP.get(&0).unwrap().to_string(); // _send_ip;
    let _udp_sub_topic = HASHMAP.get(&5).unwrap().to_string(); // _udp_sub_topic;
    let mut socket = subscribe(&Context::new())
        .connect(&_bindip)
        .unwrap()
        .subscribe(_udp_sub_topic.as_bytes())
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
async fn received_tcp() {
    let _bindip = HASHMAP.get(&0).unwrap().to_string(); // _send_ip;
    let _udp_sub_topic = HASHMAP.get(&9).unwrap().to_string(); // _udp_sub_topic;
    let mut socket = subscribe(&Context::new())
        .connect(&_bindip)
        .unwrap()
        .subscribe(_udp_sub_topic.as_bytes())
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
    let _bindip = HASHMAP.get(&0).unwrap().to_string(); //_send_ip;
    let _serial_sub_topic = HASHMAP.get(&6).unwrap().to_string(); // _serial_sub_topic;
    let mut socket = subscribe(&Context::new())
        .connect(&_bindip)
        .unwrap()
        .subscribe(_serial_sub_topic.as_bytes())
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
    let _bindip = HASHMAP.get(&0).unwrap().to_string(); // _send_ip;
    let _http_sub_topic = HASHMAP.get(&7).unwrap().to_string(); // _http_sub_topic;
    let mut socket = subscribe(&Context::new())
        .connect(&_bindip)
        .unwrap()
        .subscribe(_http_sub_topic.as_bytes())
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
