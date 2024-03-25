use log::{debug, error, info, trace, warn};
use log4rs;
mod settings;
use serde::{Deserialize, Serialize};
use settings::Settings;


use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use std::env;
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

    // Allow passing an address to listen on as the first argument of this
    // program, but otherwise we'll just set up our TCP listener on
    // 127.0.0.1:8080 for connections.
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6142".to_string());

    // Next up we create a TCP listener which will listen for incoming
    // connections. This TCP listener is bound to the address we determined
    // above and must be associated with an event loop.
    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on: {}", addr);

    loop {
        // Asynchronously wait for an inbound socket.
        let (mut socket, _) = listener.accept().await?;

        // And this is where much of the magic of this server happens. We
        // crucially want all clients to make progress concurrently, rather than
        // blocking one on completion of another. To achieve this we use the
        // `tokio::spawn` function to execute the work in the background.
        //
        // Essentially here we're executing a new task to run concurrently,
        // which will allow all of our clients to be processed concurrently.

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return;
                }

                socket
                    .write_all(&buf[0..n])
                    .await
                    .expect("failed to write data to socket");
                let outstring = &buf[0..n];
                info!("received:{:?}", outstring);
            }
        });
    }
}
