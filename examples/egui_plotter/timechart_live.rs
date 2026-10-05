//! Live plotting example: zenoh subscriber -> `egui_plotter::charts::TimeData`.
//!
//! Subscribes to `demo/easy_example/**` with zenoh and plots the `funky` field of
//! every received [`TransmissionData`] as it arrives, with the X axis being the
//! seconds elapsed since the application started.
//!
//! The zenoh session is async (tokio) while eframe drives a blocking native event
//! loop, so the subscriber runs on a background thread and hands samples to the UI
//! through a channel. Samples are appended with `TimeData::push` and the chart is
//! kept to a rolling window with `TimeData::set_window` (default 100 samples,
//! adjustable with the slider), so it scrolls instead of being rebuilt.
//!
//! Run the publisher from another terminal first:
//!
//! ```sh
//! cargo run --example zenoh_pub
//! cargo run --example egui_timechart_live
//! ```

use std::{
    sync::mpsc::{channel, Receiver, Sender},
    time::{Duration, Instant},
};

use easy_example::TransmissionData;
use eframe::egui::{self, CentralPanel, Slider, Visuals};
use egui_plotter::charts::TimeData;

const KEY_EXPR: &str = "demo/easy_example/**";
/// Default number of samples kept on screen.
const DEFAULT_WINDOW: usize = 100;
const MIN_WINDOW: usize = 10;
const MAX_WINDOW: usize = 1000;

fn main() {
    let (tx, rx) = channel::<TransmissionData>();
    spawn_zenoh_subscriber(tx);

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Live Zenoh TimeData Example",
        native_options,
        Box::new(|cc| Ok(Box::new(LiveTimeChart::new(cc, rx)))),
    )
    .unwrap();
}

/// Runs a zenoh subscriber on a background thread and forwards every decoded
/// `TransmissionData` to the UI thread through a channel.
fn spawn_zenoh_subscriber(tx: Sender<TransmissionData>) {
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
                        // Stop when the UI is gone.
                        if tx.send(data).is_err() {
                            break;
                        }
                    }
                    Err(err) => eprintln!("dropping malformed payload: {err}"),
                }
            }
        });
    });
}

struct LiveTimeChart {
    rx: Receiver<TransmissionData>,
    timechart: TimeData,
    start: Instant,
    received: u64,
    last: Option<TransmissionData>,
    window: usize,
}

impl LiveTimeChart {
    fn new(cc: &eframe::CreationContext<'_>, rx: Receiver<TransmissionData>) -> Self {
        // Enable light mode
        cc.egui_ctx.set_visuals(Visuals::light());

        let mut timechart = TimeData::empty("funky", "Live zenoh data");
        timechart.set_window(Some(DEFAULT_WINDOW));

        Self {
            rx,
            timechart,
            start: Instant::now(),
            received: 0,
            last: None,
            window: DEFAULT_WINDOW,
        }
    }

    /// Append every pending sample to the chart.
    fn ingest_new_samples(&mut self) {
        while let Ok(data) = self.rx.try_recv() {
            self.received += 1;
            let elapsed = self.start.elapsed().as_secs_f32();
            self.timechart.push(elapsed, data.funky as f32);
            self.last = Some(data);
        }
    }
}

impl eframe::App for LiveTimeChart {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.ingest_new_samples();

        let status = match &self.last {
            Some(data) => format!("received {} samples, last = {data:?}", self.received),
            None => format!("received 0 samples, waiting on `{KEY_EXPR}` ..."),
        };

        CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(status);
                ui.separator();
                let mut window = self.window;
                if ui
                    .add(Slider::new(&mut window, MIN_WINDOW..=MAX_WINDOW).text("window"))
                    .changed()
                {
                    self.window = window;
                    self.timechart.set_window(Some(window));
                }
                ui.label(format!("{} pts", self.timechart.len()));
            });

            self.timechart.draw(ui);
        });

        // Keep refreshing so newly arrived samples show up promptly.
        ctx.request_repaint_after(Duration::from_millis(100));
    }
}
