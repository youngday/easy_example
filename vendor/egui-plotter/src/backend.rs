//! Plotter backend for egui

use std::error::Error as ErrorTrait;
use std::f32::consts::FRAC_PI_2;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Add, AddAssign, MulAssign, Sub, SubAssign};

use egui::load::SizedTexture;
use egui::{pos2, vec2, Painter, TextureId, Vec2};
use egui::{
    epaint::{PathShape, TextShape},
    Align, Align2, Color32, FontFamily as EguiFontFamily, FontId, Pos2, Rect, Stroke, Ui, CornerRadius,
};
use plotters_backend::{
    text_anchor::{HPos, Pos, VPos},
    BackendColor, BackendCoord, BackendStyle, BackendTextStyle, DrawingBackend, DrawingErrorKind,
    FontFamily as PlottersFontFamily,
};

#[derive(Debug, Clone, Copy)]
/// Error to be returned by the backend. Since egui doesn't return any errors
/// on any painter operations, this is a stub type.
pub struct EguiBackendError;

impl Display for EguiBackendError {
    #[inline]
    fn fmt(&self, _f: &mut Formatter<'_>) -> FmtResult {
        Ok(())
    }
}

impl ErrorTrait for EguiBackendError {
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone, Copy)]
/// Struct used to convert between egui's Pos2 type and plotter's coordinate tuple.
/// Also provides implementations for adding coordinates together.
struct EguiBackendCoord {
    x: f32,
    y: f32,
}

impl From<(i32, i32)> for EguiBackendCoord {
    #[inline]
    fn from(value: (i32, i32)) -> Self {
        let (x, y) = value;

        Self {
            x: x as f32,
            y: y as f32,
        }
    }
}

impl From<EguiBackendCoord> for Pos2 {
    #[inline]
    fn from(val: EguiBackendCoord) -> Self {
        Pos2 { x: val.x, y: val.y }
    }
}

impl From<EguiBackendCoord> for (u32, u32) {
    #[inline]
    fn from(val: EguiBackendCoord) -> Self {
        (val.x as u32, val.y as u32)
    }
}

impl From<Pos2> for EguiBackendCoord {
    #[inline]
    fn from(value: Pos2) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl Add<EguiBackendCoord> for EguiBackendCoord {
    type Output = EguiBackendCoord;

    #[inline]
    fn add(self, rhs: EguiBackendCoord) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<EguiBackendCoord> for EguiBackendCoord {
    type Output = EguiBackendCoord;

    #[inline]
    fn sub(self, rhs: EguiBackendCoord) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add<Pos2> for EguiBackendCoord {
    type Output = EguiBackendCoord;

    #[inline]
    fn add(self, rhs: Pos2) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<f32> for EguiBackendCoord {
    type Output = EguiBackendCoord;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl AddAssign<EguiBackendCoord> for EguiBackendCoord {
    fn add_assign(&mut self, rhs: EguiBackendCoord) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign<EguiBackendCoord> for EguiBackendCoord {
    fn sub_assign(&mut self, rhs: EguiBackendCoord) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl MulAssign<f32> for EguiBackendCoord {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

#[derive(Debug, Clone, Copy)]
/// Struct used to convert between Egui and Plotter's color types
struct EguiBackendColor {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl From<BackendColor> for EguiBackendColor {
    #[inline]
    fn from(value: BackendColor) -> Self {
        let (r, g, b) = value.rgb;

        let a = (value.alpha * 255.0) as u8;

        Self { r, g, b, a }
    }
}

impl From<EguiBackendColor> for Color32 {
    #[inline]
    fn from(val: EguiBackendColor) -> Self {
        Color32::from_rgba_unmultiplied(val.r, val.g, val.b, val.a)
    }
}

/// What size to draw the image at
pub enum BgImageSize {
    /// Stretch the image to fit to the corners of the chart
    Fill,
    /// Scale the image up at a fixed ratio, W:H
    Ratio(f32, f32),
    /// Scale the image to fit the chart but not deviate from it's original aspect ratio.
    Fit,
    /// Do not scale the image at all
    Original,
    /// Set the image size to an exact value, W*H
    Exact(f32, f32),
}

impl BgImageSize {
    fn size_from_bounds(self, bounds: Rect, ui: &Ui, texture: TextureId) -> Rect {
        match self {
            Self::Fill => bounds,
            Self::Ratio(x, y) => {
                let bounds_width = bounds.width();
                let bounds_height = bounds.height();

                if bounds_width == 0.0 || bounds_height == 0.0 || x == 0.0 || y == 0.0 {
                    return bounds;
                }

                let size_max = bounds_width.max(bounds_height);

                let normalized_ratio = vec2(x, y).normalized();

                let img_wh_max = normalized_ratio * size_max;

                let scale = (bounds_width / img_wh_max.x).min(bounds_height / img_wh_max.y);

                let image_wh = img_wh_max * scale;
                
                Rect::from_center_size(bounds.center(), image_wh)
            }
            Self::Fit => {
                let original_bounds = Self::Original.size_from_bounds(bounds, ui, texture);

                let width = original_bounds.width();
                let height = original_bounds.height();

                Self::Ratio(width, height).size_from_bounds(bounds, ui, texture)
            },
            Self::Original => {
                let tex_manager_rwlock = ui.ctx().tex_manager();
                let tex_manager_rwlock_reader = tex_manager_rwlock.read();
                let tex_meta_result = tex_manager_rwlock_reader.meta(texture);

                let (width, height) = match tex_meta_result {
                    Some(meta) => {
                        let [width, height] = meta.size;

                        (width as f32, height as f32)
                    },
                    None => {
                        return bounds;
                    }
                };

                Rect::from_center_size(bounds.center(), vec2(width, height))
            }
            Self::Exact(w, h) => Rect::from_center_size(bounds.center(), vec2(w, h)),
        }
    }
}

/// Plotter backend for egui; simply provide a reference to the ui element to
/// use.
pub struct EguiBackend<'a> {
    ui: &'a Ui,
    x: i32,
    y: i32,
    scale: f32,
    bounds: Rect,
    painter: Painter,
}

impl<'a> EguiBackend<'a> {
    #[inline]
    /// Create a backend given a reference to a Ui.
    pub fn new(ui: &'a Ui) -> Self {
        let bounds = ui.max_rect();

        Self {
            ui,
            x: 0,
            y: 0,
            scale: 1.0,
            bounds,
            painter: ui.painter().with_clip_rect(bounds)
        }
    }

    /// Draw a background image for your chart given a texture id.
    ///
    /// The image is drawn when this fuction is called.
    pub fn bg_image(self, image_id: TextureId, size: BgImageSize) -> Self {
        let uv = Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));
        let image_bounds = size.size_from_bounds(self.bounds, self.ui, image_id);
        self.painter.image(image_id, image_bounds, uv, Color32::WHITE);

        self
    }

    #[inline]
    /// Transform point
    fn point_transform(&self, mut point: EguiBackendCoord, bounds: Rect) -> EguiBackendCoord {
        let center = EguiBackendCoord::from(bounds.center()) - EguiBackendCoord::from(bounds.min);
        point -= center;
        point *= self.scale;
        point += center;

        point += EguiBackendCoord::from((self.x, self.y));
        point += EguiBackendCoord::from(bounds.min);

        point
    }
    #[inline]
    /// Set the offset(x + y) of the backend.
    pub fn set_offset(&mut self, offset: (i32, i32)) {
        (self.x, self.y) = offset
    }

    #[inline]
    /// Set the offset(x + y) of the backend. Consumes self.
    pub fn offset(mut self, offset: (i32, i32)) -> Self {
        self.set_offset(offset);

        self
    }

    #[inline]
    /// Set the scale of the backend.
    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale
    }

    #[inline]
    /// Set the scale of the backend. Consume self.
    pub fn scale(mut self, scale: f32) -> Self {
        self.set_scale(scale);

        self
    }
}

impl<'a> DrawingBackend for EguiBackend<'a> {
    type ErrorType = std::io::Error;

    fn get_size(&self) -> (u32, u32) {
        let bounds = self.ui.max_rect();
        (bounds.width() as u32, bounds.height() as u32)
    }

    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn draw_pixel(
        &mut self,
        point: (i32, i32),
        color: BackendColor,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let p0 = self.point_transform(EguiBackendCoord::from(point), self.bounds);

        let p1 = p0 + 1.0;

        let color: Color32 = EguiBackendColor::from(color).into();

        let stroke = Stroke::new(1.0, color);

        self.painter.line_segment([p0.into(), p1.into()], stroke);

        Ok(())
    }

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: (i32, i32),
        to: (i32, i32),
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let p0 = self.point_transform(EguiBackendCoord::from(from), self.bounds);
        let p1 = self.point_transform(EguiBackendCoord::from(to), self.bounds);

        let color: Color32 = EguiBackendColor::from(style.color()).into();

        let stroke = Stroke::new(style.stroke_width() as f32, color);

        self.painter.line_segment([p0.into(), p1.into()], stroke);

        Ok(())
    }

    fn draw_rect<S: BackendStyle>(
        &mut self,
        upper_left: BackendCoord,
        bottom_right: BackendCoord,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let p0 = self.point_transform(EguiBackendCoord::from(upper_left), self.bounds);
        let p1 = self.point_transform(EguiBackendCoord::from(bottom_right), self.bounds);
        let color: Color32 = EguiBackendColor::from(style.color()).into();
        if fill {
            self.painter.rect_filled(
                egui::Rect {
                    min: p0.into(),
                    max: p1.into(),
                },
                CornerRadius::default(),
                color,
            );
        } else {
            let stroke = Stroke::new(style.stroke_width() as f32, color);
            self.painter.rect(
                egui::Rect {
                    min: p0.into(),
                    max: p1.into(),
                },
                CornerRadius::default(),
                Color32::TRANSPARENT,
                stroke,
                egui::StrokeKind::Inside
            );
        }

        Ok(())
    }

    fn draw_path<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        path: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let points: Vec<Pos2> = path
            .into_iter()
            .map(|point| {
                let point = self.point_transform(EguiBackendCoord::from(point), self.bounds);

                point.into()
            })
            .collect();

        let color: Color32 = EguiBackendColor::from(style.color()).into();

        let stroke = Stroke::new(style.stroke_width() as f32, color);

        let shape = PathShape::line(points, stroke);

        self.painter.add(shape);
        Ok(())
    }

    fn draw_circle<S: BackendStyle>(
        &mut self,
        center: BackendCoord,
        radius: u32,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let center = self.point_transform(EguiBackendCoord::from(center), self.bounds);
        let color: Color32 = EguiBackendColor::from(style.color()).into();
        if fill {
            self.painter.circle_filled(center.into(), radius as _, color);
        } else {
            let stroke = Stroke::new(style.stroke_width() as f32, color);
            self.painter.circle(center.into(), radius as _, Color32::TRANSPARENT, stroke);
        }

        Ok(())
    }

    fn fill_polygon<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        vert: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let points: Vec<Pos2> = vert
            .into_iter()
            .map(|point| {
                let point = self.point_transform(EguiBackendCoord::from(point), self.bounds);

                point.into()
            })
            .collect();

        let color: Color32 = EguiBackendColor::from(style.color()).into();

        let stroke = Stroke::NONE;

        let shape = PathShape::convex_polygon(points, color, stroke);

        self.painter.add(shape);

        Ok(())
    }

    fn draw_text<TStyle: BackendTextStyle>(
        &mut self,
        text: &str,
        style: &TStyle,
        pos: (i32, i32),
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let pos = self.point_transform(EguiBackendCoord::from(pos), self.bounds);

        let font_size = style.size() as f32;
        let font_family = match style.family() {
            PlottersFontFamily::Serif | PlottersFontFamily::SansSerif => {
                EguiFontFamily::Proportional
            }
            PlottersFontFamily::Monospace => EguiFontFamily::Monospace,
            PlottersFontFamily::Name(string) => EguiFontFamily::Name(string.into()),
        };

        let font = FontId {
            size: font_size,
            family: font_family,
        };

        let color: Color32 = EguiBackendColor::from(style.color()).into();

        let rotations = style.transform() as usize;
        let angle = rotations as f32 * FRAC_PI_2;

        let Pos { h_pos, v_pos } = style.anchor();

        // !TODO! Find a slightly more eligant rotation function.
        let mut anchor = Align2([
            match h_pos {
                HPos::Left => Align::LEFT,
                HPos::Right => Align::RIGHT,
                HPos::Center => Align::Center,
            },
            match v_pos {
                VPos::Top => Align::TOP,
                VPos::Center => Align::Center,
                VPos::Bottom => Align::BOTTOM,
            },
        ]);
        fn rotate(anchor: &mut Align2) {
            *anchor = match anchor {
                &mut Align2::LEFT_TOP => Align2::RIGHT_TOP,
                &mut Align2::RIGHT_TOP => Align2::RIGHT_BOTTOM,
                &mut Align2::RIGHT_BOTTOM => Align2::LEFT_BOTTOM,
                &mut Align2::LEFT_BOTTOM => Align2::LEFT_TOP,
                &mut Align2::LEFT_CENTER => Align2::CENTER_TOP,
                &mut Align2::CENTER_TOP => Align2::RIGHT_CENTER,
                &mut Align2::RIGHT_CENTER => Align2::CENTER_BOTTOM,
                &mut Align2::CENTER_BOTTOM => Align2::LEFT_CENTER,
                &mut Align2::CENTER_CENTER => Align2::CENTER_CENTER,
            }
        }
        for _ in 0..rotations {
            rotate(&mut anchor)
        }
        let galley = self.painter.layout_no_wrap(text.to_string(), font, color);
        let rect = anchor.anchor_rect(Rect::from_min_size(pos.into(), galley.size()));
        if !galley.is_empty() {
            self.painter.add(TextShape {
                angle,
                ..TextShape::new(rect.min, galley, Color32::PLACEHOLDER)
            });
        }

        Ok(())
    }
}
