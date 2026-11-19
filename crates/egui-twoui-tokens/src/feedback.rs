//! Touch press recoil, surface elevation, concentric corners, and cubic-Bézier motion helpers.

use egui::{
    epaint::Shadow, Color32, Context, CornerRadius, Id, Rect, Response, Stroke, StrokeKind, Ui,
    Vec2,
};

use crate::palette::TwoUiPalette;

/// Surface elevation tiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ElevationTier {
    /// Flat level (`0dp`).
    Zero,
    /// Subtle level (`2dp`).
    Sm,
    /// Standard floating pill level (`4dp`).
    #[default]
    Md,
    /// Dialogs and floating panes (`8dp`).
    Lg,
    /// High-emphasis overlays (`16dp`).
    Xl,
}

/// Builds an `egui::epaint::Shadow` for the given [`ElevationTier`].
pub fn elevation_shadow(tier: ElevationTier, dark: bool) -> Shadow {
    let (offset_y, blur, light_alpha, dark_alpha) = match tier {
        ElevationTier::Zero => return Shadow::NONE,
        ElevationTier::Sm => (2, 8, 22, 72),
        ElevationTier::Md => (4, 16, 30, 108),
        ElevationTier::Lg => (8, 28, 42, 148),
        ElevationTier::Xl => (12, 36, 54, 180),
    };
    Shadow {
        offset: [0, offset_y],
        blur,
        spread: 0,
        color: Color32::from_black_alpha(if dark { dark_alpha } else { light_alpha }),
    }
}

/// Computes a concentric inner radius given `outer_radius` and `inset`, preserving full rounding when `outer_radius >= 250`.
pub fn concentric_radius(outer_radius: u8, inset: f32) -> CornerRadius {
    if outer_radius >= 250 {
        return CornerRadius::same(255);
    }
    let inner = (f32::from(outer_radius) - inset).max(0.0).round() as u8;
    CornerRadius::same(inner)
}

/// Paints an elevated rounded surface with shadow, fill, and optional border stroke.
pub fn paint_frosted_surface(
    ui: &Ui,
    rect: Rect,
    corner_radius: CornerRadius,
    fill: Color32,
    stroke_color: Color32,
    tier: ElevationTier,
    dark: bool,
) {
    let shadow = elevation_shadow(tier, dark);
    if shadow != Shadow::NONE {
        ui.painter().add(shadow.as_shape(rect, corner_radius));
    }
    let stroke = if stroke_color.a() > 0 {
        Stroke::new(0.75, stroke_color)
    } else {
        Stroke::NONE
    };
    ui.painter()
        .rect(rect, corner_radius, fill, stroke, StrokeKind::Inside);
}

/// Paints a floating capsule (`ElevationTier::Md`, borderless pill).
pub fn paint_floating_capsule(
    ui: &Ui,
    rect: Rect,
    corner_radius: CornerRadius,
    dark: bool,
    palette: &TwoUiPalette,
) {
    paint_frosted_surface(
        ui,
        rect,
        corner_radius,
        palette.floating_surface,
        Color32::TRANSPARENT,
        ElevationTier::Md,
        dark,
    );
}

/// Evaluates a cubic Bézier easing curve `(x1, y1, x2, y2)` at `t in 0.0..=1.0`.
pub fn cubic_bezier(x1: f32, y1: f32, x2: f32, y2: f32, t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    let mut lo = 0.0_f32;
    let mut hi = 1.0_f32;
    let mut u = t;
    for _ in 0..10 {
        let inv = 1.0 - u;
        let x = 3.0 * inv * inv * u * x1 + 3.0 * inv * u * u * x2 + u * u * u;
        if x < t {
            lo = u;
        } else {
            hi = u;
        }
        u = 0.5 * (lo + hi);
    }
    let inv = 1.0 - u;
    3.0 * inv * inv * u * y1 + 3.0 * inv * u * u * y2 + u * u * u
}

/// Release deceleration curve (`0.22, 0.25, 0.0, 1.0`).
pub fn ease_release(t: f32) -> f32 {
    cubic_bezier(0.22, 0.25, 0.0, 1.0, t)
}

/// Out-70 sine curve (`0.17, 0.17, 0.3, 1.0`).
pub fn ease_sine_out_70(t: f32) -> f32 {
    cubic_bezier(0.17, 0.17, 0.3, 1.0, t)
}

/// Out-80 sine curve (`0.17, 0.17, 0.2, 1.0`).
pub fn ease_sine_out_80(t: f32) -> f32 {
    cubic_bezier(0.17, 0.17, 0.2, 1.0, t)
}

/// In-out-80 curve (`0.33, 0.0, 0.2, 1.0`).
pub fn ease_sine_in_out_80(t: f32) -> f32 {
    cubic_bezier(0.33, 0.0, 0.2, 1.0, t)
}

/// Standard fast-out-slow-in curve (`0.4, 0.0, 0.2, 1.0`).
pub fn ease_fast_out_slow_in(t: f32) -> f32 {
    cubic_bezier(0.4, 0.0, 0.2, 1.0, t)
}

/// Animates a boolean toggle `0.0..=1.0` with a directional easing function.
pub fn animate_toggle(
    ui: &Ui,
    id: Id,
    target: bool,
    duration: f32,
    ease: fn(f32) -> f32,
) -> f32 {
    animate_toggle_ctx(ui.ctx(), id, target, duration, ease)
}

/// Context-level helper matching [`animate_toggle`], also usable by overlays (`Modal`/`Area`).
pub fn animate_toggle_ctx(
    ctx: &Context,
    id: Id,
    target: bool,
    duration: f32,
    ease: fn(f32) -> f32,
) -> f32 {
    let linear = ctx.animate_bool_with_time(id, target, duration);
    if target {
        ease(linear)
    } else {
        1.0 - ease(1.0 - linear)
    }
}

/// Computes `0.0..=1.0` press and hover progress (`100ms` press, `350ms` `(0.22, 0.25, 0.0, 1.0)` release).
pub fn interaction_factors(ui: &Ui, response: &Response) -> (f32, f32) {
    let pressed = response.is_pointer_button_down_on()
        || (response.has_focus() && ui.input(|i| i.key_down(egui::Key::Space)));
    let press_dur = if pressed { 0.10 } else { 0.35 };
    let linear_press = ui
        .ctx()
        .animate_bool_with_time(response.id.with("twoui_press"), pressed, press_dur);
    let press_t = if pressed {
        linear_press
    } else {
        1.0 - ease_release(1.0 - linear_press)
    };
    let hover_t = animate_toggle(
        ui,
        response.id.with("twoui_hover"),
        response.hovered(),
        0.15,
        ease_sine_out_80,
    );
    (press_t, hover_t)
}

/// Applies subtle press recoil shrink to `rect` without altering layout allocation.
pub fn recoil_rect(rect: Rect, press_t: f32, max_shrink_px: f32) -> Rect {
    if press_t <= 0.001 {
        return rect;
    }
    let delta = max_shrink_px * press_t;
    let max_safe = (rect.width().min(rect.height()) * 0.12).min(delta);
    rect.shrink2(Vec2::splat(max_safe))
}

/// Blends an interactive state overlay (hover + press) onto `base_fill`.
pub fn interactive_fill(
    base_fill: Color32,
    palette: &TwoUiPalette,
    hover_t: f32,
    press_t: f32,
) -> Color32 {
    let mut result = base_fill;
    if hover_t > 0.001 {
        result = result.blend(palette.hover_overlay.gamma_multiply(hover_t));
    }
    if press_t > 0.001 {
        result = result.blend(palette.press_overlay.gamma_multiply(press_t));
    }
    result
}

/// Paints a hover/press overlay pill directly over `rect`.
pub fn paint_state_overlay(
    ui: &Ui,
    rect: Rect,
    corner_radius: CornerRadius,
    palette: &TwoUiPalette,
    hover_t: f32,
    press_t: f32,
) {
    if hover_t > 0.001 {
        ui.painter().rect_filled(
            rect,
            corner_radius,
            palette.hover_overlay.gamma_multiply(hover_t),
        );
    }
    if press_t > 0.001 {
        ui.painter().rect_filled(
            rect,
            corner_radius,
            palette.press_overlay.gamma_multiply(press_t),
        );
    }
}

/// Paints a full-pill press highlight covering the exact `rect` and `corner_radius`.
pub fn paint_pill_press_ripple(
    ui: &Ui,
    rect: Rect,
    corner_radius: CornerRadius,
    palette: &TwoUiPalette,
    press_t: f32,
) {
    if press_t <= 0.001 || rect.width() <= 1.0 || rect.height() <= 1.0 {
        return;
    }
    let dark = palette.round_and_bg.r() < 40;
    let alpha = if dark {
        (60.0 * press_t).round() as u8
    } else {
        (40.0 * press_t).round() as u8
    };
    let col = if dark {
        Color32::from_white_alpha(alpha)
    } else {
        Color32::from_black_alpha(alpha)
    };
    ui.painter().rect_filled(rect, corner_radius, col);
}
