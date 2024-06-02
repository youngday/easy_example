use futures::StreamExt;

use tmq::{subscribe, Context, Result};

use log::{debug, error, info, trace, warn};
use log4rs;
#[tokio::main]
async fn main() -> Result<()> {
    log4rs::init_file("examples/config/log.yaml", Default::default()).unwrap();
    let version: String = "0.3.1102".to_string();
    trace!("some trace log");
    debug!("some debug log");
    info!("some information log");
    warn!("some warning log");
    error!("some error log");

    info!("version:{0}",version);
    

    let mut socket = subscribe(&Context::new())
        .connect("tcp://127.0.0.1:7899").unwrap()
        .subscribe(b"topic").unwrap();



    while let Some(msg) = socket.next().await {
        info!(
            "Subscribe: {:?}",
            msg?.iter()
                .map(|item| item.as_str().unwrap_or("invalid text"))
                .collect::<Vec<&str>>()
        );
    }
    Ok(())
}
