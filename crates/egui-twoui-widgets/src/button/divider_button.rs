use egui::{pos2, vec2, Color32, CornerRadius, FontFamily, FontId, Id, Rect, Sense, Stroke, Ui, Vec2};
use egui_twoui_icons::TwoUiIcon;
use egui_twoui_tokens::{
    concentric_radius, interaction_factors, interactive_fill, paint_floating_capsule,
    paint_pill_press_ripple, recoil_rect, theme_from_ui, tokens::dimens,
};

/// Segment descriptor for [`TwoUiDividerButtonLayout`].
#[derive(Debug, Clone, Copy)]
pub struct DividerButtonItem<'a> {
    pub label: &'a str,
    pub icon: Option<TwoUiIcon>,
    pub enabled: bool,
}

impl<'a> DividerButtonItem<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            icon: None,
            enabled: true,
        }
    }

    pub fn icon(mut self, icon: TwoUiIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// Floating horizontal split action capsule (`52dp` button height, `6dp` padding,
/// hairline dividers, `ElevationTier::Md`).
pub struct TwoUiDividerButtonLayout<'a> {
    id_salt: &'a str,
    items: &'a [DividerButtonItem<'a>],
    selected: Option<usize>,
}

impl<'a> TwoUiDividerButtonLayout<'a> {
    pub fn new(id_salt: &'a str, items: &'a [DividerButtonItem<'a>]) -> Self {
        Self {
            id_salt,
            items,
            selected: None,
        }
    }

    /// Highlights a selected segment index cleanly (useful for floating mode toggles).
    pub fn selected(mut self, selected: Option<usize>) -> Self {
        self.selected = selected;
        self
    }

    /// Renders the borderless `DividerButtonLayout` pill and returns the clicked button index, if any.
    pub fn show(self, ui: &mut Ui) -> Option<usize> {
        if self.items.is_empty() {
            return None;
        }

        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let dark = theme.mode.is_dark();
        let count = self.items.len();

        let pad = dimens::DIVIDER_BUTTON_PADDING;
        let btn_h = dimens::DIVIDER_BUTTON_HEIGHT;
        let total_h = btn_h + pad * 2.0;

        let font = FontId::new(15.5, FontFamily::Proportional);
        let mut ideal_item_w = 90.5_f32;
        for item in self.items {
            let text_w = ui
                .painter()
                .layout_no_wrap(item.label.to_owned(), font.clone(), Color32::WHITE)
                .size()
                .x;
            let icon_w = if item.icon.is_some() { 22.0 } else { 0.0 };
            let w = (text_w + icon_w + 24.0).clamp(90.5, 173.5);
            ideal_item_w = ideal_item_w.max(w);
        }

        let div_slot_w = 1.0;
        let max_total_w =
            (count as f32 * ideal_item_w + (count - 1) as f32 * div_slot_w + pad * 2.0)
                .min(ui.available_width().max(160.0));
        let slot_w =
            ((max_total_w - pad * 2.0 - (count - 1) as f32 * div_slot_w) / count as f32).max(64.0);

        let (rect, _) = ui.allocate_exact_size(vec2(max_total_w, total_h), Sense::hover());
        let capsule_r = CornerRadius::same(dimens::CAPSULE_CORNER_RADIUS);
        let inner_r = concentric_radius(dimens::CAPSULE_CORNER_RADIUS, pad);

        if ui.is_rect_visible(rect) {
            paint_floating_capsule(ui, rect, capsule_r, dark, p);
        }

        let mut clicked = None;
        let div_color = p
            .outline_button
            .gamma_multiply(dimens::DIVIDER_BUTTON_DIVIDER_ALPHA);

        for (i, item) in self.items.iter().enumerate() {
            let x = rect.left() + pad + i as f32 * (slot_w + div_slot_w);
            if i > 0 && ui.is_rect_visible(rect) {
                let dx = x - div_slot_w * 0.5;
                let half_div = dimens::DIVIDER_BUTTON_DIVIDER_HEIGHT * 0.5;
                ui.painter().line_segment(
                    [
                        pos2(dx, rect.center().y - half_div),
                        pos2(dx, rect.center().y + half_div),
                    ],
                    Stroke::new(1.0, div_color),
                );
            }

            let slot_rect = Rect::from_min_size(pos2(x, rect.top() + pad), vec2(slot_w, btn_h));
            let sense = if item.enabled {
                Sense::click()
            } else {
                Sense::hover()
            };
            let resp = ui.interact(
                slot_rect,
                Id::unique((self.id_salt, "twoui_div_btn", i)),
                sense,
            );
            if item.enabled && resp.clicked() {
                clicked = Some(i);
            }

            if ui.is_rect_visible(slot_rect) {
                let (pt, ht) = if item.enabled {
                    interaction_factors(ui, &resp)
                } else {
                    (0.0, 0.0)
                };
                let is_sel = self.selected == Some(i);
                let pill_rect = slot_rect.shrink2(vec2(1.0, 0.0));
                let paint_r = recoil_rect(pill_rect, pt, 0.6);
                let base_bg = if is_sel {
                    p.selected_bg
                } else {
                    Color32::TRANSPARENT
                };
                let fill = if base_bg == Color32::TRANSPARENT {
                    if ht > 0.001 {
                        p.hover_overlay.gamma_multiply(ht)
                    } else {
                        Color32::TRANSPARENT
                    }
                } else {
                    interactive_fill(base_bg, p, ht * 0.7, 0.0)
                };
                if fill != Color32::TRANSPARENT {
                    ui.painter().rect_filled(pill_rect, inner_r, fill);
                }
                if pt > 0.001 {
                    paint_pill_press_ripple(ui, pill_rect, inner_r, p, pt);
                }

                let fg = if !item.enabled {
                    p.text_disabled
                } else if is_sel {
                    p.primary_high
                } else {
                    p.text_primary
                };
                let label_galley =
                    ui.painter()
                        .layout_no_wrap(item.label.to_owned(), font.clone(), fg);
                let icon_w = if item.icon.is_some() { 18.0 } else { 0.0 };
                let icon_gap = if item.icon.is_some() && !item.label.is_empty() {
                    6.0
                } else {
                    0.0
                };
                let total_cw = icon_w + icon_gap + label_galley.size().x;
                let cx0 = paint_r.center().x - total_cw * 0.5;
                if let Some(icon) = item.icon {
                    icon.paint(
                        ui,
                        Rect::from_center_size(
                            pos2(cx0 + icon_w * 0.5, paint_r.center().y),
                            Vec2::splat(icon_w),
                        ),
                        fg,
                    );
                }
                ui.painter().galley(
                    pos2(
                        cx0 + icon_w + icon_gap,
                        paint_r.center().y - label_galley.size().y * 0.5,
                    ),
                    label_galley,
                    fg,
                );
            }
        }

        clicked
    }
}
