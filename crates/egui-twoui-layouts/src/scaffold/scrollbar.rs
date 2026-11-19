//! Proportional right-side scrollbar overlay for [`super::TwoUiScaffold`].

use egui::{pos2, vec2, Area, Color32, CornerRadius, Id, Order, Rect, Sense, Stroke, Ui};
use egui_twoui_tokens::{dimens, paint_frosted_surface, ElevationTier, TwoUiPalette};

#[allow(clippy::too_many_arguments)]
pub(crate) fn paint_twoui_fast_scrollbar(
    ui: &Ui,
    id_salt: &str,
    avail: Rect,
    header_h: f32,
    content_h: f32,
    view_h: f32,
    scroll_y: f32,
    max_scroll: f32,
    prev_scroll_y: f32,
    force_scroll_id: Id,
    scroll_mem_id: Id,
    p: &TwoUiPalette,
    dark: bool,
) {
    if max_scroll <= 4.0 || content_h <= view_h {
        return;
    }

    let now = ui.ctx().input(|i| i.time);
    let active_time_id = Id::unique((id_salt, "twoui_fs_last_scroll"));
    if (scroll_y - prev_scroll_y).abs() > 0.5 {
        ui.ctx().data_mut(|d| d.insert_temp(active_time_id, now));
    }
    let last_active = ui
        .ctx()
        .data(|d| d.get_temp::<f64>(active_time_id))
        .unwrap_or(-10.0);
    let recently_scrolled = (now - last_active) < 1.2;

    let top = avail.top() + header_h + 6.0;
    let bot = (avail.bottom() - 16.0).max(top + 80.0);
    let track_h = bot - top;
    // Leave rightmost 6px free for borderless window edge resize.
    let strip_rect = Rect::from_min_max(
        pos2((avail.right() - 28.0).max(avail.left()), top),
        pos2((avail.right() - 6.0).max(avail.left() + 8.0), bot),
    );

    Area::new(Id::unique((id_salt, "twoui_fastscroll")))
        .fixed_pos(strip_rect.min)
        .movable(false)
        .constrain(false)
        .order(Order::Middle)
        .show(ui.ctx(), |ui| {
            ui.set_clip_rect(strip_rect);
            let (_, resp) = ui.allocate_exact_size(strip_rect.size(), Sense::click_and_drag());
            let pressing = resp.dragged() || resp.is_pointer_button_down_on();
            let hot = resp.hovered() || pressing;

            let expand_t = ui.ctx().animate_bool_with_time(
                Id::unique((id_salt, "twoui_fs_expand")),
                hot,
                0.15,
            );
            let vis_t = ui.ctx().animate_bool_with_time(
                Id::unique((id_salt, "twoui_fs_vis")),
                hot || recently_scrolled,
                0.22,
            );
            let thumb_alpha = (0.32 + 0.68 * vis_t).clamp(0.32, 1.0);

            let min_thumb_h = 44.0_f32;
            let max_thumb_h = (track_h * 0.85).max(min_thumb_h);
            let thumb_h = ((view_h / content_h) * track_h).clamp(min_thumb_h, max_thumb_h);
            let usable_h = (track_h - thumb_h).max(1.0);

            let current_frac = (scroll_y / max_scroll).clamp(0.0, 1.0);
            let current_thumb_top = top + current_frac * usable_h;
            let current_thumb_bot = current_thumb_top + thumb_h;

            if resp.dragged() {
                let dy = resp.drag_delta().y;
                if dy.abs() > 0.01 {
                    let target_y = (scroll_y + (dy / usable_h) * max_scroll).clamp(0.0, max_scroll);
                    ui.ctx()
                        .data_mut(|d| d.insert_temp(force_scroll_id, Some(target_y)));
                    ui.ctx().data_mut(|d| d.insert_temp(scroll_mem_id, target_y));
                    ui.ctx().data_mut(|d| d.insert_temp(active_time_id, now));
                    ui.ctx().request_repaint();
                }
            } else if resp.clicked() {
                if let Some(pos) = resp.interact_pointer_pos() {
                    if pos.y < current_thumb_top || pos.y > current_thumb_bot {
                        let frac = ((pos.y - top - thumb_h * 0.5) / usable_h).clamp(0.0, 1.0);
                        let target_y = frac * max_scroll;
                        ui.ctx()
                            .data_mut(|d| d.insert_temp(force_scroll_id, Some(target_y)));
                        ui.ctx().data_mut(|d| d.insert_temp(scroll_mem_id, target_y));
                        ui.ctx().data_mut(|d| d.insert_temp(active_time_id, now));
                        ui.ctx().request_repaint();
                    }
                }
            }

            let frac = (scroll_y / max_scroll).clamp(0.0, 1.0);
            let thumb_y = top + frac * usable_h;
            let thumb_w = egui::lerp(4.0..=16.0_f32, expand_t);
            let right_x = strip_rect.right();
            let thumb_rect = Rect::from_min_size(
                pos2(right_x - thumb_w, thumb_y),
                vec2(thumb_w, thumb_h),
            );

            if expand_t > 0.12 {
                let pill_fill = if pressing {
                    p.primary.gamma_multiply(0.95)
                } else {
                    p.floating_surface.gamma_multiply(thumb_alpha.max(0.90))
                };
                paint_frosted_surface(
                    ui,
                    thumb_rect,
                    CornerRadius::same(dimens::CAPSULE_CORNER_RADIUS),
                    pill_fill,
                    Color32::TRANSPARENT,
                    ElevationTier::Sm,
                    dark,
                );
                let arrow_col = if pressing {
                    p.on_primary
                } else {
                    p.text_secondary
                }
                .gamma_multiply(expand_t);
                let cx = thumb_rect.center().x;
                let cy = thumb_rect.center().y;
                let s = Stroke::new(1.5, arrow_col);
                ui.painter().line_segment([pos2(cx - 3.2, cy - 4.0), pos2(cx, cy - 7.2)], s);
                ui.painter().line_segment([pos2(cx, cy - 7.2), pos2(cx + 3.2, cy - 4.0)], s);
                ui.painter().line_segment([pos2(cx - 3.2, cy + 4.0), pos2(cx, cy + 7.2)], s);
                ui.painter().line_segment([pos2(cx, cy + 7.2), pos2(cx + 3.2, cy + 4.0)], s);
            } else {
                let col = p.text_muted.gamma_multiply(thumb_alpha * 0.72);
                ui.painter().rect_filled(
                    thumb_rect,
                    CornerRadius::same(dimens::CAPSULE_CORNER_RADIUS),
                    col,
                );
            }
        });
}
