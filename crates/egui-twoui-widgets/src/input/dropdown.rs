use egui::{
    pos2, vec2, Color32, CornerRadius, FontFamily, FontId, Frame, Margin, Popup, Rect, Response,
    Sense, Stroke, StrokeKind, Ui, Vec2,
};
use egui_twoui_icons::TwoUiIcon;
use egui_twoui_tokens::{
    interaction_factors, interactive_fill, recoil_rect, show_frosted_frame, theme_from_ui,
    tokens::dimens, ElevationTier,
};

/// Pill dropdown selector (`26dp` rounded popup menu with checkmark).
pub struct TwoUiDropdown<'a> {
    id_salt: &'a str,
    options: &'a [&'a str],
    selected: &'a mut usize,
    min_width: f32,
}

impl<'a> TwoUiDropdown<'a> {
    pub fn new(id_salt: &'a str, options: &'a [&'a str], selected: &'a mut usize) -> Self {
        Self {
            id_salt,
            options,
            selected,
            min_width: 170.0,
        }
    }

    pub fn min_width(mut self, width: f32) -> Self {
        self.min_width = width;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        ui.push_id(self.id_salt, |ui| {
            let theme = theme_from_ui(ui);
            let p = &theme.palette;
            let count = self.options.len();
            if count > 0 {
                *self.selected = (*self.selected).min(count - 1);
            }

            let current_label = self
                .options
                .get(*self.selected)
                .copied()
                .unwrap_or("Select");

            let font_id = FontId::new(16.0, FontFamily::Proportional);
            let galley = ui
                .painter()
                .layout_no_wrap(current_label.to_owned(), font_id, p.text_primary);
            let btn_w = (galley.size().x + 48.0).max(110.0);
            let btn_h = 38.0;

            let (rect, mut response) = ui.allocate_exact_size(vec2(btn_w, btn_h), Sense::click());
            if ui.is_rect_visible(rect) {
                let (press_t, hover_t) = interaction_factors(ui, &response);
                let paint_rect = recoil_rect(rect, press_t, 1.0);
                let bg = interactive_fill(p.surface_container, p, hover_t, press_t);
                ui.painter()
                    .rect_filled(paint_rect, CornerRadius::same(18), bg);

                ui.painter().galley(
                    pos2(
                        paint_rect.left() + 16.0,
                        paint_rect.center().y - galley.size().y * 0.5,
                    ),
                    galley,
                    p.text_primary,
                );
                let chevron_rect = Rect::from_center_size(
                    pos2(paint_rect.right() - 18.0, paint_rect.center().y),
                    Vec2::splat(16.0),
                );
                TwoUiIcon::ChevronDown.paint(ui, chevron_rect, p.text_muted);
            }

            let mut selection_changed = false;
            let dark = theme.mode.is_dark();

            Popup::menu(&response)
                .frame(Frame::NONE)
                .width(self.min_width.max(btn_w))
                .show(|ui| {
                    show_frosted_frame(
                        ui,
                        CornerRadius::same(dimens::DIALOG_CORNER_RADIUS),
                        p.floating_surface,
                        Margin::symmetric(8, 8),
                        ElevationTier::Md,
                        dark,
                        |ui| {
                            ui.spacing_mut().item_spacing.y = 2.0;
                            for (i, option) in self.options.iter().enumerate() {
                                let is_sel = *self.selected == i;
                                let item_w = ui.available_width().max(self.min_width - 16.0);
                                let (item_rect, item_resp) =
                                    ui.allocate_exact_size(vec2(item_w, 42.0), Sense::click());

                                if ui.is_rect_visible(item_rect) {
                                    let (press_t, hover_t) = interaction_factors(ui, &item_resp);
                                    let fill = if is_sel {
                                        interactive_fill(p.selected_bg, p, hover_t, press_t)
                                    } else {
                                        let mut c = Color32::TRANSPARENT;
                                        if hover_t > 0.001 {
                                            c = p.hover_overlay.gamma_multiply(hover_t);
                                        }
                                        if press_t > 0.001 {
                                            c = c.blend(p.press_overlay.gamma_multiply(press_t));
                                        }
                                        c
                                    };

                                    if fill != Color32::TRANSPARENT {
                                        ui.painter().rect(
                                            recoil_rect(item_rect, press_t, 0.8),
                                            CornerRadius::same(18),
                                            fill,
                                            Stroke::NONE,
                                            StrokeKind::Inside,
                                        );
                                    }

                                    let text_col =
                                        if is_sel { p.primary_high } else { p.text_primary };
                                    let og = ui.painter().layout_no_wrap(
                                        (*option).to_owned(),
                                        FontId::new(15.5, FontFamily::Proportional),
                                        text_col,
                                    );
                                    ui.painter().galley(
                                        pos2(
                                            item_rect.left() + 14.0,
                                            item_rect.center().y - og.size().y * 0.5,
                                        ),
                                        og,
                                        text_col,
                                    );

                                    if is_sel {
                                        let chk_rect = Rect::from_center_size(
                                            pos2(item_rect.right() - 18.0, item_rect.center().y),
                                            Vec2::splat(16.0),
                                        );
                                        TwoUiIcon::Check.paint(ui, chk_rect, p.primary_high);
                                    }
                                }

                                if item_resp.clicked() {
                                    if *self.selected != i {
                                        *self.selected = i;
                                        selection_changed = true;
                                    }
                                    ui.close();
                                }
                            }
                        },
                    );
                });

            if selection_changed {
                response.mark_changed();
            }
            response
        })
        .inner
    }
}
