use std::f32::consts::{FRAC_PI_2, TAU};

use egui::{vec2, Color32, Mesh, Pos2, Rect, Response, Sense, Shape, Ui, Vec2, Widget};
use egui_twoui_tokens::{theme_from_ui, tokens::colors, TwoUiPalette};

use super::{cubic_bezier, gradient_color_at, gradient_sweep_phase};

/// Spinner size presets (`SmallTitle`, `Small`, `Medium`, `Large`, `XLarge`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpinnerSize {
    /// Compact `16dp` title bar size (`2.3dp` stroke).
    SmallTitle,
    Small,
    #[default]
    Medium,
    Large,
    XLarge,
}

impl SpinnerSize {
    pub fn diameter(self) -> f32 {
        match self {
            Self::SmallTitle => 16.0,
            Self::Small => 24.0,
            Self::Medium => 42.0,
            Self::Large => 60.0,
            Self::XLarge => 75.0,
        }
    }

    /// Ring stroke widths matched to each preset diameter.
    pub fn stroke_width(self) -> f32 {
        match self {
            Self::SmallTitle => 2.3,
            Self::Small => 3.5,
            Self::Medium => 5.6,
            Self::Large => 8.8,
            Self::XLarge => 10.9,
        }
    }

    pub fn circle_padding(self) -> f32 {
        match self {
            Self::SmallTitle => 0.7,
            Self::Small => 1.0,
            Self::Medium => 1.6,
            Self::Large => 2.5,
            Self::XLarge => 3.1,
        }
    }
}

/// Circular progress ring and 4-dot indeterminate spinner.
pub struct TwoUiSpinner {
    size: SpinnerSize,
    progress: Option<f32>,
    gradient: bool,
}

impl TwoUiSpinner {
    /// Creates an indeterminate 4-dot circular loader.
    pub fn indeterminate(size: SpinnerSize) -> Self {
        Self {
            size,
            progress: None,
            gradient: false,
        }
    }

    /// Creates a determinate circular progress ring (`0.0..=1.0`).
    pub fn determinate(size: SpinnerSize, progress: f32) -> Self {
        Self {
            size,
            progress: Some(progress.clamp(0.0, 1.0)),
            gradient: false,
        }
    }

    /// Enables rotating sweep-gradient arc on determinate progress rings.
    pub fn gradient(mut self, enabled: bool) -> Self {
        self.gradient = enabled;
        self
    }

    /// Paints the 4-dot orbiting/converging loader centered inside `rect`.
    pub fn paint_indeterminate_at(
        ui: &Ui,
        rect: Rect,
        _stroke_width: f32,
        palette: &TwoUiPalette,
    ) {
        let time = ui.input(|i| i.time) as f32;
        ui.ctx().request_repaint();

        let center = rect.center();
        let d = rect.width().min(rect.height()).max(8.0);
        let ms = (time % 2.0) * 1000.0;
        let rot_angle = (ms / 2000.0) * (2.0 * TAU);

        let (norm_dist, cluster_scale, alpha_02, alpha_13) = if ms < 633.0 {
            let u = ms / 633.0;
            let dist = egui::lerp(0.35_f32..=0.1667, cubic_bezier(0.497, 0.0, 0.833, 0.571, u));
            (dist, 1.0_f32, 1.0_f32, 1.0_f32)
        } else if ms < 1099.0 {
            let u = (ms - 633.0) / 466.0;
            let dist = egui::lerp(0.1667_f32..=0.0208, cubic_bezier(0.344, 0.830, 0.605, 1.0, u));
            let sc = egui::lerp(1.0_f32..=2.0, cubic_bezier(0.499, 0.0, 0.765, 1.0, u));
            let a02 = egui::lerp(1.0_f32..=0.45, cubic_bezier(0.497, 0.763, 0.605, 1.0, u));
            let a13 = egui::lerp(1.0_f32..=0.70, cubic_bezier(0.497, 1.399, 0.605, 1.0, u));
            (dist, sc, a02.clamp(0.25, 1.0), a13.clamp(0.4, 1.0))
        } else if ms < 1799.0 {
            let u = (ms - 1099.0) / 700.0;
            let dist = egui::lerp(0.0208_f32..=0.35, cubic_bezier(0.117, 0.126, 0.312, 1.0, u));
            let sc = egui::lerp(2.0_f32..=1.0, cubic_bezier(0.091, 0.0, 0.0, 1.0, u));
            let a02 = egui::lerp(0.45_f32..=1.0, cubic_bezier(0.497, 0.763, 0.605, 1.0, u));
            let a13 = egui::lerp(0.70_f32..=1.0, cubic_bezier(0.497, 1.399, 0.605, 1.0, u));
            (dist, sc, a02.clamp(0.25, 1.0), a13.clamp(0.4, 1.0))
        } else {
            (0.35_f32, 1.0_f32, 1.0_f32, 1.0_f32)
        };

        let base_dot_r = (d * 0.112).clamp(1.4, 10.5);
        let dot_r = base_dot_r * cluster_scale;
        let orbit_r = (norm_dist * d * cluster_scale).min(d * 0.5 - dot_r * 0.5);

        let dots = [
            (palette.primary, alpha_02),
            (Color32::from_rgb(0x3B, 0xA3, 0xC3), alpha_13),
            (Color32::from_rgb(0x3C, 0xB9, 0xA2), alpha_02),
            (colors::PROGRESS_TEAL, alpha_13),
        ];

        for (i, (col, dot_alpha)) in dots.into_iter().enumerate() {
            let angle = rot_angle + (i as f32) * FRAC_PI_2;
            let pos = center + vec2(angle.cos(), angle.sin()) * orbit_r;
            ui.painter()
                .circle_filled(pos, dot_r, col.gamma_multiply(dot_alpha));
        }
    }
}

impl Widget for TwoUiSpinner {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let d = self.size.diameter();
        let sw = self.size.stroke_width();
        let pad = self.size.circle_padding();
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(d), Sense::hover());

        if ui.is_rect_visible(rect) {
            match self.progress {
                None => Self::paint_indeterminate_at(ui, rect, sw, p),
                Some(val) => {
                    let vis_val = ui
                        .ctx()
                        .animate_value_with_time(response.id.with("det_prog"), val, 0.08)
                        .clamp(0.0, 1.0);
                    let center = rect.center();
                    let radius = ((d - sw) * 0.5 - pad).max(2.0);
                    paint_annulus_arc_mesh(
                        ui,
                        center,
                        radius,
                        sw,
                        -FRAC_PI_2,
                        TAU,
                        |_| p.progress_track,
                        false,
                    );
                    if vis_val > 0.005 {
                        let start = -FRAC_PI_2;
                        let sweep = vis_val * TAU;
                        let round_caps = sweep < TAU - 0.004;
                        if self.gradient {
                            let time = ui.input(|i| i.time) as f32;
                            ui.ctx().request_repaint();
                            let rot = gradient_sweep_phase(time);
                            paint_annulus_arc_mesh(
                                ui,
                                center,
                                radius,
                                sw,
                                start,
                                sweep,
                                |t| gradient_color_at(t * vis_val - rot, p.primary),
                                round_caps,
                            );
                        } else {
                            paint_annulus_arc_mesh(
                                ui,
                                center,
                                radius,
                                sw,
                                start,
                                sweep,
                                |_| p.primary,
                                round_caps,
                            );
                        }
                    }
                }
            }
        }

        response
    }
}

#[allow(clippy::too_many_arguments)]
fn paint_annulus_arc_mesh(
    ui: &Ui,
    center: Pos2,
    radius: f32,
    stroke_width: f32,
    start_angle: f32,
    sweep_angle: f32,
    color_fn: impl Fn(f32) -> Color32,
    round_caps: bool,
) {
    if sweep_angle.abs() <= 0.001 || stroke_width <= 0.1 || radius <= 0.5 {
        return;
    }

    let half_sw = stroke_width * 0.5;
    let r_in = (radius - half_sw).max(0.0);
    let r_out = radius + half_sw;
    let feather_in = (half_sw * 0.28).min(0.35);
    let feather_out = 0.45_f32;
    let r0 = (r_in - feather_out).max(0.0);
    let r1 = r_in + feather_in;
    let r2 = (r_out - feather_in).max(r1 + 0.05);
    let r3 = r_out + feather_out;

    let steps = ((sweep_angle.abs() / TAU) * 96.0)
        .ceil()
        .clamp(8.0, 96.0) as usize;

    let mut mesh = Mesh::default();
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let a = start_angle + sweep_angle * t;
        let u = vec2(a.cos(), a.sin());
        let col = color_fn(t);

        mesh.colored_vertex(center + u * r0, Color32::TRANSPARENT);
        mesh.colored_vertex(center + u * r1, col);
        mesh.colored_vertex(center + u * r2, col);
        mesh.colored_vertex(center + u * r3, Color32::TRANSPARENT);
    }

    for i in 0..steps {
        let a = (i * 4) as u32;
        let b = ((i + 1) * 4) as u32;
        mesh.add_triangle(a, a + 1, b);
        mesh.add_triangle(a + 1, b + 1, b);
        mesh.add_triangle(a + 1, a + 2, b + 1);
        mesh.add_triangle(a + 2, b + 2, b + 1);
        mesh.add_triangle(a + 2, a + 3, b + 2);
        mesh.add_triangle(a + 3, b + 3, b + 2);
    }

    if round_caps {
        let sign = sweep_angle.signum();
        let cap_core = (half_sw - feather_in).max(0.05);
        let cap_aa = half_sw + feather_out;
        paint_annulus_cap(
            &mut mesh,
            center,
            radius,
            start_angle,
            -sign,
            cap_core,
            cap_aa,
            color_fn(0.0),
        );
        paint_annulus_cap(
            &mut mesh,
            center,
            radius,
            start_angle + sweep_angle,
            sign,
            cap_core,
            cap_aa,
            color_fn(1.0),
        );
    }

    ui.painter().add(Shape::mesh(mesh));
}

#[allow(clippy::too_many_arguments)]
fn paint_annulus_cap(
    mesh: &mut Mesh,
    center: Pos2,
    radius: f32,
    angle: f32,
    outward_tan_sign: f32,
    cap_core: f32,
    cap_aa: f32,
    col: Color32,
) {
    const CAP_STEPS: usize = 10;
    let rad = vec2(angle.cos(), angle.sin());
    let tan = vec2(-angle.sin(), angle.cos()) * outward_tan_sign;
    let cap_center = center + rad * radius;

    let center_idx = mesh.vertices.len() as u32;
    mesh.colored_vertex(cap_center, col);

    for k in 0..=CAP_STEPS {
        let theta = -FRAC_PI_2 + std::f32::consts::PI * (k as f32 / CAP_STEPS as f32);
        let dir = tan * theta.cos() + rad * theta.sin();
        mesh.colored_vertex(cap_center + dir * cap_core, col);
        mesh.colored_vertex(cap_center + dir * cap_aa, Color32::TRANSPARENT);
    }

    for k in 0..CAP_STEPS {
        let v0 = center_idx + 1 + (k * 2) as u32;
        let v1 = v0 + 2;
        mesh.add_triangle(center_idx, v0, v1);
        mesh.add_triangle(v0, v0 + 1, v1);
        mesh.add_triangle(v0 + 1, v1 + 1, v1);
    }
}
