//! Design tokens, light/dark palettes, theme management, and touch recoil helpers for `egui`.

pub mod feedback;
pub mod palette;
pub mod system_mode;
pub mod theme;
pub mod tokens;
pub mod typography;

pub use feedback::{
    animate_toggle, animate_toggle_ctx, concentric_radius, cubic_bezier,
    ease_fast_out_slow_in, ease_release, ease_sine_in_out_80, ease_sine_out_70,
    ease_sine_out_80, elevation_shadow, interaction_factors, interactive_fill,
    paint_floating_capsule, paint_frosted_surface, paint_pill_press_ripple, paint_state_overlay,
    recoil_rect, ElevationTier,
};
pub use palette::{TwoUiMode, TwoUiPalette};
pub use system_mode::detect_system_mode;
pub use theme::{
    apply_theme, install_themes, install_themes_with_preference, theme_from_ctx, theme_from_ui,
    TwoUiTheme,
};
pub use tokens::{colors, dimens, gray};
#[cfg(feature = "font")]
pub use typography::install_typography;
pub use typography::{
    build_variable_font_definitions, font_id, font_id_for, has_twoui_fonts, install_custom_font,
    install_custom_variable_font, reset_default_fonts, TwoUiFontWeight, TwoUiTypography,
};

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{pos2, vec2, Color32, Context, Rect};

    #[test]
    fn install_themes_populates_ctx_light_and_dark() {
        let ctx = Context::default();
        let custom = Color32::from_rgb(0x11, 0xA8, 0x5F);

        install_themes(&ctx, Some(custom), TwoUiMode::Light);
        let light = theme_from_ctx(&ctx);
        assert!(!light.mode.is_dark());
        assert_eq!(light.palette.primary, custom);

        install_themes(&ctx, Some(custom), TwoUiMode::Dark);
        let dark = theme_from_ctx(&ctx);
        assert!(dark.mode.is_dark());
        assert_eq!(dark.palette.primary, custom);
    }

    #[test]
    fn recoil_rect_shrinks_proportionally_and_bounds_safely() {
        let base = Rect::from_min_size(pos2(10.0, 20.0), vec2(100.0, 40.0));
        let unpressed = recoil_rect(base, 0.0, 2.0);
        assert_eq!(unpressed, base);

        let pressed = recoil_rect(base, 1.0, 2.0);
        assert!(pressed.width() < base.width());
        assert!(pressed.height() < base.height());
        assert_eq!(pressed.center(), base.center());
    }

    #[test]
    fn concentric_radius_preserves_capsules_and_insets() {
        assert_eq!(concentric_radius(255, 6.0).nw, 255);
        assert_eq!(concentric_radius(22, 4.0).nw, 18);
        assert_eq!(concentric_radius(6, 10.0).nw, 0);
    }
}
