use egui::{
    pos2, vec2, Align2, Area, Color32, Context, CornerRadius, Id, Order, Rect, Sense,
};
use egui_twoui_tokens::{
    font_id, interaction_factors, interactive_fill, paint_frosted_surface, recoil_rect,
    theme_from_ctx, tokens::{colors, dimens, gray}, ElevationTier, TwoUiFontWeight,
};

/// Floating capsule snackbar manager (`#CC2D2D30`, `22dp` corner radius, spring expansion, and action pill).
#[derive(Debug)]
pub struct TwoUiSnackBarState {
    message: String,
    action_label: Option<String>,
    shown_at: Option<f64>,
    expires_at: Option<f64>,
    bottom_offset: f32,
}

impl Default for TwoUiSnackBarState {
    fn default() -> Self {
        Self {
            message: String::new(),
            action_label: None,
            shown_at: None,
            expires_at: None,
            bottom_offset: 88.0,
        }
    }
}

impl TwoUiSnackBarState {
    pub fn show_message(&mut self, ctx: &Context, message: impl Into<String>, action: Option<&str>) {
        let now = ctx.input(|i| i.time);
        self.message = message.into();
        self.action_label = action.map(str::to_owned);
        self.shown_at = Some(now);
        self.expires_at = Some(now + 4.2);
        ctx.request_repaint();
    }

    /// Sets vertical clearance above the bottom edge (defaults to `88.0` to clear floating bars).
    pub fn set_bottom_offset(&mut self, offset: f32) {
        self.bottom_offset = offset.max(16.0);
    }

    pub fn dismiss(&mut self) {
        self.expires_at = None;
        self.shown_at = None;
    }

    /// Paints the animated floating snackbar; returns `true` when the action pill is clicked.
    pub fn render(&mut self, ctx: &Context, host_rect: Rect) -> bool {
        let (Some(shown_at), Some(exp)) = (self.shown_at, self.expires_at) else {
            return false;
        };
        let now = ctx.input(|i| i.time);
        if now >= exp {
            self.dismiss();
            return false;
        }
        ctx.request_repaint();

        let elapsed = (now - shown_at) as f32;
        let remaining = (exp - now) as f32;

        let expand_raw = (elapsed / 0.34).clamp(0.0, 1.0);
        let spring_expand = if expand_raw >= 1.0 {
            1.0
        } else {
            let decay = (-8.5 * expand_raw).exp();
            (1.0 - decay * (10.5 * expand_raw).cos()).clamp(0.0, 1.06)
        };
        let content_alpha =
            ((elapsed - 0.14) / 0.16).clamp(0.0, 1.0) * (remaining / 0.25).clamp(0.0, 1.0);
        let outer_alpha = (elapsed / 0.10).clamp(0.0, 1.0) * (remaining / 0.25).clamp(0.0, 1.0);

        let theme = theme_from_ctx(ctx);
        let p = &theme.palette;
        let mut action_clicked = false;

        Area::new(Id::unique("twoui_snackbar_overlay"))
            .order(Order::Foreground)
            .anchor(Align2::CENTER_BOTTOM, vec2(0.0, -self.bottom_offset))
            .pivot(Align2::CENTER_BOTTOM)
            .show(ctx, |ui| {
                let text_col = gray::GRAY_01.gamma_multiply(content_alpha);
                let has_action = self.action_label.is_some();
                let action_reserve = if has_action { 116.0 } else { 44.0 };
                let max_msg_w = (host_rect.width() - 48.0 - action_reserve).clamp(140.0, 420.0);

                let msg_galley = ui.painter().layout(
                    self.message.clone(),
                    font_id(ui, 14.0, TwoUiFontWeight::Regular),
                    text_col,
                    max_msg_w,
                );
                let msg_sz = msg_galley.size();

                let action_galley = self.action_label.as_ref().map(|act| {
                    ui.painter().layout_no_wrap(
                        act.clone(),
                        font_id(ui, 13.0, TwoUiFontWeight::SemiBold),
                        gray::GRAY_100,
                    )
                });
                let action_w = action_galley
                    .as_ref()
                    .map_or(0.0, |g| g.size().x + 32.0 + 10.0);

                let target_w =
                    (20.0 + msg_sz.x + action_w + if has_action { 6.0 } else { 20.0 }).max(100.0);
                let target_h = (msg_sz.y + 18.0).max(44.0);

                let cur_w = egui::lerp(44.0..=target_w, spring_expand).min(host_rect.width() - 24.0);
                let cur_h = egui::lerp(44.0..=target_h, spring_expand);

                let (rect, _) = ui.allocate_exact_size(vec2(cur_w, cur_h), Sense::hover());
                let fill = colors::SUGGEST_SNACKBAR_BG.gamma_multiply(outer_alpha);
                let stroke_col = Color32::from_rgba_unmultiplied(0xFF, 0xFF, 0xFF, 0x22)
                    .gamma_multiply(outer_alpha);
                paint_frosted_surface(
                    ui,
                    rect,
                    CornerRadius::same(22),
                    fill,
                    stroke_col,
                    ElevationTier::Md,
                    true,
                );

                if content_alpha > 0.05 {
                    let clip = ui.painter().with_clip_rect(rect);
                    let msg_x = rect.left() + 20.0;
                    let msg_y = rect.center().y - msg_sz.y * 0.5;
                    clip.galley(pos2(msg_x, msg_y), msg_galley, text_col);

                    if let Some(ag) = action_galley {
                        let btn_w = ag.size().x + 32.0;
                        let btn_h = 32.0;
                        let btn_rect = Rect::from_min_size(
                            pos2(rect.right() - 6.0 - btn_w, rect.center().y - btn_h * 0.5),
                            vec2(btn_w, btn_h),
                        );
                        let aresp = ui.interact(
                            btn_rect,
                            Id::unique("twoui_snackbar_act"),
                            Sense::click(),
                        );
                        let (pt, ht) = interaction_factors(ui, &aresp);
                        let paint_r = recoil_rect(btn_rect, pt, 1.0);
                        let btn_fill = interactive_fill(
                            colors::SUGGEST_SNACKBAR_ACTION_BG,
                            p,
                            ht * 0.5,
                            pt,
                        )
                        .gamma_multiply(content_alpha);
                        clip.rect_filled(
                            paint_r,
                            CornerRadius::same(dimens::CAPSULE_CORNER_RADIUS),
                            btn_fill,
                        );
                        let ag_sz = ag.size();
                        clip.galley(
                            pos2(
                                paint_r.center().x - ag_sz.x * 0.5,
                                paint_r.center().y - ag_sz.y * 0.5,
                            ),
                            ag,
                            gray::GRAY_100.gamma_multiply(content_alpha),
                        );
                        if aresp.clicked() {
                            action_clicked = true;
                            self.dismiss();
                        }
                    }
                }
            });

        action_clicked
    }
}
