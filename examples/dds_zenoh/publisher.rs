//! Zenoh publisher example.
//!
//! Publishes a [`TransmissionData`] sample every second on the key expression
//! `demo/easy_example/transmission`, serialized as JSON.
//!
//! Start the subscriber first, then this publisher (zenoh peers discover each
//! other automatically on the local network, so order does not really matter):
//!
//! ```sh
//! cargo run --example zenoh_sub
//! cargo run --example zenoh_pub
//! ```

use std::time::Duration;

use easy_example::TransmissionData;

const KEY_EXPR: &str = "demo/easy_example/transmission";
const CYCLE_TIME: Duration = Duration::from_secs(1);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let session = zenoh::open(zenoh::Config::default()).await?;
    let publisher = session.declare_publisher(KEY_EXPR).await?;

    println!("zenoh publisher declared on `{KEY_EXPR}`");

    let mut counter: u64 = 0;
    loop {
        counter += 1;

        let sample = TransmissionData {
            x: counter as i32,
            y: counter as i32 * 3,
            funky: counter as f64 * 812.12,
        };

        let payload = serde_json::to_vec(&sample)?;
        publisher.put(payload).await?;
        println!("send sample {counter}: {sample:?}");

        tokio::time::sleep(CYCLE_TIME).await;
    }
}
