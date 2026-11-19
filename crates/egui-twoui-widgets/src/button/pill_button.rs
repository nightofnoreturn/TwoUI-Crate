use egui::{
    pos2, vec2, Color32, CornerRadius, Rect, Response, Role, Sense, Stroke, StrokeKind, Ui, Vec2,
    Widget, WidgetInfo,
};
use egui_twoui_icons::TwoUiIcon;
use egui_twoui_tokens::{
    font_id, interaction_factors, interactive_fill, paint_pill_press_ripple, recoil_rect,
    theme_from_ui, tokens::dimens, TwoUiFontWeight,
};

use crate::progress::TwoUiSpinner;

/// Button visual style variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TwoUiButtonStyle {
    /// Tonal gray pill (`18dp` radius).
    #[default]
    Default,
    /// Accent blue filled pill (`18dp` radius).
    Primary,
    /// Borderless transparent button with pill hover/press feedback.
    Transparent,
    /// Thin outlined pill button (`18dp` radius).
    Outline,
    /// Large full-pill tonal CTA (`26dp` capsule).
    Contained,
    /// Large full-pill primary CTA (`26dp` capsule).
    ContainedPrimary,
    /// Large full-pill transparent CTA (`26dp` capsule).
    ContainedTransparent,
    /// Destructive action button (red text, subtle tint on hover).
    Destructive,
}

/// Pill button supporting standard, primary, outline, transparent, and contained CTA styles
/// with touch recoil and optional loading spinner.
pub struct TwoUiButton {
    text: String,
    style: TwoUiButtonStyle,
    icon: Option<TwoUiIcon>,
    text_color: Option<Color32>,
    corner_radius: Option<u8>,
    min_size: Option<Vec2>,
    fill_width: bool,
    enabled: bool,
    loading: bool,
}

impl TwoUiButton {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: TwoUiButtonStyle::Default,
            icon: None,
            text_color: None,
            corner_radius: None,
            min_size: None,
            fill_width: false,
            enabled: true,
            loading: false,
        }
    }

    pub fn text_color(mut self, color: Color32) -> Self {
        self.text_color = Some(color);
        self
    }

    pub fn corner_radius(mut self, radius: u8) -> Self {
        self.corner_radius = Some(radius);
        self
    }

    pub fn primary(text: impl Into<String>) -> Self {
        Self::new(text).style(TwoUiButtonStyle::Primary)
    }

    pub fn contained(text: impl Into<String>) -> Self {
        Self::new(text).style(TwoUiButtonStyle::Contained)
    }

    pub fn contained_primary(text: impl Into<String>) -> Self {
        Self::new(text).style(TwoUiButtonStyle::ContainedPrimary)
    }

    pub fn outline(text: impl Into<String>) -> Self {
        Self::new(text).style(TwoUiButtonStyle::Outline)
    }

    pub fn transparent(text: impl Into<String>) -> Self {
        Self::new(text).style(TwoUiButtonStyle::Transparent)
    }

    pub fn destructive(text: impl Into<String>) -> Self {
        Self::new(text).style(TwoUiButtonStyle::Destructive)
    }

    pub fn style(mut self, style: TwoUiButtonStyle) -> Self {
        self.style = style;
        self
    }

    pub fn icon(mut self, icon: TwoUiIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn min_size(mut self, min_size: Vec2) -> Self {
        self.min_size = Some(min_size);
        self
    }

    pub fn fill_width(mut self, fill: bool) -> Self {
        self.fill_width = fill;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Replaces button contents with a centered 4-dot loading spinner.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }
}

impl Widget for TwoUiButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let is_enabled = self.enabled && ui.is_enabled();

        let is_contained = matches!(
            self.style,
            TwoUiButtonStyle::Contained
                | TwoUiButtonStyle::ContainedPrimary
                | TwoUiButtonStyle::ContainedTransparent
        );

        let font_size = if is_contained {
            dimens::TEXT_BUTTON_CONTAINED
        } else {
            dimens::TEXT_BUTTON
        };
        let label_font = font_id(ui, font_size, TwoUiFontWeight::Medium);
        let galley = ui
            .painter()
            .layout_no_wrap(self.text.clone(), label_font.clone(), Color32::WHITE);
        let text_size = galley.size();

        let icon_w = if self.icon.is_some() { 18.0 } else { 0.0 };
        let icon_gap = if self.icon.is_some() && !self.text.is_empty() {
            8.0
        } else {
            0.0
        };

        let h_pad = if is_contained { 24.0 } else { 18.0 };
        let default_min = if is_contained {
            vec2(196.0, dimens::CONTAINED_BUTTON_MIN_HEIGHT)
        } else {
            vec2(72.0, dimens::BUTTON_MIN_HEIGHT)
        };
        let min_size = self.min_size.unwrap_or(default_min);

        let content_w = text_size.x + icon_w + icon_gap + h_pad * 2.0;
        let width = if self.fill_width {
            ui.available_width().max(content_w)
        } else {
            content_w.max(min_size.x)
        };
        let height = (text_size.y + 16.0).max(min_size.y);

        let sense = if is_enabled {
            Sense::click()
        } else {
            Sense::hover()
        };
        let (rect, response) = ui.allocate_exact_size(vec2(width, height), sense);

        if ui.is_rect_visible(rect) {
            let (press_t, hover_t) = if is_enabled {
                interaction_factors(ui, &response)
            } else {
                (0.0, 0.0)
            };

            let paint_rect = recoil_rect(rect, press_t, 0.6);
            let default_r = if is_contained {
                dimens::CONTAINED_BUTTON_CORNER_RADIUS
            } else {
                dimens::BUTTON_CORNER_RADIUS
            };
            let radius = CornerRadius::same(self.corner_radius.unwrap_or(default_r));

            let (base_fill, text_color, border) = match self.style {
                TwoUiButtonStyle::Default | TwoUiButtonStyle::Contained => {
                    (p.surface_container, p.text_primary, Stroke::NONE)
                }
                TwoUiButtonStyle::Primary | TwoUiButtonStyle::ContainedPrimary => {
                    (p.primary, p.on_primary, Stroke::NONE)
                }
                TwoUiButtonStyle::Transparent | TwoUiButtonStyle::ContainedTransparent => {
                    (Color32::TRANSPARENT, p.text_primary, Stroke::NONE)
                }
                TwoUiButtonStyle::Outline => (
                    Color32::TRANSPARENT,
                    p.outline_button,
                    Stroke::new(0.9, p.outline_button.gamma_multiply(0.55)),
                ),
                TwoUiButtonStyle::Destructive => {
                    (Color32::TRANSPARENT, p.functional_red, Stroke::NONE)
                }
            };

            let final_fill = if !is_enabled {
                if base_fill == Color32::TRANSPARENT {
                    Color32::TRANSPARENT
                } else {
                    base_fill.gamma_multiply(0.42)
                }
            } else if base_fill == Color32::TRANSPARENT {
                if hover_t > 0.001 {
                    p.hover_overlay.gamma_multiply(hover_t)
                } else {
                    Color32::TRANSPARENT
                }
            } else {
                interactive_fill(base_fill, p, hover_t, 0.0)
            };

            let final_text = self.text_color.unwrap_or(text_color);
            let final_text = if is_enabled {
                final_text
            } else {
                final_text.gamma_multiply(0.4)
            };

            let final_border = if is_enabled {
                border
            } else {
                Stroke::new(border.width, border.color.gamma_multiply(0.4))
            };

            ui.painter()
                .rect(rect, radius, final_fill, final_border, StrokeKind::Inside);
            paint_pill_press_ripple(ui, rect, radius, p, press_t);

            if self.loading {
                let spin_rect = Rect::from_center_size(paint_rect.center(), Vec2::splat(22.0));
                TwoUiSpinner::paint_indeterminate_at(ui, spin_rect, 2.5, p);
            } else {
                let total_content_w = icon_w + icon_gap + text_size.x;
                let start_x = paint_rect.center().x - total_content_w * 0.5;
                let cy = paint_rect.center().y;

                if let Some(icon) = self.icon {
                    let icon_rect = Rect::from_center_size(
                        pos2(start_x + icon_w * 0.5, cy),
                        vec2(icon_w, icon_w),
                    );
                    icon.paint(ui, icon_rect, final_text);
                }

                if !self.text.is_empty() {
                    let text_pos = pos2(start_x + icon_w + icon_gap, cy);
                    let text_galley = ui.painter().layout_no_wrap(
                        self.text.clone(),
                        label_font,
                        final_text,
                    );
                    ui.painter().galley(
                        pos2(text_pos.x, text_pos.y - text_galley.size().y * 0.5),
                        text_galley,
                        final_text,
                    );
                }
            }
        }

        response.widget_info(|| WidgetInfo::labeled(Role::Button, is_enabled, &self.text));
        response
    }
}
