use std::sync::Arc;

use egui::{
    pos2, vec2, Galley, Id, Rect, Response, Role, Sense, Stroke, Ui, Vec2, Widget, WidgetInfo,
};
use egui_twoui_icons::TwoUiIcon;
use egui_twoui_tokens::{
    animate_toggle, ease_sine_out_70, font_id, interaction_factors, theme_from_ui,
    TwoUiFontWeight, TwoUiPalette,
};

const CONTROL_BOX_SIZE: f32 = 22.0;

fn layout_control(
    ui: &mut Ui,
    label: Option<&str>,
    is_enabled: bool,
    p: &TwoUiPalette,
) -> (Rect, Response, Option<Arc<Galley>>) {
    let sense = if is_enabled {
        Sense::click()
    } else {
        Sense::hover()
    };
    let text_col = if is_enabled {
        p.text_primary
    } else {
        p.text_disabled
    };
    let text_galley = label.map(|lbl| {
        ui.painter().layout_no_wrap(
            lbl.to_owned(),
            font_id(ui, 15.5, TwoUiFontWeight::Regular),
            text_col,
        )
    });
    let width = match &text_galley {
        Some(g) => CONTROL_BOX_SIZE + 12.0 + g.size().x,
        None => CONTROL_BOX_SIZE + 4.0,
    };
    let height = text_galley
        .as_ref()
        .map_or(CONTROL_BOX_SIZE + 6.0, |g| g.size().y.max(CONTROL_BOX_SIZE) + 8.0);
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), sense);
    (rect, response, text_galley)
}

fn paint_control_label(
    ui: &Ui,
    rect: Rect,
    galley: Option<Arc<Galley>>,
    is_enabled: bool,
    p: &TwoUiPalette,
) {
    if let Some(galley) = galley {
        let text_pos = pos2(
            rect.left() + CONTROL_BOX_SIZE + 14.0,
            rect.center().y - galley.size().y * 0.5,
        );
        let text_col = if is_enabled {
            p.text_primary
        } else {
            p.text_disabled
        };
        ui.painter().galley(text_pos, galley, text_col);
    }
}

/// Circular animated checkbox.
pub struct TwoUiCheckBox<'a> {
    checked: &'a mut bool,
    label: Option<String>,
    enabled: bool,
}

impl<'a> TwoUiCheckBox<'a> {
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
}

impl Widget for TwoUiCheckBox<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let is_enabled = self.enabled && ui.is_enabled();
        let (rect, mut response, text_galley) =
            layout_control(ui, self.label.as_deref(), is_enabled, p);

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
            let check_t = animate_toggle(
                ui,
                response.id.with("twoui_chk"),
                *self.checked,
                0.133,
                ease_sine_out_70,
            );

            let circle_center = pos2(rect.left() + CONTROL_BOX_SIZE * 0.5 + 2.0, rect.center().y);
            if press_t > 0.001 {
                ui.painter().circle_filled(
                    circle_center,
                    CONTROL_BOX_SIZE * 0.56,
                    p.press_overlay.gamma_multiply(press_t),
                );
            }

            let r = (CONTROL_BOX_SIZE * 0.5) * (1.0 - 0.08 * press_t);
            let alpha = if is_enabled { 1.0 } else { 0.4 };

            if check_t > 0.01 {
                ui.painter().circle_filled(
                    circle_center,
                    r * (0.85 + 0.15 * check_t),
                    p.primary.gamma_multiply(check_t * alpha),
                );
            }
            if check_t < 0.99 {
                let ring_color = p.text_muted.gamma_multiply((1.0 - check_t) * alpha);
                ui.painter()
                    .circle_stroke(circle_center, r, Stroke::new(1.75, ring_color));
            }
            if check_t > 0.25 {
                let icon_rect =
                    Rect::from_center_size(circle_center, Vec2::splat(CONTROL_BOX_SIZE * 0.72));
                TwoUiIcon::Check.paint(ui, icon_rect, p.on_primary.gamma_multiply(check_t * alpha));
            }

            paint_control_label(ui, rect, text_galley, is_enabled, p);
        }

        let checked_val = *self.checked;
        let acc_label = self.label.as_deref().unwrap_or("CheckBox");
        response.widget_info(|| {
            WidgetInfo::selected(Role::CheckBox, is_enabled, checked_val, acc_label)
        });
        response
    }
}

/// Circular animated radio button.
pub struct TwoUiRadioButton {
    selected: bool,
    label: Option<String>,
    enabled: bool,
}

impl TwoUiRadioButton {
    pub fn new(selected: bool) -> Self {
        Self {
            selected,
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

    /// Paints a radio button centered at `center` without allocating UI space.
    pub fn paint_only(
        ui: &Ui,
        anim_id: Id,
        center: egui::Pos2,
        selected: bool,
        enabled: bool,
        press_t: f32,
    ) {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let sel_t = animate_toggle(ui, anim_id, selected, 0.133, ease_sine_out_70);
        let alpha = if enabled { 1.0 } else { 0.4 };
        let outer_r = 10.0 * (1.0 - 0.08 * press_t);

        let ring_color = p
            .text_muted
            .lerp_to_gamma(p.primary, sel_t)
            .gamma_multiply(alpha);
        ui.painter()
            .circle_stroke(center, outer_r, Stroke::new(1.85, ring_color));

        if sel_t > 0.02 {
            ui.painter()
                .circle_filled(center, 5.2 * sel_t, p.primary.gamma_multiply(alpha * sel_t));
        }
    }
}

impl Widget for TwoUiRadioButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let is_enabled = self.enabled && ui.is_enabled();
        let (rect, response, text_galley) =
            layout_control(ui, self.label.as_deref(), is_enabled, p);

        if ui.is_rect_visible(rect) {
            let (press_t, _) = if is_enabled {
                interaction_factors(ui, &response)
            } else {
                (0.0, 0.0)
            };
            let center = pos2(rect.left() + CONTROL_BOX_SIZE * 0.5 + 2.0, rect.center().y);
            if press_t > 0.001 {
                ui.painter().circle_filled(
                    center,
                    CONTROL_BOX_SIZE * 0.56,
                    p.press_overlay.gamma_multiply(press_t),
                );
            }

            Self::paint_only(
                ui,
                response.id.with("twoui_radio"),
                center,
                self.selected,
                is_enabled,
                press_t,
            );
            paint_control_label(ui, rect, text_galley, is_enabled, p);
        }

        let acc_label = self.label.as_deref().unwrap_or("RadioButton");
        response.widget_info(|| {
            WidgetInfo::selected(Role::RadioButton, is_enabled, self.selected, acc_label)
        });
        response
    }
}
