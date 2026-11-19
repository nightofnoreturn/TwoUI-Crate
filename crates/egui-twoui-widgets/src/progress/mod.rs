//! Linear and circular progress indicators.
//!
//! Includes horizontal wave progress, 4-dot circular loader, determinate rings, and gradient sweeps.

mod linear;
mod spinner;

pub use linear::TwoUiProgressBar;
pub use spinner::{TwoUiSpinner, SpinnerSize};

use egui::Color32;
pub(crate) use egui_twoui_tokens::cubic_bezier;

/// Shared animated gradient phase (`2000ms` continuous forward sweep, `(0.3, 0.2, 0.7, 0.8)`).
pub(crate) fn gradient_sweep_phase(time: f32) -> f32 {
    cubic_bezier(0.3, 0.2, 0.7, 0.8, (time % 2.0) / 2.0)
}

/// Evaluates Catmull-Rom cubic-spline smoothed gradient color at `t`.
pub(crate) fn gradient_color_at(t: f32, _primary: Color32) -> Color32 {
    const STOPS: [(f32, [u8; 4]); 10] = [
        (0.00, [59, 163, 243, 210]),
        (0.03, [57, 140, 251, 215]),
        (0.07, [56, 122, 255, 220]),
        (0.20, [60, 185, 162, 240]),
        (0.38, [61, 204, 135, 245]),
        (0.58, [56, 122, 255, 245]),
        (0.85, [59, 163, 243, 215]),
        (0.96, [61, 204, 135, 215]),
        (0.98, [60, 180, 165, 215]),
        (1.00, [59, 163, 243, 210]),
    ];

    let t = t.rem_euclid(1.0);
    let mut seg = 0;
    while seg + 1 < STOPS.len() - 1 && t > STOPS[seg + 1].0 {
        seg += 1;
    }
    let (p1, c1) = STOPS[seg];
    let (p2, c2) = STOPS[seg + 1];
    let u = ((t - p1) / (p2 - p1).max(1e-5)).clamp(0.0, 1.0);
    let c0 = STOPS[seg.saturating_sub(1)].1;
    let c3 = STOPS[(seg + 2).min(STOPS.len() - 1)].1;

    let mut out = [0u8; 4];
    for ch in 0..4 {
        let v0 = f32::from(c0[ch]) / 255.0;
        let v1 = f32::from(c1[ch]) / 255.0;
        let v2 = f32::from(c2[ch]) / 255.0;
        let v3 = f32::from(c3[ch]) / 255.0;
        let u2 = u * u;
        let u3 = u2 * u;
        let crm = 0.5
            * ((2.0 * v1)
                + (-v0 + v2) * u
                + (2.0 * v0 - 5.0 * v1 + 4.0 * v2 - v3) * u2
                + (-v0 + 3.0 * v1 - 3.0 * v2 + v3) * u3);
        out[ch] = (crm.clamp(0.0, 1.0) * 255.0).round() as u8;
    }
    Color32::from_rgba_unmultiplied(out[0], out[1], out[2], out[3])
}
