use egui::{
    pos2, vec2, Color32, CornerRadius, Id, Rect, Response, Role, Sense, Ui, Vec2, Widget,
    WidgetInfo,
};
use egui_twoui_tokens::{
    animate_toggle, ease_release, font_id, interaction_factors, theme_from_ui,
    TwoUiFontWeight,
};

/// Animated pill toggle switch.
pub struct TwoUiSwitch<'a> {
    checked: &'a mut bool,
    label: Option<String>,
    enabled: bool,
}

impl<'a> TwoUiSwitch<'a> {
    /// Standard switch dimensions (`32x20dp`).
    pub const DEFAULT_SIZE: Vec2 = vec2(32.0, 20.0);

    pub fn new(checked: &'a mut bool) -> Self {
        Self {
            checked,
            label: None,
            enabled: true,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Paints a switch within `rect` without allocating UI space.
    pub fn paint_only(
        ui: &Ui,
        anim_id: Id,
        rect: Rect,
        checked: bool,
        enabled: bool,
        press_t: f32,
    ) {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let t = animate_toggle(ui, anim_id, checked, 0.30, ease_release);

        let track_off = if enabled {
            p.switch_track_off
        } else if theme.mode.is_dark() {
            Color32::from_rgba_unmultiplied(0x63, 0x63, 0x68, 0x66)
        } else {
            Color32::from_rgba_unmultiplied(0x99, 0x99, 0x9E, 0x66)
        };
        let track_on = if enabled {
            p.primary
        } else {
            p.primary.gamma_multiply(0.4)
        };
        let track_color = track_off.lerp_to_gamma(track_on, t);

        let track_radius = CornerRadius::same((rect.height() * 0.5).round() as u8);
        ui.painter().rect_filled(rect, track_radius, track_color);

        // 16dp thumb inset 2dp inside 32x20dp track with press recoil & scale pulse.
        let half_h = rect.height() * 0.5;
        let thumb_center = pos2(
            egui::lerp((rect.left() + half_h)..=(rect.right() - half_h), t),
            rect.center().y,
        );

        if enabled && press_t > 0.001 {
            ui.painter().circle_filled(
                thumb_center,
                13.5,
                p.press_overlay.gamma_multiply(press_t),
            );
        }

        let thumb_scale = 1.0 - 0.14 * press_t;
        let inner_radius = (half_h - 2.0).max(2.0) * thumb_scale;
        let thumb_fill = if enabled {
            p.switch_thumb
        } else if theme.mode.is_dark() {
            Color32::from_rgb(0x85, 0x85, 0x85)
        } else {
            p.switch_thumb
        };

        ui.painter()
            .circle_filled(thumb_center, inner_radius, thumb_fill);
    }
}

impl Widget for TwoUiSwitch<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let is_enabled = self.enabled && ui.is_enabled();
        let sense = if is_enabled {
            Sense::click()
        } else {
            Sense::hover()
        };

        if let Some(label) = self.label {
            let label_font = font_id(ui, 15.5, TwoUiFontWeight::Regular);
            let galley = ui
                .painter()
                .layout_no_wrap(label.clone(), label_font, p.text_primary);
            let min_w = Self::DEFAULT_SIZE.x + 16.0 + galley.size().x;
            let total_w = ui.available_width().max(min_w);
            let total_h = Self::DEFAULT_SIZE.y.max(galley.size().y).max(34.0);
            let (rect, mut response) = ui.allocate_exact_size(vec2(total_w, total_h), sense);

            if is_enabled && response.clicked() {
                *self.checked = !*self.checked;
                response.mark_changed();
            }

            if ui.is_rect_visible(rect) {
                let (press_t, _) = if is_enabled {
                    interaction_factors(ui, &response)
                } else {
                    (0.0, 0.0)
                };
                let text_color = if is_enabled {
                    p.text_primary
                } else {
                    p.text_disabled
                };
                let text_pos = pos2(rect.left(), rect.center().y - galley.size().y * 0.5);
                ui.painter().galley(text_pos, galley, text_color);

                let switch_rect = Rect::from_min_size(
                    pos2(
                        rect.right() - Self::DEFAULT_SIZE.x,
                        rect.center().y - Self::DEFAULT_SIZE.y * 0.5,
                    ),
                    Self::DEFAULT_SIZE,
                );
                Self::paint_only(
                    ui,
                    response.id.with("sw"),
                    switch_rect,
                    *self.checked,
                    is_enabled,
                    press_t,
                );
            }

            let checked_val = *self.checked;
            response.widget_info(|| {
                WidgetInfo::selected(Role::Switch, is_enabled, checked_val, &label)
            });
            response
        } else {
            let (rect, mut response) = ui.allocate_exact_size(Self::DEFAULT_SIZE, sense);
            if is_enabled && response.clicked() {
                *self.checked = !*self.checked;
                response.mark_changed();
            }
            if ui.is_rect_visible(rect) {
                let (press_t, _) = if is_enabled {
                    interaction_factors(ui, &response)
                } else {
                    (0.0, 0.0)
                };
                Self::paint_only(
                    ui,
                    response.id.with("sw"),
                    rect,
                    *self.checked,
                    is_enabled,
                    press_t,
                );
            }
            let checked_val = *self.checked;
            response.widget_info(|| {
                WidgetInfo::selected(Role::Switch, is_enabled, checked_val, "Switch")
            });
            response
        }
    }
}
