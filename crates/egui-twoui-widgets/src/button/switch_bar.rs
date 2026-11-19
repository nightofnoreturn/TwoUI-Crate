use egui::{
    pos2, vec2, Align2, CornerRadius, FontFamily, FontId, Rect, Response, Role, Sense, Ui, Vec2,
    Widget, WidgetInfo,
};
use egui_twoui_tokens::{
    interaction_factors, interactive_fill, recoil_rect, theme_from_ui, tokens::dimens,
};

use crate::{controls::TwoUiSwitch, progress::TwoUiSpinner};

/// Full-width master toggle bar (`52dp` height, `26dp` corner radius).
pub struct TwoUiSwitchBar<'a> {
    checked: &'a mut bool,
    on_label: String,
    off_label: String,
    show_progress: bool,
}

impl<'a> TwoUiSwitchBar<'a> {
    pub fn new(checked: &'a mut bool) -> Self {
        Self {
            checked,
            on_label: "On".to_owned(),
            off_label: "Off".to_owned(),
            show_progress: false,
        }
    }

    pub fn labels(mut self, on_label: impl Into<String>, off_label: impl Into<String>) -> Self {
        self.on_label = on_label.into();
        self.off_label = off_label.into();
        self
    }

    pub fn show_progress(mut self, show: bool) -> Self {
        self.show_progress = show;
        self
    }
}

impl Widget for TwoUiSwitchBar<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let width = ui.available_width().max(120.0);
        let height = dimens::SWITCH_BAR_HEIGHT;

        let (rect, mut response) = ui.allocate_exact_size(vec2(width, height), Sense::click());
        if response.clicked() {
            *self.checked = !*self.checked;
            response.mark_changed();
        }

        if ui.is_rect_visible(rect) {
            let (press_t, hover_t) = interaction_factors(ui, &response);
            let on_t = ui
                .ctx()
                .animate_bool_responsive(response.id.with("switchbar_on"), *self.checked);

            let off_bg = p.surface;
            let on_bg = if theme.mode.is_dark() {
                p.surface_container
            } else {
                p.surface_container.lerp_to_gamma(p.primary, 0.14)
            };
            let base_bg = off_bg.lerp_to_gamma(on_bg, on_t);
            let fill = interactive_fill(base_bg, p, hover_t, press_t);

            let paint_rect = recoil_rect(rect, press_t, 1.5);
            ui.painter().rect_filled(
                paint_rect,
                CornerRadius::same(dimens::ROUNDED_CORNER_RADIUS),
                fill,
            );

            let label = if *self.checked {
                &self.on_label
            } else {
                &self.off_label
            };
            let label_color = p.text_primary.lerp_to_gamma(p.primary_high, on_t);
            ui.painter().text(
                pos2(paint_rect.left() + 24.0, paint_rect.center().y),
                Align2::LEFT_CENTER,
                label,
                FontId::new(17.0, FontFamily::Proportional),
                label_color,
            );

            let switch_size = TwoUiSwitch::DEFAULT_SIZE;
            let switch_right = paint_rect.right() - 20.0;
            let switch_rect = Rect::from_min_size(
                pos2(
                    switch_right - switch_size.x,
                    paint_rect.center().y - switch_size.y * 0.5,
                ),
                switch_size,
            );
            TwoUiSwitch::paint_only(
                ui,
                response.id.with("bar_sw"),
                switch_rect,
                *self.checked,
                true,
                press_t,
            );

            if self.show_progress {
                let spinner_rect = Rect::from_center_size(
                    pos2(switch_rect.left() - 22.0, paint_rect.center().y),
                    Vec2::splat(20.0),
                );
                TwoUiSpinner::paint_indeterminate_at(ui, spinner_rect, 2.4, p);
            }
        }

        let checked_val = *self.checked;
        response.widget_info(|| {
            WidgetInfo::selected(Role::Switch, ui.is_enabled(), checked_val, "Master switch")
        });
        response
    }
}
