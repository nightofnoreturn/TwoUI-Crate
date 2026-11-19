//! Theme configuration and `egui::Style` / `egui::Visuals` installer.

use egui::{
    epaint::Shadow, vec2, Color32, Context, CornerRadius, FontId, Id, Stroke, Style, TextStyle,
    Ui, Visuals,
};

use crate::palette::{TwoUiMode, TwoUiPalette};
use crate::tokens::dimens;

/// Complete theme configuration stored inside [`egui::Context`].
#[derive(Debug, Clone, PartialEq)]
pub struct TwoUiTheme {
    pub mode: TwoUiMode,
    pub palette: TwoUiPalette,
    pub card_corner_radius: u8,
    pub dialog_corner_radius: u8,
    pub button_corner_radius: u8,
    pub animation_time: f32,
}

impl Default for TwoUiTheme {
    fn default() -> Self {
        Self::light()
    }
}

impl TwoUiTheme {
    /// Creates the default Light theme.
    pub fn light() -> Self {
        Self {
            mode: TwoUiMode::Light,
            palette: TwoUiPalette::light(),
            card_corner_radius: dimens::ROUNDED_CORNER_RADIUS,
            dialog_corner_radius: dimens::DIALOG_CORNER_RADIUS,
            button_corner_radius: dimens::BUTTON_CORNER_RADIUS,
            animation_time: 0.20,
        }
    }

    /// Creates the default Dark theme.
    pub fn dark() -> Self {
        Self {
            mode: TwoUiMode::Dark,
            palette: TwoUiPalette::dark(),
            card_corner_radius: dimens::ROUNDED_CORNER_RADIUS,
            dialog_corner_radius: dimens::DIALOG_CORNER_RADIUS,
            button_corner_radius: dimens::BUTTON_CORNER_RADIUS,
            animation_time: 0.20,
        }
    }

    /// Creates a theme for the given [`TwoUiMode`].
    pub fn for_mode(mode: TwoUiMode) -> Self {
        match mode {
            TwoUiMode::Light => Self::light(),
            TwoUiMode::Dark => Self::dark(),
        }
    }

    /// Overrides the primary accent color across the palette.
    pub fn with_accent(mut self, accent: Color32) -> Self {
        self.palette = self.palette.with_accent(accent);
        self
    }

    /// Builds an [`egui::Visuals`] configured for this theme.
    pub fn to_visuals(&self) -> Visuals {
        let p = &self.palette;
        let mut visuals = if self.mode.is_dark() {
            Visuals::dark()
        } else {
            Visuals::light()
        };

        visuals.dark_mode = self.mode.is_dark();
        visuals.override_text_color = None;
        visuals.hyperlink_color = p.primary_high;
        visuals.faint_bg_color = p.surface_container;
        visuals.extreme_bg_color = p.round_and_bg;
        visuals.text_edit_bg_color = Some(p.surface_container);
        visuals.code_bg_color = p.surface_container;
        visuals.warn_fg_color = p.functional_orange;
        visuals.error_fg_color = p.functional_red;

        visuals.panel_fill = p.round_and_bg;
        visuals.window_fill = p.surface_elevated;
        visuals.window_corner_radius = CornerRadius::same(self.dialog_corner_radius);
        visuals.window_stroke = Stroke::new(0.75, p.floating_stroke);
        visuals.window_shadow = if self.mode.is_dark() {
            Shadow {
                offset: [0, 6],
                blur: 24,
                spread: 0,
                color: Color32::from_black_alpha(120),
            }
        } else {
            Shadow {
                offset: [0, 4],
                blur: 18,
                spread: 0,
                color: Color32::from_black_alpha(28),
            }
        };
        visuals.popup_shadow = visuals.window_shadow;
        visuals.menu_corner_radius = CornerRadius::same(self.dialog_corner_radius);

        visuals.selection.bg_fill = p.primary.gamma_multiply(0.22);
        visuals.selection.stroke = Stroke::new(1.0, p.primary);
        visuals.text_cursor.stroke = Stroke::new(2.0, p.primary);
        visuals.slider_trailing_fill = true;

        visuals.widgets.noninteractive.bg_fill = p.surface;
        visuals.widgets.noninteractive.weak_bg_fill = p.surface;
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, p.divider);
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, p.text_primary);
        visuals.widgets.noninteractive.corner_radius = CornerRadius::same(self.card_corner_radius);

        visuals.widgets.inactive.bg_fill = p.surface_container;
        visuals.widgets.inactive.weak_bg_fill = p.surface_container;
        visuals.widgets.inactive.bg_stroke = Stroke::NONE;
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.5, p.text_primary);
        visuals.widgets.inactive.corner_radius = CornerRadius::same(self.button_corner_radius);

        let hover_bg = p.surface_container.blend(p.hover_overlay);
        visuals.widgets.hovered.bg_fill = hover_bg;
        visuals.widgets.hovered.weak_bg_fill = hover_bg;
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, p.outline);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.5, p.text_primary);
        visuals.widgets.hovered.corner_radius = CornerRadius::same(self.button_corner_radius);
        visuals.widgets.hovered.expansion = 0.0;

        let active_bg = p.surface_container.blend(p.press_overlay);
        visuals.widgets.active.bg_fill = active_bg;
        visuals.widgets.active.weak_bg_fill = active_bg;
        visuals.widgets.active.bg_stroke = Stroke::new(1.0, p.primary);
        visuals.widgets.active.fg_stroke = Stroke::new(1.5, p.text_primary);
        visuals.widgets.active.corner_radius = CornerRadius::same(self.button_corner_radius);
        visuals.widgets.active.expansion = 0.0;

        visuals.widgets.open.bg_fill = hover_bg;
        visuals.widgets.open.weak_bg_fill = hover_bg;
        visuals.widgets.open.bg_stroke = Stroke::new(1.0, p.primary);
        visuals.widgets.open.fg_stroke = Stroke::new(1.5, p.text_primary);
        visuals.widgets.open.corner_radius = CornerRadius::same(self.button_corner_radius);

        visuals
    }

    /// Builds a complete [`egui::Style`] tuned for consistent spacing, scrollbars, and typography.
    pub fn to_egui_style(&self) -> Style {
        let mut style = Style {
            visuals: self.to_visuals(),
            animation_time: self.animation_time,
            ..Style::default()
        };

        style.spacing.item_spacing = vec2(8.0, 8.0);
        style.spacing.button_padding = vec2(16.0, 8.0);
        style.spacing.window_margin = egui::Margin::same(16);
        style.spacing.menu_margin = egui::Margin::same(8);
        style.spacing.indent = 20.0;
        style.spacing.interact_size = vec2(40.0, 36.0);
        style.spacing.slider_width = 180.0;

        style.interaction.selectable_labels = false;
        style.spacing.scroll = egui::style::ScrollStyle::floating();
        style.spacing.scroll.floating_width = 5.0;
        style.spacing.scroll.floating_allocated_width = 0.0;

        let has_fonts = crate::typography::has_twoui_fonts_or_bundled();
        let prop_or = |w: crate::typography::TwoUiFontWeight| {
            if has_fonts {
                w.to_family()
            } else {
                egui::FontFamily::Proportional
            }
        };

        style.text_styles.insert(
            TextStyle::Heading,
            FontId::new(
                dimens::TEXT_DIALOG_TITLE,
                prop_or(crate::typography::TwoUiFontWeight::SemiBold),
            ),
        );
        style.text_styles.insert(
            TextStyle::Body,
            FontId::new(dimens::TEXT_BODY, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            TextStyle::Button,
            FontId::new(
                dimens::TEXT_BUTTON,
                prop_or(crate::typography::TwoUiFontWeight::Medium),
            ),
        );
        style.text_styles.insert(
            TextStyle::Small,
            FontId::new(dimens::TEXT_LIST_SECONDARY, egui::FontFamily::Proportional),
        );

        style
    }
}

fn theme_storage_id() -> Id {
    Id::unique("egui_twoui_active_theme")
}

/// Applies the supplied [`TwoUiTheme`] to `ctx`, installing its `egui::Style` and storing the
/// semantic [`TwoUiPalette`].
pub fn apply_theme(ctx: &Context, theme: &TwoUiTheme) {
    crate::typography::ensure_default_typography(ctx);
    let style = theme.to_egui_style();
    let mode_theme = if theme.mode.is_dark() {
        egui::Theme::Dark
    } else {
        egui::Theme::Light
    };
    ctx.set_theme(mode_theme);
    ctx.set_style_of(mode_theme, style);
    ctx.data_mut(|d| d.insert_temp(theme_storage_id(), theme.clone()));
}

/// Registers Light and Dark styles on `ctx` (optionally accented with `accent`)
/// and sets an explicit `preference` (`Light`, `Dark`, or `System`) while activating `resolved_mode`.
pub fn install_themes_with_preference(
    ctx: &Context,
    accent: Option<Color32>,
    resolved_mode: TwoUiMode,
    preference: egui::ThemePreference,
) {
    crate::typography::ensure_default_typography(ctx);
    let mut light = TwoUiTheme::light();
    let mut dark = TwoUiTheme::dark();
    if let Some(col) = accent {
        light = light.with_accent(col);
        dark = dark.with_accent(col);
    }
    ctx.set_style_of(egui::Theme::Light, light.to_egui_style());
    ctx.set_style_of(egui::Theme::Dark, dark.to_egui_style());

    let active = match resolved_mode {
        TwoUiMode::Light => light,
        TwoUiMode::Dark => dark,
    };
    ctx.set_theme(preference);
    ctx.data_mut(|d| d.insert_temp(theme_storage_id(), active));
}

/// Registers Light and Dark styles on `ctx` (optionally accented with `accent`)
/// and activates `initial_mode`.
pub fn install_themes(ctx: &Context, accent: Option<Color32>, initial_mode: TwoUiMode) {
    let preference = if initial_mode.is_dark() {
        egui::ThemePreference::Dark
    } else {
        egui::ThemePreference::Light
    };
    install_themes_with_preference(ctx, accent, initial_mode, preference);
}

/// Reads the currently active [`TwoUiTheme`] from `ctx`.
pub fn theme_from_ctx(ctx: &Context) -> TwoUiTheme {
    if let Some(theme) = ctx.data(|d| d.get_temp::<TwoUiTheme>(theme_storage_id())) {
        return theme;
    }
    TwoUiTheme::for_mode(if ctx.global_style().visuals.dark_mode {
        TwoUiMode::Dark
    } else {
        TwoUiMode::Light
    })
}

/// Convenience wrapper over [`theme_from_ctx`] taking a `&Ui`.
pub fn theme_from_ui(ui: &Ui) -> TwoUiTheme {
    theme_from_ctx(ui.ctx())
}
