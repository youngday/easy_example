use tmq::{publish, Context, Result};

use futures::SinkExt;
use std::time::Duration;
use tokio::time::sleep;
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

    let mut socket = publish(&Context::new()).bind("tcp://127.0.0.1:7899")?;

    let mut i: f64 = 0.0;
    loop {
 
        if i<100.0 {
            i += 1.0;
        }
        else {i=0.0;}
        let message = format!("{}", i*0.01);
        info!("Publish: {}", message);

        socket
            .send(vec![b"topic" as &[u8], message.as_bytes()])
            .await?;
        sleep(Duration::from_secs_f64(0.08)).await;
    }
}
