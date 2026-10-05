#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

mod backend;
mod chart;
pub mod charts;

pub use backend::{EguiBackend, EguiBackendError, BgImageSize};
pub use chart::{
    Chart, MouseButton, MouseConfig, Transform, DEFAULT_MOVE_SCALE, DEFAULT_SCROLL_SCALE,
};
use egui::{Response, Sense, Ui, UiBuilder, Widget};

#[cfg(feature = "timechart")]
use std::ops::Range;

#[cfg(feature = "timechart")]
fn mult_range(range: Range<f32>, mult: f32) -> Range<f32> {
    let delta = range.end - range.start;

    let half_delta = delta / 2.0;

    let midpoint = range.end - half_delta;

    let adjusted_delta = half_delta * mult;

    let start = midpoint - adjusted_delta;
    let end = midpoint + adjusted_delta;

    Range { start, end }
}

pub struct PlotterCanvas <CB: FnOnce(&mut Ui) -> R, R> {
    callback: CB,
    width: Option<f32>,
    height: Option<f32>,
}

impl<CB: FnOnce(&mut Ui) -> R, R> PlotterCanvas<CB, R> {
    pub fn with_cb(callback: CB) -> Self {
        Self {
            callback,
            width: None,
            height: None,
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);

        self
    }
    
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);

        self
    }

    pub fn set_width(&mut self, width: f32) {
        self.width = Some(width);
    }
    
    pub fn set_height(&mut self, height: f32) {
        self.height = Some(height);
    }
}

impl<CB: FnOnce(&mut Ui) -> R, R> Widget for PlotterCanvas<CB, R> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let mut bounds = ui.max_rect();

        if let Some(width) = self.width {
            bounds.set_width(width);
        }

        if let Some(height) = self.height {
            bounds.set_height(height);
        }

        let ui_builder = UiBuilder::new().max_rect(bounds);

        ui.scope_builder(ui_builder, self.callback).response
    }
}
