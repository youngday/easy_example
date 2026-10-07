use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter, Registry};
use zeromq::{Socket, SocketRecv, SubSocket};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tracing subscriber 初始化
    let file_appender = tracing_appender::rolling::daily("examples/logs", "zmq_sub.log");
    Registry::default()
        .with(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .with(fmt::layer().pretty().with_line_number(true))
        .with(fmt::layer().json().with_writer(file_appender))
        .init();

    let version: String = "0.3.1102".to_string();
    info!("version:{0}", version);

    let mut socket = SubSocket::new();
    socket.connect("tcp://127.0.0.1:7899").await?;
    socket.subscribe("topic").await?;

    loop {
        let msg = socket.recv().await?;
        info!(
            "Subscribe: {:?}",
            msg.iter()
                .map(|item| std::str::from_utf8(item).unwrap_or("invalid text"))
                .collect::<Vec<&str>>()
        );
    }
}
