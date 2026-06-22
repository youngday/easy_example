use tmq::{publish, Context, Result};

use futures::SinkExt;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::{fmt, prelude::*, EnvFilter, Registry};

#[tokio::main]
async fn main() -> Result<()> {
    // tracing subscriber 初始化
    let file_appender = tracing_appender::rolling::daily("examples/logs", "zmq_pub.log");
    Registry::default()
        .with(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .with(fmt::layer().pretty().with_line_number(true))
        .with(fmt::layer().json().with_writer(file_appender))
        .init();

    let version: String = "0.3.1102".to_string();
    trace!("some trace log");
    debug!("some debug log");
    info!("some information log");
    warn!("some warning log");
    error!("some error log");

    info!("version:{0}", version);

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