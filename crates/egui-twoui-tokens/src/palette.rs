//! Light and dark polarity and semantic color palette.

use egui::Color32;

use crate::tokens::{colors, gray};

/// Light / dark polarity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TwoUiMode {
    #[default]
    Light,
    Dark,
}

impl TwoUiMode {
    /// Returns `true` if the mode is [`TwoUiMode::Dark`].
    pub fn is_dark(self) -> bool {
        matches!(self, Self::Dark)
    }

    /// Toggles between Light and Dark mode.
    pub fn toggled(self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }
}

/// Semantic color palette.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TwoUiPalette {
    /// App window / scaffold canvas.
    pub round_and_bg: Color32,
    /// Card / grouped section surface.
    pub surface: Color32,
    /// Tonal gray container.
    pub surface_container: Color32,
    /// Dialog, bottom sheet, and popup menu fill.
    pub surface_elevated: Color32,
    /// Inset floating drawer card fill.
    pub drawer_surface: Color32,
    /// Translucent floating capsule surface.
    pub floating_surface: Color32,
    /// Translucent toolbar projection pill surface.
    pub floating_projection: Color32,
    /// Hairline rim highlight around floating drawer cards & menus.
    pub floating_stroke: Color32,
    /// Subtle tone-on-tone pill surface.
    pub tone_on_tone: Color32,
    /// Active selection tint.
    pub selected_bg: Color32,

    /// Primary accent color (defaults to `#387AFF`).
    pub primary: Color32,
    /// High-contrast accent used for links, labels, and active icons.
    pub primary_high: Color32,
    /// Text / glyph contrast color drawn inside primary filled shapes.
    pub on_primary: Color32,

    /// Highest-emphasis body / title text.
    pub text_primary: Color32,
    /// Medium-emphasis body text.
    pub text_secondary: Color32,
    /// Muted subtitles, hints, and captions.
    pub text_muted: Color32,
    /// Disabled control label text.
    pub text_disabled: Color32,

    /// Hairline section & list dividers.
    pub divider: Color32,
    /// Control borders and subtle rims.
    pub outline: Color32,
    /// High-contrast outline button text & border.
    pub outline_button: Color32,

    /// Hover overlay highlight.
    pub hover_overlay: Color32,
    /// Press / recoil overlay tint.
    pub press_overlay: Color32,

    /// Switch track off fill.
    pub switch_track_off: Color32,
    /// Switch thumb center fill.
    pub switch_thumb: Color32,

    /// Progress & SeekBar inactive track fill.
    pub progress_track: Color32,

    /// Semantic green / success accent.
    pub functional_green: Color32,
    /// Semantic orange / warning accent (also Dual-Color SeekBar overlap zone).
    pub functional_orange: Color32,
    /// Semantic red / destructive action accent.
    pub functional_red: Color32,

    /// Orange dot / number badge fill.
    pub badge_bg: Color32,
    /// Full-window modal dim scrim.
    pub backdrop: Color32,
}

impl TwoUiPalette {
    /// Default Light palette.
    pub fn light() -> Self {
        Self {
            round_and_bg: gray::GRAY_05,
            surface: gray::GRAY_01,
            surface_container: gray::GRAY_15,
            surface_elevated: gray::GRAY_01,
            drawer_surface: colors::DRAWER_SURFACE_LIGHT,
            floating_surface: colors::FLOATING_GLASS_LIGHT,
            floating_projection: colors::FLOATING_TOOLBAR_GLASS_LIGHT,
            floating_stroke: gray::GRAY_15,
            tone_on_tone: colors::TONE_ON_TONE_LIGHT,
            selected_bg: Color32::from_rgba_unmultiplied_const(0x00, 0x00, 0x00, 0x1A),

            primary: colors::PRIMARY_BLUE,
            primary_high: colors::PRIMARY_BLUE_HIGH_LIGHT,
            on_primary: gray::GRAY_01,

            text_primary: gray::GRAY_100,
            text_secondary: gray::GRAY_70,
            text_muted: gray::GRAY_50,
            text_disabled: Color32::from_rgba_unmultiplied_const(0x8C, 0x8C, 0x91, 0x66),

            divider: gray::GRAY_15,
            outline: gray::GRAY_20,
            outline_button: gray::GRAY_50,

            hover_overlay: colors::HOVER_OVERLAY_LIGHT,
            press_overlay: colors::PRESS_OVERLAY_LIGHT,

            switch_track_off: gray::GRAY_40,
            switch_thumb: gray::GRAY_01,

            progress_track: colors::TONE_ON_TONE_HIGH_LIGHT,

            functional_green: colors::GREEN_LIGHT,
            functional_orange: colors::ORANGE,
            functional_red: colors::RED_LIGHT,

            badge_bg: colors::ORANGE,
            backdrop: colors::BACKDROP_LIGHT,
        }
    }

    /// Default Dark palette.
    pub fn dark() -> Self {
        Self {
            round_and_bg: gray::GRAY_100,
            surface: gray::GRAY_95,
            surface_container: gray::GRAY_80,
            surface_elevated: gray::GRAY_90,
            drawer_surface: colors::DRAWER_SURFACE_DARK,
            floating_surface: colors::FLOATING_GLASS_DARK,
            floating_projection: colors::FLOATING_TOOLBAR_GLASS_DARK,
            floating_stroke: Color32::from_rgba_unmultiplied_const(0xFF, 0xFF, 0xFF, 0x22),
            tone_on_tone: colors::TONE_ON_TONE_DARK,
            selected_bg: Color32::from_rgba_unmultiplied_const(0xFF, 0xFF, 0xFF, 0x1A),

            primary: colors::PRIMARY_BLUE,
            primary_high: colors::PRIMARY_BLUE_HIGH_DARK,
            on_primary: gray::GRAY_01,

            text_primary: gray::GRAY_01,
            text_secondary: gray::GRAY_20,
            text_muted: gray::GRAY_40,
            text_disabled: Color32::from_rgba_unmultiplied_const(0x99, 0x99, 0x9E, 0x66),

            divider: gray::GRAY_80,
            outline: gray::GRAY_60,
            outline_button: gray::GRAY_40,

            hover_overlay: colors::HOVER_OVERLAY_DARK,
            press_overlay: colors::PRESS_OVERLAY_DARK,

            switch_track_off: gray::GRAY_60,
            switch_thumb: gray::GRAY_01,

            progress_track: colors::TONE_ON_TONE_HIGH_DARK,

            functional_green: colors::GREEN_DARK_TEXT,
            functional_orange: colors::ORANGE_DARK_TEXT,
            functional_red: colors::RED_DARK,

            badge_bg: colors::ORANGE,
            backdrop: colors::BACKDROP_DARK,
        }
    }

    /// Overrides the primary accent across the palette with accessible high-contrast derivation.
    pub fn with_accent(mut self, accent: Color32) -> Self {
        self.primary = accent;
        let lum = (0.299 * f32::from(accent.r())
            + 0.587 * f32::from(accent.g())
            + 0.114 * f32::from(accent.b()))
            / 255.0;
        let is_dark_surface = self.round_and_bg.r() < 40;
        self.primary_high = if is_dark_surface {
            accent.lerp_to_gamma(Color32::WHITE, 0.20)
        } else {
            accent.lerp_to_gamma(Color32::BLACK, 0.14)
        };
        self.on_primary = if lum > 0.72 { gray::GRAY_100 } else { gray::GRAY_01 };
        self
    }
}
