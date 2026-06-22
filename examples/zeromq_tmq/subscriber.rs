use futures::StreamExt;
use tmq::{subscribe, Context, Result};
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter, Registry};

#[tokio::main]
async fn main() -> Result<()> {
    // tracing subscriber 初始化
    let file_appender = tracing_appender::rolling::daily("examples/logs", "zmq_sub.log");
    Registry::default()
        .with(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .with(fmt::layer().pretty().with_line_number(true))
        .with(fmt::layer().json().with_writer(file_appender))
        .init();

    let version: String = "0.3.1102".to_string();
    info!("version:{0}", version);
    
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