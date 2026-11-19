//! Design tokens: gray scale, colors, radii, and spacing constants.

use egui::Color32;

/// Neutral gray scale.
pub mod gray {
    use super::Color32;

    /// Pure white (`#FFFFFF`).
    pub const WHITE: Color32 = Color32::from_rgb(0xFF, 0xFF, 0xFF);
    /// Soft off-white light canvas (`#FCFCFF`).
    pub const GRAY_01: Color32 = Color32::from_rgb(0xFC, 0xFC, 0xFF);
    /// Light background gray (`#F3F3F5`).
    pub const GRAY_05: Color32 = Color32::from_rgb(0xF3, 0xF3, 0xF5);
    /// Light neutral divider (`#E4E4E7`).
    pub const GRAY_15: Color32 = Color32::from_rgb(0xE4, 0xE4, 0xE7);
    /// Muted border / secondary light icon tint (`#CCCCCE`).
    pub const GRAY_20: Color32 = Color32::from_rgb(0xCC, 0xCC, 0xCE);
    /// Mid-light neutral (`#A3A3A7`).
    pub const GRAY_40: Color32 = Color32::from_rgb(0xA3, 0xA3, 0xA7);
    /// Balanced mid-gray (`#848487`).
    pub const GRAY_50: Color32 = Color32::from_rgb(0x84, 0x84, 0x87);
    /// Muted body text gray (`#636368`).
    pub const GRAY_60: Color32 = Color32::from_rgb(0x63, 0x63, 0x68);
    /// Secondary dark text gray (`#4D4D52`).
    pub const GRAY_70: Color32 = Color32::from_rgb(0x4D, 0x4D, 0x52);
    /// Subtle dark divider (`#3A3A3D`).
    pub const GRAY_80: Color32 = Color32::from_rgb(0x3A, 0x3A, 0x3D);
    /// Dark container and dialog surface (`#2D2D30`).
    pub const GRAY_85: Color32 = Color32::from_rgb(0x2D, 0x2D, 0x30);
    /// Deep charcoal (`#252528`).
    pub const GRAY_90: Color32 = Color32::from_rgb(0x25, 0x25, 0x28);
    /// Dark card surface (`#17171A`).
    pub const GRAY_95: Color32 = Color32::from_rgb(0x17, 0x17, 0x1A);
    /// OLED dark canvas (`#010102`).
    pub const GRAY_100: Color32 = Color32::from_rgb(0x01, 0x01, 0x02);
}

/// Brand, semantic, state, and floating surface color tokens.
pub mod colors {
    use super::{gray, Color32};

    /// Primary blue accent (`#387AFF`).
    pub const PRIMARY_BLUE: Color32 = Color32::from_rgb(0x38, 0x7A, 0xFF);
    /// High-contrast primary accent for dark mode (`#598FFF`).
    pub const PRIMARY_BLUE_HIGH_DARK: Color32 = Color32::from_rgb(0x59, 0x8F, 0xFF);
    /// High-contrast primary accent for light mode (`#2E65D4`).
    pub const PRIMARY_BLUE_HIGH_LIGHT: Color32 = Color32::from_rgb(0x2E, 0x65, 0xD4);

    /// Success green for light surfaces.
    pub const GREEN_LIGHT: Color32 = Color32::from_rgb(0x11, 0xA8, 0x5F);
    /// Success green text for dark surfaces.
    pub const GREEN_DARK_TEXT: Color32 = Color32::from_rgb(0x58, 0xDB, 0x9C);

    /// Warning orange for light surfaces.
    pub const ORANGE: Color32 = Color32::from_rgb(0xE6, 0x5B, 0x17);
    /// Warning orange text for dark surfaces.
    pub const ORANGE_DARK_TEXT: Color32 = Color32::from_rgb(0xFC, 0x86, 0x4C);

    /// Destructive red for light surfaces.
    pub const RED_LIGHT: Color32 = Color32::from_rgb(0xD9, 0x3E, 0x36);
    /// Destructive red for dark surfaces.
    pub const RED_DARK: Color32 = Color32::from_rgb(0xF5, 0x69, 0x62);

    /// Secondary progress accent teal.
    pub const PROGRESS_TEAL: Color32 = Color32::from_rgb(0x23, 0xC4, 0x92);

    /// Floating capsule background (light, 92% opacity).
    pub const FLOATING_GLASS_LIGHT: Color32 =
        Color32::from_rgba_unmultiplied_const(0xFC, 0xFC, 0xFF, 235);
    /// Floating capsule background (dark, 92% opacity).
    pub const FLOATING_GLASS_DARK: Color32 =
        Color32::from_rgba_unmultiplied_const(0x25, 0x25, 0x28, 235);

    /// Floating toolbar pill background (light, 90% opacity).
    pub const FLOATING_TOOLBAR_GLASS_LIGHT: Color32 =
        Color32::from_rgba_unmultiplied_const(0xFC, 0xFC, 0xFF, 230);
    /// Floating toolbar pill background (dark, 90% opacity).
    pub const FLOATING_TOOLBAR_GLASS_DARK: Color32 =
        Color32::from_rgba_unmultiplied_const(0x25, 0x25, 0x28, 230);

    /// Navigation drawer surface (light, `#F2F2F3`).
    pub const DRAWER_SURFACE_LIGHT: Color32 = Color32::from_rgb(0xF2, 0xF2, 0xF3);
    /// Navigation drawer surface (dark, `#17171A`).
    pub const DRAWER_SURFACE_DARK: Color32 = gray::GRAY_95;

    /// Tone-on-tone surface fill (light).
    pub const TONE_ON_TONE_LIGHT: Color32 =
        Color32::from_rgba_unmultiplied_const(0x17, 0x17, 0x1A, 0x0D);
    /// Tone-on-tone surface fill (dark).
    pub const TONE_ON_TONE_DARK: Color32 =
        Color32::from_rgba_unmultiplied_const(0xFC, 0xFC, 0xFF, 0x1A);
    /// Higher-contrast tone-on-tone fill (light).
    pub const TONE_ON_TONE_HIGH_LIGHT: Color32 =
        Color32::from_rgba_unmultiplied_const(0x17, 0x17, 0x1A, 0x1A);
    /// Higher-contrast tone-on-tone fill (dark).
    pub const TONE_ON_TONE_HIGH_DARK: Color32 =
        Color32::from_rgba_unmultiplied_const(0xFC, 0xFC, 0xFF, 0x26);

    /// Floating snackbar background (`#CC2D2D30`).
    pub const SUGGEST_SNACKBAR_BG: Color32 =
        Color32::from_rgba_unmultiplied_const(0x2D, 0x2D, 0x30, 0xCC);
    /// Snackbar action pill fill (`#FCFCFC`).
    pub const SUGGEST_SNACKBAR_ACTION_BG: Color32 = Color32::from_rgb(0xFC, 0xFC, 0xFC);

    /// Dark suggestive dialog container fill.
    pub const SUGGEST_DIALOG_BG: Color32 =
        Color32::from_rgba_unmultiplied_const(0x2D, 0x2D, 0x30, 0xEE);

    /// Tooltip balloon background.
    pub const TIP_POPUP_BG: Color32 = Color32::from_rgb(0x47, 0x47, 0x47);
    /// Tooltip balloon text color.
    pub const TIP_POPUP_TEXT: Color32 = Color32::from_rgb(0xFA, 0xFA, 0xFA);

    /// Press highlight overlay (light).
    pub const PRESS_OVERLAY_LIGHT: Color32 = Color32::from_rgba_unmultiplied_const(0, 0, 0, 0x1A);
    /// Press highlight overlay (dark).
    pub const PRESS_OVERLAY_DARK: Color32 =
        Color32::from_rgba_unmultiplied_const(0xFF, 0xFF, 0xFF, 0x33);
    /// Hover highlight (light): 5% dark tint.
    pub const HOVER_OVERLAY_LIGHT: Color32 = Color32::from_rgba_unmultiplied_const(1, 1, 2, 0x0D);
    /// Hover highlight (dark): 10% light tint.
    pub const HOVER_OVERLAY_DARK: Color32 =
        Color32::from_rgba_unmultiplied_const(0xFA, 0xFA, 0xFF, 0x1A);

    /// Modal backdrop scrim in Light Mode.
    pub const BACKDROP_LIGHT: Color32 = Color32::from_rgba_unmultiplied_const(0, 0, 0, 89);
    /// Modal backdrop scrim in Dark Mode.
    pub const BACKDROP_DARK: Color32 = Color32::from_rgba_unmultiplied_const(0, 0, 0, 166);
}

/// Radii, spacing, and typography metrics.
pub mod dimens {
    /// Full capsule radius (`255`).
    pub const CAPSULE_CORNER_RADIUS: u8 = 255;

    /// Standard card corner radius (`26dp`).
    pub const ROUNDED_CORNER_RADIUS: u8 = 26;
    /// Dialog container corner radius (`26dp`).
    pub const DIALOG_CORNER_RADIUS: u8 = 26;
    /// Dialog action button corner radius (`22dp`).
    pub const DIALOG_BUTTON_CORNER_RADIUS: u8 = 22;
    /// Standard pill button corner radius (`18dp`).
    pub const BUTTON_CORNER_RADIUS: u8 = 18;
    /// Contained CTA full-pill corner radius (`26dp`).
    pub const CONTAINED_BUTTON_CORNER_RADIUS: u8 = 26;

    /// Drawer panel corner radius (`22dp`).
    pub const DRAWER_CORNER_RADIUS: u8 = 22;
    /// Drawer outer start inset (`10dp`).
    pub const DRAWER_START_INSET: f32 = 10.0;
    /// Expanded navigation drawer width (`300dp`).
    pub const DRAWER_WIDTH_OPEN: f32 = 300.0;

    /// Sub-tab pill height (`40dp`).
    pub const SUBTAB_HEIGHT: f32 = 40.0;
    /// Sub-tab pill corner radius (`20dp`).
    pub const SUBTAB_RADIUS: u8 = 20;

    /// Floating navigation bar maximum width (`430dp`).
    pub const FLOATING_NAV_MAX_WIDTH: f32 = 430.0;
    /// Floating navigation container inner padding (`4dp`).
    pub const FLOATING_NAV_INNER_PADDING: f32 = 4.0;

    /// Split action bar button height (`52dp`).
    pub const DIVIDER_BUTTON_HEIGHT: f32 = 52.0;
    /// Split action bar outer padding (`6dp`).
    pub const DIVIDER_BUTTON_PADDING: f32 = 6.0;
    /// Split action bar vertical divider height (`16dp`).
    pub const DIVIDER_BUTTON_DIVIDER_HEIGHT: f32 = 16.0;
    /// Split action bar vertical divider alpha (`0.23`).
    pub const DIVIDER_BUTTON_DIVIDER_ALPHA: f32 = 0.23;

    /// Search bar plate height (`52dp`).
    pub const SEARCH_PLATE_HEIGHT: f32 = 52.0;
    /// Search bar corner radius (`26dp`).
    pub const SEARCH_PLATE_RADIUS: u8 = 26;
    /// Search bar bottom highlight stroke (`0.5dp`).
    pub const SEARCH_BOTTOM_STROKE: f32 = 0.5;

    /// Outer section horizontal margin (`10dp`).
    pub const SECTION_HORIZONTAL_MARGIN: f32 = 10.0;
    /// Standard list item horizontal padding (`20dp`).
    pub const LIST_HORIZONTAL_PADDING: f32 = 20.0;
    /// Preferred list item minimum height (`56dp`).
    pub const LIST_ITEM_MIN_HEIGHT: f32 = 56.0;
    /// Compact list item minimum height (`52dp`).
    pub const LIST_ITEM_MIN_HEIGHT_SMALL: f32 = 52.0;

    /// Top inset inside the AppBar (`6dp`).
    pub const TOOLBAR_TOP_INSET: f32 = 6.0;
    /// Pinned AppBar action row height (`44dp`).
    pub const TOOLBAR_ROW_HEIGHT: f32 = 44.0;
    /// Compact AppBar header height (`56dp`).
    pub const TOOLBAR_HEIGHT: f32 = 56.0;
    /// Extended collapsing AppBar hero height (`152dp`).
    pub const EXTENDED_APPBAR_EXTRA_HEIGHT: f32 = 152.0;

    /// Standard contained CTA button minimum height (`48dp`).
    pub const CONTAINED_BUTTON_MIN_HEIGHT: f32 = 48.0;
    /// Standard button minimum height (`36dp`).
    pub const BUTTON_MIN_HEIGHT: f32 = 36.0;
    /// Switch bar height (`52dp`).
    pub const SWITCH_BAR_HEIGHT: f32 = 52.0;

    /// Suggestion card title size (`15dp`).
    pub const APPBAR_SUGGEST_TITLE: f32 = 15.0;
    /// Suggestion card subtitle size (`13dp`).
    pub const APPBAR_SUGGEST_SUBTITLE: f32 = 13.0;

    /// List caption top padding (`8dp`).
    pub const LIST_CAPTION_PADDING_TOP: f32 = 8.0;
    /// List caption bottom padding (`16dp`).
    pub const LIST_CAPTION_PADDING_BOTTOM: f32 = 16.0;
    /// List caption text size (`12sp`).
    pub const LIST_CAPTION_TEXT_SIZE: f32 = 12.0;

    // Typography sizes
    pub const TEXT_EXTENDED_TITLE: f32 = 34.0;
    pub const TEXT_TOOLBAR_TITLE: f32 = 19.0;
    pub const TEXT_DIALOG_TITLE: f32 = 18.0;
    pub const TEXT_LIST_PRIMARY: f32 = 16.0;
    pub const TEXT_BUTTON_CONTAINED: f32 = 16.0;
    pub const TEXT_BUTTON: f32 = 14.5;
    pub const TEXT_BODY: f32 = 14.0;
    pub const TEXT_SUBHEADER: f32 = 13.0;
    pub const TEXT_LIST_SECONDARY: f32 = 12.5;
}
