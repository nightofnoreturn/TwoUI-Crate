//! Modular UI component library for `egui`.
//!
//! Combines the workspace crates behind feature flags:
//! - `egui-twoui-tokens` (`tokens`, `theme`, `palette`, `system_mode`, `feedback`, `typography`)
//! - `egui-twoui-icons` (`icons`, `TwoUiIcon`, behind feature `icons`)
//! - `egui-twoui-widgets` (`widgets`, `ext`, behind feature `widgets` + `qrcode`)
//! - `egui-twoui-layouts` (`layouts`, behind feature `layouts`)

#[cfg(feature = "widgets")]
pub mod ext;

pub use egui_twoui_tokens::{backdrop_blur, feedback, palette, system_mode, tokens, typography};

/// Merged theme, palette, and system dark/light detection exports.
pub mod theme {
    pub use egui_twoui_tokens::palette::*;
    pub use egui_twoui_tokens::system_mode::*;
    pub use egui_twoui_tokens::theme::*;
}

#[cfg(feature = "icons")]
pub use egui_twoui_icons as icons;

#[cfg(feature = "widgets")]
pub use egui_twoui_widgets as widgets;

#[cfg(feature = "layouts")]
pub use egui_twoui_layouts as layouts;

#[cfg(feature = "widgets")]
pub use ext::TwoUiExt;

#[cfg(feature = "icons")]
pub use egui_twoui_icons::TwoUiIcon;

pub use egui_twoui_tokens::{
    animate_toggle, animate_toggle_ctx, apply_theme, concentric_radius, cubic_bezier,
    detect_system_mode, ease_fast_out_slow_in, ease_release, ease_sine_in_out_80,
    ease_sine_out_70, ease_sine_out_80, elevation_shadow, enable_backdrop_blur, font_id,
    font_id_for, has_twoui_fonts, install_custom_font, install_custom_variable_font,
    install_themes, install_themes_with_preference, is_backdrop_blur_enabled,
    paint_floating_capsule, paint_frosted_surface, reset_default_fonts,
    set_backdrop_blur_enabled, show_frosted_frame, theme_from_ctx, theme_from_ui, ElevationTier,
    TwoUiFontWeight, TwoUiMode, TwoUiPalette, TwoUiTheme, TwoUiTypography,
};
#[cfg(feature = "font")]
pub use egui_twoui_tokens::install_typography;

/// Convenient re-exports for applications using `egui-twoui`.
pub mod prelude {
    pub use crate::backdrop_blur::{
        enable_backdrop_blur, is_backdrop_blur_enabled, set_backdrop_blur_enabled,
    };
    pub use crate::feedback::{
        animate_toggle, animate_toggle_ctx, concentric_radius, cubic_bezier,
        ease_fast_out_slow_in, ease_release, ease_sine_in_out_80, ease_sine_out_70,
        ease_sine_out_80, elevation_shadow, paint_floating_capsule, paint_frosted_surface,
        show_frosted_frame, ElevationTier,
    };
    pub use crate::theme::{
        apply_theme, detect_system_mode, install_themes, install_themes_with_preference,
        theme_from_ctx, theme_from_ui, TwoUiMode, TwoUiPalette, TwoUiTheme,
    };
    pub use crate::typography::{
        font_id, font_id_for, has_twoui_fonts, install_custom_font, install_custom_variable_font,
        reset_default_fonts, TwoUiFontWeight, TwoUiTypography,
    };
    #[cfg(feature = "font")]
    pub use crate::typography::install_typography;
    #[cfg(feature = "icons")]
    pub use crate::icons::TwoUiIcon;
    #[cfg(feature = "widgets")]
    pub use crate::ext::TwoUiExt;
    #[cfg(feature = "widgets")]
    pub use crate::widgets::{
        color32_to_hsva, days_in_month, hsva_to_color32, is_leap_year, paint_badge, paint_divider,
        weekday_sunday0, BottomNavBar, BottomNavItem, BottomNavOverflowItem, CardItem, CardSection,
        DatePickerMode, DividerButtonItem, DividerInset, MainTabBar, TwoUiActionModeBar,
        TwoUiButton, TwoUiButtonStyle, TwoUiCheckBox, TwoUiChip, TwoUiColorPicker, TwoUiDatePicker,
        TwoUiDividerButtonLayout, TwoUiDropdown, TwoUiIconButton, TwoUiIndexScroll,
        TwoUiNumberPicker, TwoUiPreferenceCaption, TwoUiProgressBar, TwoUiRadioButton,
        TwoUiSearchBar, TwoUiSeekBar, TwoUiSeekBarPlus, TwoUiSeekBarPro, TwoUiSleepTimePicker,
        TwoUiSnackBarState, TwoUiSpinner, TwoUiSwitch, TwoUiSwitchBar, TwoUiTimePicker, RadioItem,
        RelativeLinksCard, SectionHeader, SeekBarMode, SpinnerSize, SubTabBar, SuggestionCard,
        SwitchItem, TipsCard,
    };
    #[cfg(all(feature = "widgets", feature = "qrcode"))]
    pub use crate::widgets::TwoUiQrCode;
    #[cfg(feature = "layouts")]
    pub use crate::layouts::{
        apply_borderless_window_resize, show_bottom_floating_bar, show_color_picker_dialog,
        AppInfoView, AppUpdateState, DialogAction, DrawerEntry, TwoUiBottomSheet, TwoUiDialog,
        TwoUiDrawer, TwoUiScaffold, TwoUiSuggestiveDialog, TwoUiTipPopup, TwoUiToastState,
        ScaffoldNavButton, ScaffoldResponse,
    };
}

#[cfg(test)]
mod tests {
    use super::prelude::*;
    use egui::{Color32, Context};

    #[test]
    fn facade_prelude_installs_themes_without_optional_features() {
        let ctx = Context::default();
        let custom = Color32::from_rgb(0x11, 0xA8, 0x5F);

        install_themes(&ctx, Some(custom), TwoUiMode::Dark);
        let dark = theme_from_ctx(&ctx);
        assert!(dark.mode.is_dark());
        assert_eq!(dark.palette.primary, custom);
    }
}
