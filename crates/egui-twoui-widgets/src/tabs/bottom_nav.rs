use egui::{pos2, vec2, CornerRadius, Frame, Id, Margin, Popup, Rect, Response, Sense, Stroke, Ui};
use egui_twoui_icons::TwoUiIcon;
use egui_twoui_tokens::{
    animate_toggle, concentric_radius, ease_fast_out_slow_in, font_id, interaction_factors,
    paint_floating_capsule, paint_pill_press_ripple, paint_state_overlay, recoil_rect,
    show_frosted_frame, theme_from_ui, tokens::dimens, ElevationTier, TwoUiFontWeight,
    TwoUiPalette,
};

use super::{floating_capsule_colors, paint_nav_slot_contents};
use crate::controls::paint_badge;

/// Destination entry shared by [`super::MainTabBar`] and [`BottomNavBar`].
#[derive(Debug, Clone, Copy)]
pub struct BottomNavItem<'a> {
    pub label: &'a str,
    pub icon: Option<TwoUiIcon>,
    pub badge: Option<&'a str>,
}

impl<'a> BottomNavItem<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            icon: None,
            badge: None,
        }
    }

    pub fn icon(mut self, icon: TwoUiIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn badge(mut self, badge: &'a str) -> Self {
        self.badge = Some(badge);
        self
    }
}

/// Overflow menu item rendered in [`BottomNavBar`]'s `"More"` popup (`showAsAction="never"`).
#[derive(Debug, Clone, Copy)]
pub struct BottomNavOverflowItem<'a> {
    pub label: &'a str,
    pub badge: Option<&'a str>,
    pub divider_before: bool,
}

impl<'a> BottomNavOverflowItem<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            badge: None,
            divider_before: false,
        }
    }

    pub fn badge(mut self, badge: &'a str) -> Self {
        self.badge = Some(badge);
        self
    }

    /// Inserts a `1dp` group divider directly above this item.
    pub fn divider_before(mut self, enabled: bool) -> Self {
        self.divider_before = enabled;
        self
    }
}

/// Floating bottom navigation capsule (`430dp` max-width, Icon+Text or Icon-Only mode, `"More"` popup).
pub struct BottomNavBar<'a> {
    id_salt: &'a str,
    items: &'a [BottomNavItem<'a>],
    overflow: &'a [BottomNavOverflowItem<'a>],
    selected: &'a mut usize,
    icon_only: bool,
}

impl<'a> BottomNavBar<'a> {
    pub fn new(
        id_salt: &'a str,
        items: &'a [BottomNavItem<'a>],
        selected: &'a mut usize,
    ) -> Self {
        Self {
            id_salt,
            items,
            overflow: &[],
            selected,
            icon_only: false,
        }
    }

    /// Adds overflow items collapsed behind a trailing `"More"` button (`showAsAction="never"`).
    pub fn overflow(mut self, items: &'a [BottomNavOverflowItem<'a>]) -> Self {
        self.overflow = items;
        self
    }

    /// Uses compact icon-only slots (`Widget.Design.BottomNavigationView.Floating.Icon`, `54dp` height).
    pub fn icon_only(mut self, enabled: bool) -> Self {
        self.icon_only = enabled;
        self
    }

    /// Renders the floating navigation bar and returns `(response, clicked_overflow_index)`.
    pub fn show_with_overflow(self, ui: &mut Ui) -> (Response, Option<usize>) {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let dark = theme.mode.is_dark();
        let count = self.items.len().max(1);
        *self.selected = (*self.selected).min(count - 1);

        let has_overflow = !self.overflow.is_empty();
        let slot_count = count + if has_overflow { 1 } else { 0 };
        let is_icon_only = self.icon_only && self.items.iter().any(|it| it.icon.is_some());
        let pad = dimens::FLOATING_NAV_INNER_PADDING;
        let height = if is_icon_only { 54.0 } else { 64.0 };

        let avail_w = ui.available_width().max(180.0);
        let slot_min: f32 = if is_icon_only {
            match slot_count {
                1 => 104.0,
                2 => 68.0,
                _ => 60.0,
            }
        } else {
            match slot_count {
                1 => 120.0,
                2 => 96.0,
                3 => 80.0,
                _ => 70.0,
            }
        };
        let mut measured_slot_w: f32 = slot_min;
        if !is_icon_only {
            let lbl_font = font_id(ui, 12.0, TwoUiFontWeight::SemiBold);
            for item in self.items {
                let lw = ui
                    .painter()
                    .layout_no_wrap(item.label.to_owned(), lbl_font.clone(), p.text_secondary)
                    .size()
                    .x;
                measured_slot_w = measured_slot_w.max(lw + 26.0);
            }
            if has_overflow {
                let mw = ui
                    .painter()
                    .layout_no_wrap("More".to_owned(), lbl_font, p.text_secondary)
                    .size()
                    .x;
                measured_slot_w = measured_slot_w.max(mw + 26.0);
            }
            measured_slot_w = measured_slot_w.min(104.0);
        }
        let max_bar_w = avail_w.min(dimens::FLOATING_NAV_MAX_WIDTH);
        let bar_w = (slot_count as f32 * measured_slot_w + pad * 2.0)
            .min(max_bar_w)
            .max(slot_min + pad * 2.0);
        let (outer_rect, mut response) =
            ui.allocate_exact_size(vec2(avail_w, height), Sense::hover());
        let rect = Rect::from_center_size(outer_rect.center(), vec2(bar_w, height));
        let outer_r = CornerRadius::same(dimens::CAPSULE_CORNER_RADIUS);
        let inner_r = concentric_radius(dimens::CAPSULE_CORNER_RADIUS, pad);

        if ui.is_rect_visible(rect) {
            paint_floating_capsule(ui, rect, outer_r, dark, p);
        }

        let inner = rect.shrink(pad);
        let item_w = inner.width() / slot_count as f32;
        let (sel_pill_fill, active_fg, idle_fg) = floating_capsule_colors(p, dark);

        for (i, item) in self.items.iter().enumerate() {
            let slot_r = Rect::from_min_size(
                pos2(inner.left() + i as f32 * item_w, inner.top()),
                vec2(item_w, inner.height()),
            );
            let r = ui.interact(
                slot_r,
                Id::unique((self.id_salt, "twoui_bnav", i)),
                Sense::click(),
            );
            let r = if is_icon_only {
                r.on_hover_text(item.label)
            } else {
                r
            };
            if r.clicked() && *self.selected != i {
                *self.selected = i;
                response.mark_changed();
            }

            if ui.is_rect_visible(slot_r) {
                let is_sel = *self.selected == i;
                let sel_t = animate_toggle(
                    ui,
                    Id::unique((self.id_salt, "twoui_bnav_sel", i)),
                    is_sel,
                    0.22,
                    ease_fast_out_slow_in,
                );
                if sel_t > 0.001 {
                    let ind_rect = Rect::from_center_size(
                        slot_r.center(),
                        vec2(slot_r.width() * (0.86 + 0.14 * sel_t), slot_r.height()),
                    );
                    ui.painter().rect_filled(
                        ind_rect,
                        inner_r,
                        sel_pill_fill.gamma_multiply(sel_t),
                    );
                }
                let (pt, ht) = interaction_factors(ui, &r);
                let pr = recoil_rect(slot_r, pt, 0.6);
                let ripple_t = pt * (1.0 - sel_t * 0.45);
                if ripple_t > 0.001 {
                    paint_pill_press_ripple(ui, slot_r, inner_r, p, ripple_t);
                }

                let fg = idle_fg
                    .lerp_to_gamma(p.text_primary, ht * 0.45 * (1.0 - sel_t))
                    .lerp_to_gamma(active_fg, sel_t);
                paint_nav_slot_contents(
                    ui,
                    pr,
                    item.icon,
                    if is_icon_only { None } else { Some(item.label) },
                    item.badge,
                    fg,
                    is_sel,
                );
            }
        }

        let mut clicked_overflow = None;
        if has_overflow {
            let more_rect = Rect::from_min_size(
                pos2(inner.left() + count as f32 * item_w, inner.top()),
                vec2(item_w, inner.height()),
            );
            let more_resp = ui.interact(
                more_rect,
                Id::unique((self.id_salt, "twoui_bnav_more")),
                Sense::click(),
            );
            let open_id = Id::unique((self.id_salt, "twoui_bnav_more_open"));
            let popup_open = ui.ctx().data(|d| d.get_temp::<bool>(open_id)).unwrap_or(false);
            ui.ctx().data_mut(|d| d.insert_temp(open_id, false));

            if ui.is_rect_visible(more_rect) {
                let (pt, ht) = interaction_factors(ui, &more_resp);
                let pr = recoil_rect(more_rect, pt, 0.6);
                if pt > 0.001 {
                    paint_pill_press_ripple(ui, more_rect, inner_r, p, pt);
                }
                let fg = if popup_open {
                    active_fg
                } else {
                    idle_fg.lerp_to_gamma(p.text_primary, ht * 0.45)
                };
                paint_nav_slot_contents(
                    ui,
                    pr,
                    Some(TwoUiIcon::MoreVert),
                    if is_icon_only { None } else { Some("More") },
                    None,
                    fg,
                    popup_open,
                );
            }

            clicked_overflow = show_overflow_popup_menu(&more_resp, open_id, self.overflow, dark, p);
        }

        (response, clicked_overflow)
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        self.show_with_overflow(ui).0
    }
}

fn show_overflow_popup_menu(
    more_resp: &Response,
    open_id: Id,
    overflow: &[BottomNavOverflowItem<'_>],
    dark: bool,
    p: &TwoUiPalette,
) -> Option<usize> {
    let mut clicked = None;

    Popup::menu(more_resp)
        .frame(Frame::NONE)
        .width(176.0)
        .show(|ui| {
            show_frosted_frame(
                ui,
                CornerRadius::same(dimens::DIALOG_CORNER_RADIUS),
                p.floating_surface,
                Margin::symmetric(8, 8),
                ElevationTier::Md,
                dark,
                |ui| {
                    ui.ctx().data_mut(|d| d.insert_temp(open_id, true));
                    ui.spacing_mut().item_spacing.y = 2.0;
                    for (idx, ov) in overflow.iter().enumerate() {
                        if ov.divider_before {
                            let w = ui.available_width().max(120.0);
                            let (dr, _) = ui.allocate_exact_size(vec2(w, 8.0), Sense::hover());
                            ui.painter().line_segment(
                                [
                                    pos2(dr.left() + 8.0, dr.center().y),
                                    pos2(dr.right() - 8.0, dr.center().y),
                                ],
                                Stroke::new(1.0, p.divider),
                            );
                        }
                        let w = ui.available_width().max(120.0);
                        let (row_r, row_resp) = ui.allocate_exact_size(vec2(w, 40.0), Sense::click());
                        if ui.is_rect_visible(row_r) {
                            let (pt, ht) = interaction_factors(ui, &row_resp);
                            paint_state_overlay(
                                ui,
                                recoil_rect(row_r, pt, 0.8),
                                CornerRadius::same(16),
                                p,
                                ht,
                                pt,
                            );
                            let g = ui.painter().layout_no_wrap(
                                ov.label.to_owned(),
                                font_id(ui, 15.0, TwoUiFontWeight::Regular),
                                p.text_primary,
                            );
                            ui.painter().galley(
                                pos2(row_r.left() + 14.0, row_r.center().y - g.size().y * 0.5),
                                g,
                                p.text_primary,
                            );
                            if let Some(b) = ov.badge {
                                paint_badge(ui, pos2(row_r.right() - 18.0, row_r.center().y), b);
                            }
                        }
                        if row_resp.clicked() {
                            clicked = Some(idx);
                            ui.close();
                        }
                    }
                },
            );
        });
    clicked
}
