//! Floating tip balloon popup and transient pill toast overlay.

use egui::{
    epaint::PathStroke, pos2, vec2, Area, Color32, Context, CornerRadius, Frame, Id, Margin, Order,
    Pos2, RichText, Sense, Shape, Ui,
};
use egui_twoui_tokens::{
    animate_toggle_ctx, colors, ease_sine_out_80, elevation_shadow, font_id, font_id_for,
    show_frosted_frame, theme_from_ui, ElevationTier, TwoUiFontWeight,
};

/// Dark balloon tooltip (`#474747`) with downward pointer tail and optional action button.
pub struct TwoUiTipPopup<'a> {
    id_salt: &'a str,
    message: &'a str,
    action_label: Option<&'a str>,
}

impl<'a> TwoUiTipPopup<'a> {
    pub fn new(id_salt: &'a str, message: &'a str) -> Self {
        Self {
            id_salt,
            message,
            action_label: None,
        }
    }

    pub fn action(mut self, label: &'a str) -> Self {
        self.action_label = Some(label);
        self
    }

    /// Shows the balloon above `anchor_pos` while `*open` is true. Returns `true` if action clicked.
    pub fn show(self, ctx: &Context, anchor_pos: Pos2, open: &mut bool) -> bool {
        let anim_id = Id::unique((self.id_salt, "twoui_tip_a"));
        if !*open {
            ctx.animate_bool_with_time(anim_id, false, 0.0);
            return false;
        }

        let tip_t = animate_toggle_ctx(ctx, anim_id, true, 0.15, ease_sine_out_80);
        let mut action_clicked = false;
        let balloon_pos = pos2(
            (anchor_pos.x - 130.0).max(16.0),
            (anchor_pos.y - 78.0 + (1.0 - tip_t) * 6.0).max(16.0),
        );
        let tip_bg = colors::TIP_POPUP_BG.gamma_multiply(tip_t);

        Area::new(Id::unique(self.id_salt))
            .fixed_pos(balloon_pos)
            .order(Order::Foreground)
            .show(ctx, |ui| {
                ui.multiply_opacity(tip_t);
                let mut frame = Frame::NONE
                    .fill(tip_bg)
                    .corner_radius(CornerRadius::same(22))
                    .inner_margin(Margin::symmetric(18, 12));
                if tip_t > 0.3 {
                    frame = frame.shadow(elevation_shadow(ElevationTier::Md, true));
                }
                let resp = frame.show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 12.0;
                        ui.label(
                            RichText::new(self.message)
                                .size(14.5)
                                .color(colors::TIP_POPUP_TEXT),
                        );
                        if let Some(act) = self.action_label {
                            let font = font_id_for(ui.ctx(), 14.5, TwoUiFontWeight::Medium);
                            let g = ui.painter().layout_no_wrap(
                                act.to_owned(),
                                font,
                                colors::PRIMARY_BLUE_HIGH_DARK,
                            );
                            let (r, btn_resp) =
                                ui.allocate_exact_size(g.size() + vec2(14.0, 10.0), Sense::click());
                            if btn_resp.hovered() {
                                ui.painter().rect_filled(
                                    r,
                                    CornerRadius::same(14),
                                    Color32::from_black_alpha(36),
                                );
                            }
                            ui.painter().galley(
                                pos2(r.center().x - g.size().x * 0.5, r.center().y - g.size().y * 0.5),
                                g,
                                colors::PRIMARY_BLUE_HIGH_DARK,
                            );
                            if btn_resp.clicked() {
                                action_clicked = true;
                            }
                        }
                    });
                });

                let box_rect = resp.response.rect;
                let tip_x = anchor_pos.x.clamp(box_rect.left() + 24.0, box_rect.right() - 24.0);
                let tail = vec![
                    pos2(tip_x - 8.0, box_rect.bottom() - 1.0),
                    pos2(tip_x + 8.0, box_rect.bottom() - 1.0),
                    pos2(tip_x, box_rect.bottom() + 9.0),
                ];
                ui.painter()
                    .add(Shape::convex_polygon(tail, tip_bg, PathStroke::NONE));
            });

        if action_clicked || ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            *open = false;
        }
        if !*open {
            ctx.animate_bool_with_time(anim_id, false, 0.0);
        }
        action_clicked
    }
}

/// Transient bottom pill toast manager.
#[derive(Debug, Default, Clone)]
pub struct TwoUiToastState {
    message: Option<String>,
    shown_at: f64,
    expires_at: f64,
}

impl TwoUiToastState {
    /// Schedules a toast message for `duration_secs` (typically `2.2`–`3.0`).
    pub fn show_message(&mut self, ctx: &Context, message: impl Into<String>, duration_secs: f64) {
        let now = ctx.input(|i| i.time);
        self.message = Some(message.into());
        self.shown_at = now;
        self.expires_at = now + duration_secs.max(0.5);
        ctx.request_repaint();
    }

    /// Renders the frosted pill toast near the bottom center if still active.
    pub fn ui(&mut self, ui: &Ui) {
        let Some(msg) = &self.message else {
            return;
        };
        let now = ui.input(|i| i.time);
        if now >= self.expires_at {
            self.message = None;
            return;
        }

        ui.ctx().request_repaint();
        let elapsed = (now - self.shown_at).max(0.0) as f32;
        let remaining = (self.expires_at - now) as f32;
        let enter_alpha = ease_sine_out_80((elapsed / 0.15).clamp(0.0, 1.0));
        let exit_alpha = (remaining / 0.25).clamp(0.0, 1.0);
        let alpha = enter_alpha * exit_alpha;
        let theme = theme_from_ui(ui);

        let bg = if theme.mode.is_dark() {
            colors::SUGGEST_SNACKBAR_BG
        } else {
            Color32::from_rgba_unmultiplied(0x25, 0x25, 0x28, 0xE6)
        }
        .gamma_multiply(alpha);
        let fg = Color32::from_rgb(0xFA, 0xFA, 0xFF).gamma_multiply(alpha);

        let screen_rect = ui.ctx().content_rect();
        let max_text_w = (screen_rect.width() - 84.0).clamp(160.0, 480.0);
        let galley = ui.painter().layout(
            msg.to_owned(),
            font_id(ui, 14.0, TwoUiFontWeight::Regular),
            fg,
            max_text_w,
        );
        let galley_size = galley.size();
        let pill_size = vec2(galley_size.x + 42.0, galley_size.y + 26.0);
        let pos = pos2(
            screen_rect.center().x - pill_size.x * 0.5,
            (screen_rect.bottom() - 88.0 - pill_size.y).max(screen_rect.top() + 16.0),
        );

        Area::new(Id::unique("egui_twoui_toast_overlay"))
            .fixed_pos(pos)
            .order(Order::Foreground)
            .interactable(false)
            .show(ui.ctx(), |ui| {
                show_frosted_frame(
                    ui,
                    CornerRadius::same(22),
                    bg,
                    Margin::symmetric(21, 13),
                    ElevationTier::Md,
                    true,
                    |ui| {
                        let (r, _) = ui.allocate_exact_size(galley_size, Sense::hover());
                        ui.painter().galley(r.min, galley, fg);
                    },
                );
            });
    }
}
