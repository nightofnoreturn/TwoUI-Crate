use egui::{
    pos2, vec2, Align, CornerRadius, Frame, Id, Layout, Margin, Popup, Rect, Response, Role,
    Sense, Ui, Vec2, WidgetInfo,
};
use egui_twoui_icons::TwoUiIcon;
use egui_twoui_tokens::{
    animate_toggle, concentric_radius, ease_release, font_id, interaction_factors,
    paint_floating_capsule, paint_pill_press_ripple, paint_state_overlay, recoil_rect,
    show_frosted_frame, theme_from_ui, tokens::dimens, ElevationTier, TwoUiFontWeight,
};

use super::{floating_capsule_colors, paint_nav_slot_contents, BottomNavItem};

/// Centered floating tab capsule (`430dp` max-width, optional icon+text or text-only tabs,
/// and automatic `"More"` 4-column grid popup when >4 items).
pub struct MainTabBar<'a> {
    id_salt: &'a str,
    items: &'a [BottomNavItem<'a>],
    selected: &'a mut usize,
}

impl<'a> MainTabBar<'a> {
    pub fn new(
        id_salt: &'a str,
        items: &'a [BottomNavItem<'a>],
        selected: &'a mut usize,
    ) -> Self {
        Self {
            id_salt,
            items,
            selected,
        }
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let dark = theme.mode.is_dark();
        let total = self.items.len().max(1);
        *self.selected = (*self.selected).min(total - 1);

        let has_overflow = self.items.len() > 4;
        let visible_count = if has_overflow {
            3
        } else {
            self.items.len().max(1)
        };
        let slot_count = visible_count + if has_overflow { 1 } else { 0 };
        let pad = dimens::FLOATING_NAV_INNER_PADDING;
        let has_icons = self.items.iter().any(|it| it.icon.is_some()) || has_overflow;
        let height = if has_icons { 64.0 } else { 48.0 };

        let avail_w = ui.available_width().max(220.0);
        let min_slot: f32 = if has_icons {
            match slot_count {
                1 => 120.0,
                2 => 96.0,
                3 => 80.0,
                _ => 70.0,
            }
        } else {
            76.0
        };
        let mut measured_slot_w: f32 = min_slot;
        let lbl_font = font_id(
            ui,
            if has_icons { 12.0 } else { 14.5 },
            TwoUiFontWeight::SemiBold,
        );
        let h_pad = if has_icons { 26.0 } else { 28.0 };
        for item in self.items.iter().take(visible_count) {
            let lw = ui
                .painter()
                .layout_no_wrap(item.label.to_owned(), lbl_font.clone(), p.text_secondary)
                .size()
                .x;
            measured_slot_w = measured_slot_w.max(lw + h_pad);
        }
        if has_overflow {
            let mw = ui
                .painter()
                .layout_no_wrap("More".to_owned(), lbl_font, p.text_secondary)
                .size()
                .x;
            measured_slot_w = measured_slot_w.max(mw + h_pad);
        }
        measured_slot_w = measured_slot_w.min(if has_icons { 104.0 } else { 124.0 });
        let max_bar_w = avail_w.min(dimens::FLOATING_NAV_MAX_WIDTH);
        let bar_w = (slot_count as f32 * measured_slot_w + pad * 2.0)
            .min(max_bar_w)
            .max(min_slot + pad * 2.0);
        let (outer_rect, mut response) =
            ui.allocate_exact_size(vec2(avail_w, height), Sense::hover());
        let rect = Rect::from_center_size(outer_rect.center(), vec2(bar_w, height));
        let outer_r = CornerRadius::same(dimens::CAPSULE_CORNER_RADIUS);
        let inner_r = concentric_radius(dimens::CAPSULE_CORNER_RADIUS, pad);

        if ui.is_rect_visible(rect) {
            paint_floating_capsule(ui, rect, outer_r, dark, p);
        }

        let inner = rect.shrink(pad);
        let slot_w = inner.width() / slot_count as f32;
        let (sel_pill_fill, active_fg, idle_fg) = floating_capsule_colors(p, dark);

        for i in 0..visible_count.min(self.items.len()) {
            let item = &self.items[i];
            let slot_rect = Rect::from_min_size(
                pos2(inner.left() + i as f32 * slot_w, inner.top()),
                vec2(slot_w, inner.height()),
            );
            let r = ui.interact(
                slot_rect,
                Id::unique((self.id_salt, "twoui_mtab", i)),
                Sense::click(),
            );
            let is_sel = *self.selected == i;
            r.widget_info(|| WidgetInfo::selected(Role::Tab, true, is_sel, item.label));

            if r.clicked() && *self.selected != i {
                *self.selected = i;
                response.mark_changed();
            }

            if ui.is_rect_visible(slot_rect) {
                let (press_t, hover_t) = interaction_factors(ui, &r);
                let sel_t = animate_toggle(
                    ui,
                    Id::unique((self.id_salt, "twoui_mtab_sel", i)),
                    is_sel,
                    0.15,
                    ease_release,
                );
                if sel_t > 0.001 {
                    ui.painter()
                        .rect_filled(slot_rect, inner_r, sel_pill_fill.gamma_multiply(sel_t));
                }
                let paint_r = recoil_rect(slot_rect, press_t, 0.6);
                let ripple_t = press_t * (1.0 - sel_t * 0.45);
                if ripple_t > 0.001 {
                    paint_pill_press_ripple(ui, slot_rect, inner_r, p, ripple_t);
                }

                let fg = idle_fg
                    .lerp_to_gamma(p.text_primary, hover_t * 0.45 * (1.0 - sel_t))
                    .lerp_to_gamma(active_fg, sel_t);
                paint_nav_slot_contents(
                    ui,
                    paint_r,
                    item.icon,
                    Some(item.label),
                    item.badge,
                    fg,
                    is_sel,
                );
            }
        }

        if has_overflow {
            let more_rect = Rect::from_min_size(
                pos2(inner.left() + visible_count as f32 * slot_w, inner.top()),
                vec2(slot_w, inner.height()),
            );
            let more_resp = ui.interact(
                more_rect,
                Id::unique((self.id_salt, "twoui_mtab_more")),
                Sense::click(),
            );
            let open_id = Id::unique((self.id_salt, "twoui_mtab_more_open"));
            let popup_open = ui.ctx().data(|d| d.get_temp::<bool>(open_id)).unwrap_or(false);
            ui.ctx().data_mut(|d| d.insert_temp(open_id, false));
            let overflow_active = *self.selected >= visible_count;

            if ui.is_rect_visible(more_rect) {
                let (press_t, hover_t) = interaction_factors(ui, &more_resp);
                let sel_t = animate_toggle(
                    ui,
                    Id::unique((self.id_salt, "twoui_mtab_more_sel")),
                    overflow_active,
                    0.15,
                    ease_release,
                );
                if sel_t > 0.001 {
                    ui.painter()
                        .rect_filled(more_rect, inner_r, sel_pill_fill.gamma_multiply(sel_t));
                }
                let paint_r = recoil_rect(more_rect, press_t, 0.6);
                let ripple_t = press_t * (1.0 - sel_t * 0.45);
                if ripple_t > 0.001 {
                    paint_pill_press_ripple(ui, more_rect, inner_r, p, ripple_t);
                }
                let fg = idle_fg
                    .lerp_to_gamma(p.text_primary, hover_t * 0.45 * (1.0 - sel_t))
                    .lerp_to_gamma(active_fg, if popup_open { 1.0 } else { sel_t });
                paint_nav_slot_contents(
                    ui,
                    paint_r,
                    Some(TwoUiIcon::Menu),
                    Some("More"),
                    None,
                    fg,
                    overflow_active || popup_open,
                );
            }

            let cols = 4usize;
            let cell_w = 80.0;
            let cell_h = 68.0;
            let grid_w = cols as f32 * cell_w + (cols - 1) as f32 * 4.0 + 24.0;

            Popup::menu(&more_resp)
                .frame(Frame::NONE)
                .width(grid_w)
                .show(|ui| {
                    show_frosted_frame(
                        ui,
                        CornerRadius::same(dimens::DIALOG_CORNER_RADIUS),
                        p.floating_surface,
                        Margin::symmetric(12, 12),
                        ElevationTier::Lg,
                        dark,
                        |ui| {
                            ui.ctx().data_mut(|d| d.insert_temp(open_id, true));
                            let overflow_slice = &self.items[visible_count..];
                            ui.spacing_mut().item_spacing = vec2(4.0, 6.0);
                            for (row_idx, chunk) in overflow_slice.chunks(cols).enumerate() {
                                ui.allocate_ui_with_layout(
                                    vec2(grid_w - 24.0, cell_h),
                                    Layout::left_to_right(Align::Center),
                                    |ui| {
                                        ui.spacing_mut().item_spacing.x = 4.0;
                                        for (col_idx, ov_item) in chunk.iter().enumerate() {
                                            let actual_idx =
                                                visible_count + row_idx * cols + col_idx;
                                            let (cell_r, cell_resp) = ui.allocate_exact_size(
                                                vec2(cell_w, cell_h),
                                                Sense::click(),
                                            );
                                            if ui.is_rect_visible(cell_r) {
                                                let is_sel = *self.selected == actual_idx;
                                                let (pt, ht) = interaction_factors(ui, &cell_resp);
                                                let pr = recoil_rect(cell_r, pt, 1.0);
                                                if is_sel {
                                                    ui.painter().rect_filled(
                                                        pr,
                                                        CornerRadius::same(16),
                                                        p.selected_bg,
                                                    );
                                                }
                                                paint_state_overlay(
                                                    ui,
                                                    pr,
                                                    CornerRadius::same(16),
                                                    p,
                                                    ht,
                                                    pt,
                                                );

                                                let fg = if is_sel {
                                                    p.primary_high
                                                } else {
                                                    p.text_primary
                                                };
                                                if let Some(icon) = ov_item.icon {
                                                    icon.paint(
                                                        ui,
                                                        Rect::from_center_size(
                                                            pos2(pr.center().x, pr.top() + 21.0),
                                                            Vec2::splat(22.0),
                                                        ),
                                                        fg,
                                                    );
                                                }
                                                let g = ui.painter().layout_no_wrap(
                                                    ov_item.label.to_owned(),
                                                    font_id(
                                                        ui,
                                                        11.5,
                                                        if is_sel {
                                                            TwoUiFontWeight::SemiBold
                                                        } else {
                                                            TwoUiFontWeight::Medium
                                                        },
                                                    ),
                                                    fg,
                                                );
                                                ui.painter().galley(
                                                    pos2(
                                                        pr.center().x - g.size().x * 0.5,
                                                        pr.bottom() - 15.0 - g.size().y * 0.5,
                                                    ),
                                                    g,
                                                    fg,
                                                );
                                            }
                                            if cell_resp.clicked() {
                                                *self.selected = actual_idx;
                                                response.mark_changed();
                                                ui.close();
                                            }
                                        }
                                    },
                                );
                            }
                        },
                    );
                });
        }

        response
    }
}
