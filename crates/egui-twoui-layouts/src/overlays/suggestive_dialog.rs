//! Dark high-contrast suggestive modal dialog (`26dp` corners, primary pill CTA).

use egui::{
    pos2, vec2, Align2, Area, Color32, Context, CornerRadius, Frame, Id, Margin, Modal, Order,
    RichText, Sense,
};
use egui_twoui_icons::TwoUiIcon;
use egui_twoui_tokens::{
    animate_toggle_ctx, colors, dimens, ease_sine_out_80, font_id_for, gray,
    interaction_factors, interactive_fill, recoil_rect, show_frosted_frame, theme_from_ctx,
    ElevationTier, TwoUiFontWeight,
};

use super::DialogAction;

/// High-contrast dark dialog (`#2D2D30`, `26dp` corners, pill primary CTA, and translucent secondary action).
pub struct TwoUiSuggestiveDialog<'a> {
    id_salt: &'a str,
    title: &'a str,
    body: &'a str,
    icon: Option<TwoUiIcon>,
    actions: Vec<DialogAction<'a>>,
    width: f32,
}

impl<'a> TwoUiSuggestiveDialog<'a> {
    pub fn new(id_salt: &'a str, title: &'a str, body: &'a str) -> Self {
        Self {
            id_salt,
            title,
            body,
            icon: None,
            actions: Vec::new(),
            width: 344.0,
        }
    }

    pub fn icon(mut self, icon: TwoUiIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn action(mut self, action: DialogAction<'a>) -> Self {
        self.actions.push(action);
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width.clamp(280.0, 440.0);
        self
    }

    /// Shows the suggestive dialog while `*open` is `true`. Returns the clicked action index, if any.
    pub fn show(self, ctx: &Context, open: &mut bool) -> Option<usize> {
        let slide_id = Id::unique((self.id_salt, "twoui_sdlg_y"));
        let alpha_id = Id::unique((self.id_salt, "twoui_sdlg_a"));
        if !*open {
            ctx.animate_bool_with_time(slide_id, false, 0.0);
            ctx.animate_bool_with_time(alpha_id, false, 0.0);
            return None;
        }

        let slide_t = animate_toggle_ctx(ctx, slide_id, true, 0.30, ease_sine_out_80);
        let alpha_t = animate_toggle_ctx(ctx, alpha_id, true, 0.15, ease_sine_out_80);
        let mut clicked = None;
        let theme = theme_from_ctx(ctx);
        let p = &theme.palette;

        let tier = if alpha_t > 0.3 {
            ElevationTier::Lg
        } else {
            ElevationTier::Zero
        };
        let area = Area::new(Id::unique((self.id_salt, "twoui_sdlg_area")))
            .anchor(Align2::CENTER_CENTER, vec2(0.0, (1.0 - slide_t) * 14.0))
            .order(Order::Foreground);

        let modal_resp = Modal::new(Id::unique(self.id_salt))
            .area(area)
            .backdrop_color(p.backdrop.gamma_multiply(alpha_t))
            .frame(Frame::NONE)
            .show(ctx, |ui| {
                show_frosted_frame(
                    ui,
                    CornerRadius::same(dimens::DIALOG_CORNER_RADIUS),
                    colors::SUGGEST_DIALOG_BG.gamma_multiply(alpha_t),
                    Margin::symmetric(20, 20),
                    tier,
                    true,
                    |ui| {
                        ui.multiply_opacity(alpha_t);
                        ui.set_width(self.width);
                        ui.spacing_mut().item_spacing.y = 12.0;

                        ui.horizontal(|ui| {
                            if let Some(icon) = self.icon {
                                icon.show(ui, 28.0, gray::GRAY_01);
                                ui.add_space(10.0);
                            }
                            ui.label(
                                RichText::new(self.title)
                                    .font(font_id_for(ui.ctx(), 18.0, TwoUiFontWeight::SemiBold))
                                    .color(gray::GRAY_01),
                            );
                        });

                        ui.label(
                            RichText::new(self.body)
                                .size(14.5)
                                .color(gray::GRAY_20),
                        );

                        if !self.actions.is_empty() {
                            ui.add_space(6.0);
                            let count = self.actions.len();
                            let gap = 10.0;
                            let btn_w =
                                ((self.width - (count as f32 - 1.0) * gap) / count as f32).max(80.0);
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = gap;
                                for (idx, act) in self.actions.iter().enumerate() {
                                    let (fill, fg) = if act.primary {
                                        (gray::WHITE, gray::GRAY_90)
                                    } else if act.destructive {
                                        (
                                            Color32::from_rgba_unmultiplied(0xF5, 0x69, 0x62, 0x2E),
                                            colors::RED_DARK,
                                        )
                                    } else {
                                        (
                                            Color32::from_rgba_unmultiplied(0xFF, 0xFF, 0xFF, 0x20),
                                            gray::GRAY_01,
                                        )
                                    };
                                    let (r, resp) =
                                        ui.allocate_exact_size(vec2(btn_w, 42.0), Sense::click());
                                    if ui.is_rect_visible(r) {
                                        let (pt, ht) = interaction_factors(ui, &resp);
                                        let paint_r = recoil_rect(r, pt, 1.4);
                                        let final_fill = interactive_fill(fill, p, ht * 0.7, pt);
                                        ui.painter().rect_filled(
                                            paint_r,
                                            CornerRadius::same(dimens::CAPSULE_CORNER_RADIUS),
                                            final_fill,
                                        );
                                        let g = ui.painter().layout_no_wrap(
                                            act.label.to_owned(),
                                            font_id_for(ui.ctx(), 15.0, TwoUiFontWeight::Medium),
                                            fg,
                                        );
                                        ui.painter().galley(
                                            pos2(
                                                paint_r.center().x - g.size().x * 0.5,
                                                paint_r.center().y - g.size().y * 0.5,
                                            ),
                                            g,
                                            fg,
                                        );
                                    }
                                    if resp.clicked() {
                                        clicked = Some(idx);
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
        if clicked.is_some() {
            *open = false;
        }

        if !*open {
            ctx.animate_bool_with_time(slide_id, false, 0.0);
            ctx.animate_bool_with_time(alpha_id, false, 0.0);
        }

        clicked
    }
}
