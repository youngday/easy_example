//! Zenoh subscriber example.
//!
//! Subscribes to `demo/easy_example/**` and prints every [`TransmissionData`]
//! sample it receives. Payloads are JSON, matching `zenoh_pub`.
//!
//! ```sh
//! cargo run --example zenoh_sub
//! ```

use easy_example::TransmissionData;

const KEY_EXPR: &str = "demo/easy_example/**";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let session = zenoh::open(zenoh::Config::default()).await?;
    let subscriber = session.declare_subscriber(KEY_EXPR).await?;

    println!("zenoh subscriber declared on `{KEY_EXPR}`");

    while let Ok(sample) = subscriber.recv_async().await {
        let bytes = sample.payload().to_bytes();
        let data: TransmissionData = serde_json::from_slice(bytes.as_ref())?;
        println!("received on {}: {data:?}", sample.key_expr());
    }

    Ok(())
}
