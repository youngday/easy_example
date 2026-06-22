#![allow(dead_code)]

use std::env;
use std::path::PathBuf;
use serde::Deserialize;
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::{fmt, prelude::*, EnvFilter, Registry};

#[derive(Deserialize)]
#[allow(dead_code)]

#[derive(Debug)]//🔑 add for debug user data
struct FinData {
    date: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
    adjusted: f64,
    dn: f64,
    mavg: f64,
    up: f64,
    direction: String,
}

fn load_apple_data() -> Vec<FinData> {
    let mut p = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());//TODO:could not debug by lldb,cargo run it.
    println!("path:{:?}",p);
    p = p.join("examples/assets").join("finance_charts_apple.csv");
    let mut rdr = csv::Reader::from_path(p).unwrap();
    let mut out = Vec::new();
    for result in rdr.deserialize() {
        let d: FinData = result.unwrap();
        out.push(d);
    }

    out
}

fn main() {
    // tracing subscriber 初始化
    let file_appender = tracing_appender::rolling::daily("examples/logs", "load_csv.log");
    Registry::default()
        .with(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .with(fmt::layer().pretty().with_line_number(true))
        .with(fmt::layer().json().with_writer(file_appender))
        .init();

    let data = load_apple_data();
    let date: Vec<String> = data.iter().map(|d| d.date.clone()).collect();
    let high: Vec<f64> = data.iter().map(|d| d.high).collect();

    info!("date count: {}", date.len());
    info!("high first: {:?}", high.first());
}