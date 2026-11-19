use egui::{Color32, CornerRadius, Rect, Response, Sense, Ui, Vec2, Widget};
use egui_twoui_icons::TwoUiIcon;
use egui_twoui_tokens::{
    interaction_factors, interactive_fill, paint_pill_press_ripple, recoil_rect, theme_from_ui,
    tokens::dimens,
};

/// Circular icon-only button used in toolbars, search bars, and cards.
pub struct TwoUiIconButton {
    icon: TwoUiIcon,
    size: f32,
    icon_size: f32,
    tint: Option<Color32>,
    fill: Option<Color32>,
    tooltip: Option<String>,
}

impl TwoUiIconButton {
    pub fn new(icon: TwoUiIcon) -> Self {
        Self {
            icon,
            size: 38.0,
            icon_size: 20.0,
            tint: None,
            fill: None,
            tooltip: None,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn icon_size(mut self, icon_size: f32) -> Self {
        self.icon_size = icon_size;
        self
    }

    pub fn tint(mut self, tint: Color32) -> Self {
        self.tint = Some(tint);
        self
    }

    pub fn fill(mut self, fill: Color32) -> Self {
        self.fill = Some(fill);
        self
    }

    pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }
}

impl Widget for TwoUiIconButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let is_enabled = ui.is_enabled();
        let sense = if is_enabled {
            Sense::click()
        } else {
            Sense::hover()
        };

        let (rect, mut response) = ui.allocate_exact_size(Vec2::splat(self.size), sense);
        if ui.is_rect_visible(rect) {
            let (press_t, hover_t) = if is_enabled {
                interaction_factors(ui, &response)
            } else {
                (0.0, 0.0)
            };
            let paint_rect = recoil_rect(rect, press_t, 0.8);
            let radius = CornerRadius::same(dimens::CAPSULE_CORNER_RADIUS);

            let bg = match self.fill {
                Some(f) => interactive_fill(f, p, hover_t, 0.0),
                None => {
                    if hover_t > 0.001 {
                        p.hover_overlay.gamma_multiply(hover_t)
                    } else {
                        Color32::TRANSPARENT
                    }
                }
            };

            if bg != Color32::TRANSPARENT {
                ui.painter().rect_filled(rect, radius, bg);
            }
            if press_t > 0.001 {
                paint_pill_press_ripple(ui, rect, radius, p, press_t);
            }

            let fg = self.tint.unwrap_or(p.text_primary);
            let fg = if is_enabled { fg } else { fg.gamma_multiply(0.4) };
            let icon_rect = Rect::from_center_size(paint_rect.center(), Vec2::splat(self.icon_size));
            self.icon.paint(ui, icon_rect, fg);
        }

        if let Some(tip) = self.tooltip {
            response = response.on_hover_text(tip);
        }
        response
    }
}
