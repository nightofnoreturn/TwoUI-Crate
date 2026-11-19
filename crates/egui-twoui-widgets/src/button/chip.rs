use egui::{
    pos2, vec2, Color32, CornerRadius, Rect, Response, Role, Sense, Stroke, StrokeKind, Ui, Vec2,
    Widget, WidgetInfo,
};
use egui_twoui_icons::TwoUiIcon;
use egui_twoui_tokens::{
    animate_toggle, ease_release, font_id, interaction_factors, interactive_fill,
    paint_pill_press_ripple, recoil_rect, theme_from_ui, tokens::dimens, TwoUiFontWeight,
};

/// Capsule filter/choice chip (`34dp` height).
pub struct TwoUiChip {
    label: String,
    icon: Option<TwoUiIcon>,
    selected: bool,
    enabled: bool,
}

impl TwoUiChip {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            selected: false,
            enabled: true,
        }
    }

    pub fn icon(mut self, icon: TwoUiIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl Widget for TwoUiChip {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let is_enabled = self.enabled && ui.is_enabled();

        let font = font_id(
            ui,
            13.5,
            if self.selected {
                TwoUiFontWeight::SemiBold
            } else {
                TwoUiFontWeight::Medium
            },
        );
        let g = ui
            .painter()
            .layout_no_wrap(self.label.clone(), font, Color32::WHITE);
        let icon_w = if self.icon.is_some() { 16.0 } else { 0.0 };
        let icon_gap = if self.icon.is_some() { 6.0 } else { 0.0 };
        let width = (g.size().x + icon_w + icon_gap + 28.0).max(58.0);
        let height = 34.0;

        let sense = if is_enabled {
            Sense::click()
        } else {
            Sense::hover()
        };
        let (rect, response) = ui.allocate_exact_size(vec2(width, height), sense);

        if ui.is_rect_visible(rect) {
            let (pt, ht) = if is_enabled {
                interaction_factors(ui, &response)
            } else {
                (0.0, 0.0)
            };
            let paint_r = recoil_rect(rect, pt, 0.6);
            let sel_t = animate_toggle(
                ui,
                response.id.with("chip_sel"),
                self.selected,
                0.15,
                ease_release,
            );
            let alpha = if is_enabled { 1.0 } else { 0.42 };

            let base_fill = p
                .surface_container
                .lerp_to_gamma(p.primary.gamma_multiply(0.18), sel_t);
            let fill = interactive_fill(base_fill, p, ht, 0.0).gamma_multiply(alpha);
            let border = if self.selected {
                Stroke::new(1.0, p.primary.gamma_multiply(0.65 * alpha))
            } else {
                Stroke::new(0.75, p.divider.gamma_multiply(alpha))
            };
            let chip_r = CornerRadius::same(dimens::CAPSULE_CORNER_RADIUS);
            ui.painter()
                .rect(rect, chip_r, fill, border, StrokeKind::Inside);
            if pt > 0.001 {
                paint_pill_press_ripple(ui, rect, chip_r, p, pt);
            }

            let fg = p
                .text_primary
                .lerp_to_gamma(p.primary_high, sel_t)
                .gamma_multiply(alpha);
            let total_w = icon_w + icon_gap + g.size().x;
            let sx = paint_r.center().x - total_w * 0.5;
            if let Some(icon) = self.icon {
                icon.paint(
                    ui,
                    Rect::from_center_size(
                        pos2(sx + icon_w * 0.5, paint_r.center().y),
                        Vec2::splat(icon_w),
                    ),
                    fg,
                );
            }
            ui.painter().galley(
                pos2(sx + icon_w + icon_gap, paint_r.center().y - g.size().y * 0.5),
                g,
                fg,
            );
        }

        response.widget_info(|| {
            WidgetInfo::selected(Role::Button, is_enabled, self.selected, &self.label)
        });
        response
    }
}
