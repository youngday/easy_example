use base64::{Engine as _, engine::general_purpose};

use tracing::{info, debug, trace, warn, instrument};
use tracing_subscriber::{fmt, prelude::*, EnvFilter, Registry};

#[instrument]  // 自动记录函数入参、返回值、耗时
fn do_encode(data: &[u8]) -> String {
    info!(len = data.len(), "encoding");
    general_purpose::STANDARD.encode(data)
}

#[instrument]
fn do_decode(b64: &str) -> Vec<u8> {
    info!(len = b64.len(), "decoding");
    general_purpose::STANDARD.decode(b64).unwrap()
}

fn main() {
    // 控制台 appender（带颜色、行号）
    let console_layer = fmt::layer()
        .pretty()
        .with_line_number(true);

    // 文件 appender（JSON 格式，每天滚动）
    let file_appender = tracing_appender::rolling::daily("examples/logs", "base64.log");
    let file_layer = fmt::layer()
        .json()
        .with_writer(file_appender);

    // 订阅：控制台 + 文件，可通过 RUST_LOG 环境变量覆盖级别
    Registry::default()
        .with(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .with(console_layer)
        .with(file_layer)
        .init();

    info!("Base64 testing.");

    // 示例：其他日志级别
    trace!("这是 trace 日志");
    debug!("这是 debug 日志");
    info!("这是 info 日志");
    warn!("这是 warn 日志");

    let a = b"hello world";
    let b = "aGVsbG8gd29ybGQ=";

    // ✅ tracing 的结构化字段语法（自动关联到 JSON 文件）
    info!(bytes = ?a, "原始字节");
    info!(base64 = b, "Base64 字符串");

    let encoded = do_encode(a);
    info!(encoded = %encoded, "encode 结果");

    let decoded = do_decode(b);
    info!(decoded = ?decoded, "decode 结果");

    // 断言
    assert_eq!(encoded, b);
    assert_eq!(a, decoded.as_slice());

    debug!("所有断言通过，完成");
}