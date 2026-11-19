use std::f32::consts::FRAC_PI_2;

use egui::{pos2, vec2, Color32, CornerRadius, Mesh, Rect, Response, Sense, Shape, Ui, Widget};
use egui_twoui_tokens::{theme_from_ui, tokens::colors};

use super::{cubic_bezier, gradient_color_at, gradient_sweep_phase};

/// Horizontal progress bar (`5dp` default height).
pub struct TwoUiProgressBar {
    progress: Option<f32>,
    height: f32,
    gradient: bool,
}

impl TwoUiProgressBar {
    /// Creates a determinate progress bar (`0.0..=1.0`).
    pub fn determinate(progress: f32) -> Self {
        Self {
            progress: Some(progress.clamp(0.0, 1.0)),
            height: 5.0,
            gradient: false,
        }
    }

    /// Creates an indeterminate animated progress bar.
    pub fn indeterminate() -> Self {
        Self {
            progress: None,
            height: 5.0,
            gradient: false,
        }
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height.max(3.0);
        self
    }

    /// Enables animated linear gradient fill on determinate bars.
    pub fn gradient(mut self, enabled: bool) -> Self {
        self.gradient = enabled;
        self
    }
}

impl Widget for TwoUiProgressBar {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let width = ui.available_width().max(80.0);
        let (rect, response) =
            ui.allocate_exact_size(vec2(width, self.height.max(10.0)), Sense::hover());

        if ui.is_rect_visible(rect) {
            let cy = rect.center().y;
            let bar_rect = Rect::from_min_max(
                pos2(rect.left(), cy - self.height * 0.5),
                pos2(rect.right(), cy + self.height * 0.5),
            );
            let radius = CornerRadius::same((self.height * 0.5).ceil() as u8);
            ui.painter().rect_filled(bar_rect, radius, p.progress_track);

            match self.progress {
                Some(val) => {
                    let vis_val = ui
                        .ctx()
                        .animate_value_with_time(response.id.with("det_prog"), val, 0.08)
                        .clamp(0.0, 1.0);
                    if vis_val > 0.005 {
                        let fill_w = (bar_rect.width() * vis_val).max(self.height);
                        let fill_rect = Rect::from_min_size(
                            bar_rect.min,
                            vec2(fill_w.min(bar_rect.width()), self.height),
                        );
                        if self.gradient {
                            let time = ui.input(|i| i.time) as f32;
                            ui.ctx().request_repaint();
                            let phase = gradient_sweep_phase(time);
                            let ratio =
                                (fill_rect.width() / bar_rect.width().max(1.0)).clamp(0.0, 1.0);
                            paint_pill_mesh(ui, fill_rect, |u| {
                                gradient_color_at(u * ratio - phase, p.primary)
                            });
                        } else {
                            ui.painter().rect_filled(fill_rect, radius, p.primary);
                        }
                    }
                }
                None => {
                    let time = ui.input(|i| i.time) as f32;
                    ui.ctx().request_repaint();
                    let raw = (time % 2.0) / 2.0;
                    let eased = cubic_bezier(0.540, 0.0, 0.380, 1.0, raw);
                    paint_pill_mesh(ui, bar_rect, |u| {
                        let g = 0.5 * u + 0.5 - eased;
                        v35_comet_color(g, p.primary)
                    });
                }
            }
        }

        response
    }
}

fn v35_comet_color(g: f32, primary: Color32) -> Color32 {
    if !(0.0..=0.50).contains(&g) {
        return Color32::TRANSPARENT;
    }
    let blue = primary;
    let light_cyan = Color32::from_rgb(0x3A, 0x9A, 0xD0);
    let teal = Color32::from_rgb(0x3B, 0xA3, 0xC3);
    let emerald = colors::PROGRESS_TEAL;

    let stops = [
        (0.000_f32, blue.gamma_multiply(0.0)),
        (0.061, light_cyan.gamma_multiply(0.14)),
        (0.100, emerald.gamma_multiply(0.62)),
        (0.150, teal),
        (0.200, blue),
        (0.350, teal),
        (0.500, emerald.gamma_multiply(0.0)),
    ];
    for w in stops.windows(2) {
        let (t0, c0) = w[0];
        let (t1, c1) = w[1];
        if g <= t1 {
            let u = ((g - t0) / (t1 - t0)).clamp(0.0, 1.0);
            return Color32::from_rgba_unmultiplied(
                lerp_u8(c0.r(), c1.r(), u),
                lerp_u8(c0.g(), c1.g(), u),
                lerp_u8(c0.b(), c1.b(), u),
                lerp_u8(c0.a(), c1.a(), u),
            );
        }
    }
    Color32::TRANSPARENT
}

fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    (f32::from(a) + (f32::from(b) - f32::from(a)) * t)
        .round()
        .clamp(0.0, 255.0) as u8
}

fn paint_pill_mesh(ui: &Ui, rect: Rect, color_fn: impl Fn(f32) -> Color32) {
    let w = rect.width();
    let h = rect.height();
    if w <= 0.5 || h <= 0.5 {
        return;
    }
    let r = (h * 0.5).min(w * 0.5);
    let cy = rect.center().y;
    let feather_in = (r * 0.25).min(0.35);
    let r_in = (r - feather_in).max(0.05);
    let r_out = r + 0.45;

    const CAP_STEPS: usize = 12;
    let body_w = (w - 2.0 * r).max(0.0);
    let mut slices: Vec<(f32, f32, f32)> = Vec::with_capacity(CAP_STEPS * 2 + 24);

    let cx_left = rect.left() + r;
    for i in 0..=CAP_STEPS {
        let a = (i as f32 / CAP_STEPS as f32) * FRAC_PI_2;
        slices.push((cx_left, -a.cos(), a.sin()));
    }

    if body_w > 0.25 {
        let body_steps = (body_w / 12.0).ceil().clamp(2.0, 48.0) as usize;
        for i in 1..body_steps {
            let t = i as f32 / body_steps as f32;
            slices.push((cx_left + body_w * t, 0.0, 1.0));
        }
    }

    let cx_right = rect.right() - r;
    let start_k = if body_w <= 0.25 { 1 } else { 0 };
    for i in start_k..=CAP_STEPS {
        let a = FRAC_PI_2 * (1.0 - i as f32 / CAP_STEPS as f32);
        slices.push((cx_right, a.cos(), a.sin()));
    }

    let mut mesh = Mesh::default();
    for &(sx, dx, dy) in &slices {
        let px = (sx + r * dx).clamp(rect.left(), rect.right());
        let u = ((px - rect.left()) / w).clamp(0.0, 1.0);
        let col = color_fn(u);
        mesh.colored_vertex(pos2(sx + r_out * dx, cy - r_out * dy), Color32::TRANSPARENT);
        mesh.colored_vertex(pos2(sx + r_in * dx, cy - r_in * dy), col);
        mesh.colored_vertex(pos2(sx + r_in * dx, cy + r_in * dy), col);
        mesh.colored_vertex(pos2(sx + r_out * dx, cy + r_out * dy), Color32::TRANSPARENT);
    }

    for i in 0..(slices.len().saturating_sub(1)) {
        let a = (i * 4) as u32;
        let b = ((i + 1) * 4) as u32;
        mesh.add_triangle(a, a + 1, b);
        mesh.add_triangle(a + 1, b + 1, b);
        mesh.add_triangle(a + 1, a + 2, b + 1);
        mesh.add_triangle(a + 2, b + 2, b + 1);
        mesh.add_triangle(a + 2, a + 3, b + 2);
        mesh.add_triangle(a + 3, b + 3, b + 2);
    }
    ui.painter().add(Shape::mesh(mesh));
}
