use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::{fmt, prelude::*, EnvFilter, Registry};
use easy_example::settings::Settings;
use serde::{Deserialize, Serialize};

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use std::error::Error;

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
pub async fn main() -> Result<(), Box<dyn Error>> {
    // tracing subscriber 初始化
    let file_appender = tracing_appender::rolling::daily("examples/logs", "tcp_client.log");
    Registry::default()
        .with(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .with(fmt::layer().pretty().with_line_number(true))
        .with(fmt::layer().json().with_writer(file_appender))
        .init();

    trace!("some trace log");
    debug!("some debug log");
    info!("some information log");
    warn!("some warning log");
    error!("some error log");

    let settings = Settings::new();

    // Print out our settings
    info!("{:?}", settings);

    info!("Start your app.");

    // Open a TCP stream to the socket address.
    //
    // Note that this is the Tokio TcpStream, which is fully async.
    let mut stream = TcpStream::connect("127.0.0.1:6142").await?;
    info!("created stream");

    let result = stream.write(b"hello world\n").await;
    info!("wrote to stream; success={:?}", result.is_ok());

    Ok(())
}
