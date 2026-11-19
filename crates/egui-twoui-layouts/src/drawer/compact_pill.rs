//! Compact floating vertical capsule pill mode for [`super::TwoUiDrawer`].

use egui::{
    pos2, scroll_area::ScrollBarVisibility, vec2, Align, Area, Color32, CornerRadius, Id, Layout,
    Order, Pos2, Rect, ScrollArea, Sense, Ui, UiBuilder, Vec2,
};
use egui_twoui_icons::TwoUiIcon;
use egui_twoui_tokens::{
    dimens, interaction_factors, paint_frosted_surface, paint_pill_press_ripple, ElevationTier,
    TwoUiTheme,
};

use super::{paint_drawer_dots, DrawerEntry};

enum CompactSlot<'a> {
    Divider,
    Item {
        id: usize,
        label: &'a str,
        icon: TwoUiIcon,
        has_badge: bool,
        is_header_action: bool,
    },
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn paint_compact_pill(
    ui: &Ui,
    theme: &TwoUiTheme,
    id_salt: &str,
    entries: &[DrawerEntry<'_>],
    selected_id: &mut usize,
    pill_origin: Pos2,
    max_bottom: f32,
    compact_t: f32,
    header_action: Option<(TwoUiIcon, &'static str)>,
) -> (bool, bool) {
    let mut slots: Vec<CompactSlot<'_>> = Vec::with_capacity(entries.len() + 2);
    for entry in entries {
        match entry {
            DrawerEntry::DottedDivider => {
                if !slots.is_empty() && !matches!(slots.last(), Some(CompactSlot::Divider)) {
                    slots.push(CompactSlot::Divider);
                }
            }
            DrawerEntry::SectionLabel(_) => {}
            DrawerEntry::Item {
                id,
                label,
                icon,
                badge_count,
            } => slots.push(CompactSlot::Item {
                id: *id,
                label,
                icon: *icon,
                has_badge: badge_count.is_some(),
                is_header_action: false,
            }),
        }
    }
    while matches!(slots.last(), Some(CompactSlot::Divider)) {
        slots.pop();
    }
    if let Some((hdr_icon, hdr_tip)) = header_action {
        let already_present = slots
            .iter()
            .any(|s| matches!(s, CompactSlot::Item { icon, .. } if *icon == hdr_icon));
        if !already_present {
            if !slots.is_empty() {
                slots.push(CompactSlot::Divider);
            }
            slots.push(CompactSlot::Item {
                id: usize::MAX,
                label: hdr_tip,
                icon: hdr_icon,
                has_badge: false,
                is_header_action: true,
            });
        }
    }
    if slots.is_empty() {
        return (false, false);
    }

    let pill_w: f32 = 44.0;
    let pad: f32 = 4.0;
    let slot_size: f32 = pill_w - pad * 2.0;
    let item_gap: f32 = 2.0;
    let div_h: f32 = 10.0;

    let mut inner_h: f32 = 0.0;
    for (i, slot) in slots.iter().enumerate() {
        if i > 0 {
            inner_h += item_gap;
        }
        inner_h += match slot {
            CompactSlot::Divider => div_h,
            CompactSlot::Item { .. } => slot_size,
        };
    }
    let natural_h = pad * 2.0 + inner_h;
    let max_h = (max_bottom - pill_origin.y - 16.0).max(pill_w);
    let target_h = natural_h.min(max_h);
    let collapsed_h = pill_w.min(target_h);
    let anim_h = egui::lerp(collapsed_h..=target_h, compact_t);
    let pill_rect = Rect::from_min_size(pill_origin, vec2(pill_w, anim_h));
    let needs_scroll = natural_h > max_h + 0.5;

    let p = &theme.palette;
    let dark = theme.mode.is_dark();
    let proj_t = ui
        .ctx()
        .data(|d| d.get_temp::<f32>(crate::toolbar_projection_state_id()))
        .unwrap_or(0.0)
        .clamp(0.0, 1.0);
    let pill_vis = compact_t * proj_t;
    let mut selection_changed = false;
    let mut action_clicked = false;
    let scroll_salt = (id_salt, "twoui_compact_pill_scroll");

    Area::new(Id::unique((id_salt, "twoui_compact_pill")))
        .fixed_pos(pill_origin)
        .movable(false)
        .constrain(false)
        .order(Order::Middle)
        .show(ui.ctx(), |ui| {
            let _ = ui.allocate_exact_size(pill_rect.size(), Sense::hover());

            if pill_vis > 0.01 {
                let tier = if pill_vis > 0.35 {
                    ElevationTier::Sm
                } else {
                    ElevationTier::Zero
                };
                paint_frosted_surface(
                    ui,
                    pill_rect,
                    CornerRadius::same(dimens::CAPSULE_CORNER_RADIUS),
                    p.floating_projection.gamma_multiply(pill_vis),
                    Color32::TRANSPARENT,
                    tier,
                    dark,
                );
            }

            let inner_rect = pill_rect.shrink(pad);
            if inner_rect.height() <= 4.0 || inner_rect.width() <= 4.0 {
                return;
            }
            let inner_clip = inner_rect.intersect(ui.ctx().content_rect());
            let mut inner_ui = ui.new_child(
                UiBuilder::new()
                    .max_rect(inner_rect)
                    .layout(Layout::top_down(Align::Center)),
            );
            inner_ui.set_clip_rect(inner_clip);
            inner_ui.spacing_mut().item_spacing.y = item_gap;

            let (sel, act) = if needs_scroll {
                ScrollArea::vertical()
                    .id_salt(scroll_salt)
                    .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                    .auto_shrink([false, false])
                    .show(&mut inner_ui, |ui| {
                        ui.spacing_mut().item_spacing.y = item_gap;
                        ui.set_clip_rect(inner_clip);
                        render_compact_slots(
                            ui,
                            &slots,
                            selected_id,
                            theme,
                            compact_t,
                            slot_size,
                            div_h,
                        )
                    })
                    .inner
            } else {
                render_compact_slots(
                    &mut inner_ui,
                    &slots,
                    selected_id,
                    theme,
                    compact_t,
                    slot_size,
                    div_h,
                )
            };
            selection_changed |= sel;
            action_clicked |= act;
        });

    (selection_changed, action_clicked)
}

fn render_compact_slots(
    ui: &mut Ui,
    slots: &[CompactSlot<'_>],
    selected_id: &mut usize,
    theme: &TwoUiTheme,
    compact_t: f32,
    slot_size: f32,
    div_h: f32,
) -> (bool, bool) {
    let p = &theme.palette;
    let dark = theme.mode.is_dark();
    let slot_radius = CornerRadius::same(dimens::CAPSULE_CORNER_RADIUS);
    let mut selection_changed = false;
    let mut action_clicked = false;

    for slot in slots {
        match slot {
            CompactSlot::Divider => {
                let (div_rect, _) = ui.allocate_exact_size(vec2(slot_size, div_h), Sense::hover());
                if ui.is_rect_visible(div_rect) {
                    paint_drawer_dots(ui, div_rect, dark, 8.0, compact_t);
                }
            }
            CompactSlot::Item {
                id,
                label,
                icon,
                has_badge,
                is_header_action,
            } => {
                let is_sel = !*is_header_action && *selected_id == *id;
                let (slot_rect, resp) =
                    ui.allocate_exact_size(Vec2::splat(slot_size), Sense::click());
                let resp = resp.on_hover_text(*label);

                if compact_t > 0.25 && resp.clicked() {
                    if *is_header_action {
                        action_clicked = true;
                    } else if *selected_id != *id {
                        *selected_id = *id;
                        selection_changed = true;
                    }
                }

                if ui.is_rect_visible(slot_rect) {
                    let (press_t, hover_t) = interaction_factors(ui, &resp);

                    if is_sel {
                        ui.painter().rect_filled(
                            slot_rect,
                            slot_radius,
                            p.selected_bg.gamma_multiply(compact_t),
                        );
                    }
                    if hover_t > 0.001 && !is_sel {
                        ui.painter().rect_filled(
                            slot_rect,
                            slot_radius,
                            p.hover_overlay.gamma_multiply(hover_t * compact_t),
                        );
                    }
                    paint_pill_press_ripple(ui, slot_rect, slot_radius, p, press_t * compact_t);

                    let icon_col = (if is_sel {
                        p.primary_high
                    } else {
                        p.text_muted
                    })
                    .gamma_multiply(compact_t);
                    let icon_rect = Rect::from_center_size(slot_rect.center(), Vec2::splat(20.0));
                    icon.paint(ui, icon_rect, icon_col);

                    if *has_badge {
                        ui.painter().circle_filled(
                            pos2(icon_rect.right() + 1.5, icon_rect.top() - 1.5),
                            2.8,
                            p.badge_bg.gamma_multiply(compact_t),
                        );
                    }
                }
            }
        }
    }

    (selection_changed, action_clicked)
}
