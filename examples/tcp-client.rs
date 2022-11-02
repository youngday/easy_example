use log::{debug, error, info, trace, warn};
use log4rs;
mod settings;
use settings::Settings;
use serde::{Deserialize, Serialize};


//#![warn(rust_2018_idioms)]
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

    // Open a TCP stream to the socket address.
    //
    // Note that this is the Tokio TcpStream, which is fully async.
    let mut stream = TcpStream::connect("127.0.0.1:6142").await?;
    info!("created stream");

    let result = stream.write(b"hello world\n").await;
    info!("wrote to stream; success={:?}", result.is_ok());

    Ok(())
}
