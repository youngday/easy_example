#[macro_use]
extern crate log;

use env_logger::Env;

use std::fs::File;
use std::io::Read;

use serde::{Deserialize, Serialize};

use async_std::io;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;

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

fn main() -> io::Result<()> {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "debug")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    trace!("some trace log");
    debug!("some debug log");
    info!("some information log");
    warn!("some warning log");
    error!("some error log");

    let filename = "config.yaml";
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();

            let application_data: Application = serde_yaml::from_str(&content).unwrap();
            info!("{:?}", application_data.application.build);
            info!("{:?}", application_data.application.environment);
            //info!("{:?}", application_data.application.environment2);
            //info!("{:?}", application_data.application.environment2.one_env2);
        }
        Err(error) => {
            info!("There is an error {}: {}", filename, error);
        }
    }

    info!("Start your app.");

    task::block_on(say_hi());

    task::block_on(say_hi2());

    task::block_on(async {
        let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
        info!("Connected to {}", &stream.peer_addr()?);

        let msg = "hello world";
        info!("<- {}", msg);
        stream.write_all(msg.as_bytes()).await?;

        let mut buf = vec![0u8; 1024];
        let n = stream.read(&mut buf).await?;
        //let n = stream.read(&mut buf).await?;
        info!("-> {}\n", String::from_utf8_lossy(&buf[..n]));

        Ok(())
    })
}

async fn say_hi() {
    info!("Task1!");
}

async fn say_hi2() {
    info!("Task2!");
}
