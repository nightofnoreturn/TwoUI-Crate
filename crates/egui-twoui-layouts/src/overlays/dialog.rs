//! Modal alert dialog with rounded pill buttons and optional single-choice checkbox.

use egui::{
    pos2, vec2, Align2, Area, Context, CornerRadius, Frame, Id, Margin, Modal, Order, RichText,
    Sense, Stroke, Ui,
};
use egui_twoui_tokens::{
    animate_toggle_ctx, dimens, ease_sine_out_80, font_id_for, show_frosted_frame,
    theme_from_ctx, ElevationTier, TwoUiFontWeight,
};
use egui_twoui_widgets::{
    button::{TwoUiButton, TwoUiButtonStyle},
    controls::TwoUiCheckBox,
};

use super::DialogAction;

/// Modal alert dialog (`26dp` corners, optional checkbox, `22dp` pill buttons, `1dp` hairline dividers).
pub struct TwoUiDialog<'a> {
    id_salt: &'a str,
    title: &'a str,
    body: Option<&'a str>,
    single_choice: Option<(&'a mut bool, &'a str)>,
    actions: Vec<DialogAction<'a>>,
    width: f32,
}

impl<'a> TwoUiDialog<'a> {
    pub fn new(id_salt: &'a str, title: &'a str) -> Self {
        Self {
            id_salt,
            title,
            body: None,
            single_choice: None,
            actions: Vec::new(),
            width: 360.0,
        }
    }

    pub fn body(mut self, body: &'a str) -> Self {
        self.body = Some(body);
        self
    }

    /// Adds an optional single-choice checkbox inside the dialog content area (e.g. `"Don't show again"`).
    pub fn single_choice_option(mut self, checked: &'a mut bool, label: &'a str) -> Self {
        self.single_choice = Some((checked, label));
        self
    }

    pub fn action(mut self, action: DialogAction<'a>) -> Self {
        self.actions.push(action);
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width.clamp(280.0, 520.0);
        self
    }

    /// Shows the modal while `*open` is `true`. Returns the clicked action index, if any.
    pub fn show(
        self,
        ctx: &Context,
        open: &mut bool,
        extra_content: impl FnOnce(&mut Ui),
    ) -> Option<usize> {
        let slide_id = Id::unique((self.id_salt, "twoui_dlg_y"));
        let alpha_id = Id::unique((self.id_salt, "twoui_dlg_a"));
        if !*open {
            ctx.animate_bool_with_time(slide_id, false, 0.0);
            ctx.animate_bool_with_time(alpha_id, false, 0.0);
            return None;
        }

        let slide_t = animate_toggle_ctx(ctx, slide_id, true, 0.30, ease_sine_out_80);
        let alpha_t = animate_toggle_ctx(ctx, alpha_id, true, 0.15, ease_sine_out_80);
        let mut clicked_action = None;
        let theme = theme_from_ctx(ctx);
        let dark = theme.mode.is_dark();
        let p = &theme.palette;

        let tier = if alpha_t > 0.3 {
            ElevationTier::Lg
        } else {
            ElevationTier::Zero
        };
        let area = Area::new(Id::unique((self.id_salt, "twoui_dlg_area")))
            .anchor(Align2::CENTER_CENTER, vec2(0.0, (1.0 - slide_t) * 14.0))
            .order(Order::Foreground);

        let single_choice = self.single_choice;
        let modal_resp = Modal::new(Id::unique(self.id_salt))
            .area(area)
            .backdrop_color(p.backdrop.gamma_multiply(alpha_t))
            .frame(Frame::NONE)
            .show(ctx, |ui| {
                show_frosted_frame(
                    ui,
                    CornerRadius::same(dimens::DIALOG_CORNER_RADIUS),
                    p.floating_surface.gamma_multiply(alpha_t),
                    Margin::symmetric(24, 22),
                    tier,
                    dark,
                    |ui| {
                        ui.multiply_opacity(alpha_t);
                        ui.set_width(self.width);
                        ui.spacing_mut().item_spacing.y = 12.0;

                        ui.label(
                            RichText::new(self.title)
                                .font(font_id_for(
                                    ui.ctx(),
                                    dimens::TEXT_DIALOG_TITLE,
                                    TwoUiFontWeight::SemiBold,
                                ))
                                .color(p.text_primary),
                        );

                        if let Some(body) = self.body {
                            ui.label(
                                RichText::new(body)
                                    .size(dimens::TEXT_BODY)
                                    .color(p.text_secondary),
                            );
                        }

                        if let Some((checked, label)) = single_choice {
                            ui.add(TwoUiCheckBox::new(checked).label(label));
                        }

                        extra_content(ui);

                        if !self.actions.is_empty() {
                            ui.add_space(6.0);
                            let count = self.actions.len();
                            let btn_w = ((self.width - (count as f32 - 1.0) * 12.0) / count as f32)
                                .max(70.0);
                            ui.horizontal(|ui| {
                                for (idx, act) in self.actions.iter().enumerate() {
                                    if idx > 0 {
                                        let (div_rect, _) =
                                            ui.allocate_exact_size(vec2(12.0, 22.0), Sense::hover());
                                        let x = div_rect.center().x;
                                        ui.painter().line_segment(
                                            [
                                                pos2(x, div_rect.top() + 3.0),
                                                pos2(x, div_rect.bottom() - 3.0),
                                            ],
                                            Stroke::new(1.0, p.divider.gamma_multiply(0.85)),
                                        );
                                    }

                                    let style = if act.destructive {
                                        TwoUiButtonStyle::Destructive
                                    } else {
                                        TwoUiButtonStyle::Transparent
                                    };
                                    let mut btn = TwoUiButton::new(act.label)
                                        .style(style)
                                        .corner_radius(dimens::DIALOG_BUTTON_CORNER_RADIUS)
                                        .min_size(vec2(btn_w, 40.0));
                                    if act.primary {
                                        btn = btn.text_color(p.primary_high);
                                    }
                                    if ui.add(btn).clicked() {
                                        clicked_action = Some(idx);
                                    }
                                }
                            });
                        }
                    },
                );
            });

        if modal_resp.backdrop_response.clicked() || ctx.input(|i| i.key_pressed(egui::Key::Escape))
        {
            *open = false;
        }
        if clicked_action.is_some() {
            *open = false;
        }

        if !*open {
            ctx.animate_bool_with_time(slide_id, false, 0.0);
            ctx.animate_bool_with_time(alpha_id, false, 0.0);
        }

        clicked_action
    }
}
