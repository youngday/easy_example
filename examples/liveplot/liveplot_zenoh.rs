//! Live plotting example: zenoh subscriber -> `liveplot` UI.
//!
//! The [`liveplot`](https://github.com/ulikoehler/liveplot-rs) analogue of
//! `egui_timechart_live`: same pub/sub stream, but the live view is rendered by
//! the `liveplot` crate (egui/eframe) instead of the vendored `egui-plotter`.
//! Samples are pushed through liveplot's [`PlotSink`] — a plain
//! `std::sync::mpsc` channel — and [`run_liveplot`] drives the window.
//!
//! Every received [`TransmissionData`] becomes one point: the sample arrival
//! time (wall-clock seconds) on the X axis, the `y` field on the Y axis. The
//! plot keeps a rolling time window (default 10 s, also adjustable in the UI).
//!
//! The zenoh session is async (tokio) while liveplot owns the main thread, so
//! the subscriber runs on a background thread. Closing the window drops the
//! channel's receiver, `send_point` then fails, and the thread ends.
//!
//! Run the publisher from another terminal first:
//!
//! ```sh
//! cargo run --example zenoh_pub
//! cargo run --example liveplot_zenoh
//! ```

use std::time::{SystemTime, UNIX_EPOCH};

use easy_example::TransmissionData;
use liveplot::{channel_plot, run_liveplot, LivePlotConfig, PlotPoint, PlotSink, Trace};

const KEY_EXPR: &str = "demo/easy_example/**";
/// Rolling time window shown by the plot, in seconds.
const WINDOW_SECS: f64 = 10.0;

fn main() -> eframe::Result<()> {
    let (sink, rx) = channel_plot();

    // One trace per field worth watching. Add more `create_trace` calls here and
    // more `send_point` calls below to overlay additional signals.
    let trace_y = sink.create_trace("y", Some("TransmissionData.y"));

    spawn_zenoh_subscriber(sink, trace_y);

    let config = LivePlotConfig {
        title: "Live zenoh example (liveplot)".to_string(),
        time_window_secs: WINDOW_SECS,
        ..Default::default()
    };

    // Blocks until the window is closed.
    run_liveplot(rx, config)
}

/// Runs a zenoh subscriber on a background thread and pushes every decoded
/// sample into the liveplot channel.
fn spawn_zenoh_subscriber(sink: PlotSink, trace: Trace) {
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().expect("build tokio runtime");
        runtime.block_on(async move {
            let session = zenoh::open(zenoh::Config::default())
                .await
                .expect("open zenoh session");
            let subscriber = session
                .declare_subscriber(KEY_EXPR)
                .await
                .expect("declare zenoh subscriber");
            println!("zenoh subscriber declared on `{KEY_EXPR}`");

            while let Ok(sample) = subscriber.recv_async().await {
                let bytes = sample.payload().to_bytes();
                match serde_json::from_slice::<TransmissionData>(bytes.as_ref()) {
                    Ok(data) => {
                        println!("received {data:?}");
                        let now = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .map(|d| d.as_secs_f64())
                            .unwrap_or(0.0);

                        // Stop when the UI is gone.
                        if sink
                            .send_point(&trace, PlotPoint {
                                x: now,
                                y: data.y as f64,
                            })
                            .is_err()
                        {
                            break;
                        }
                    }
                    Err(err) => eprintln!("dropping malformed payload: {err}"),
                }
            }
        });
    });
}
