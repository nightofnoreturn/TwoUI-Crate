//! Borderless desktop CSD window controls (Minimize, Maximize/Restore, Close) and
//! 8-direction window edge/corner resize handling.

use egui::{
    pos2, vec2, Color32, Context, CornerRadius, CursorIcon, Id, Pos2, Rect, ResizeDirection,
    Sense, Stroke, StrokeKind, Ui, Vec2,
};
use egui_twoui_tokens::{
    dimens, interaction_factors, interactive_fill, paint_pill_press_ripple, recoil_rect,
    TwoUiPalette,
};

pub(crate) fn paint_window_controls(ui: &mut Ui, p: &TwoUiPalette) {
    let is_max = ui
        .ctx()
        .input(|i| i.viewport().maximized.unwrap_or(false));

    if paint_window_btn(ui, "Minimize", false, p, |painter, c, fg| {
        painter.line_segment(
            [pos2(c.x - 4.5, c.y + 0.5), pos2(c.x + 4.5, c.y + 0.5)],
            Stroke::new(1.5, fg),
        );
    }) {
        ui.ctx()
            .send_viewport_cmd(egui::ViewportCommand::Minimized(true));
    }

    let max_tip = if is_max { "Restore" } else { "Maximize" };
    if paint_window_btn(ui, max_tip, false, p, |painter, c, fg| {
        let s = Stroke::new(1.35, fg);
        if is_max {
            let back = Rect::from_center_size(c + vec2(1.5, -1.5), Vec2::splat(6.5));
            let front = Rect::from_center_size(c + vec2(-1.5, 1.5), Vec2::splat(6.5));
            painter.rect_stroke(back, CornerRadius::same(1), s, StrokeKind::Inside);
            painter.rect_filled(front, CornerRadius::same(1), p.surface);
            painter.rect_stroke(front, CornerRadius::same(1), s, StrokeKind::Inside);
        } else {
            let r = Rect::from_center_size(c, Vec2::splat(8.0));
            painter.rect_stroke(r, CornerRadius::same(1), s, StrokeKind::Inside);
        }
    }) {
        ui.ctx()
            .send_viewport_cmd(egui::ViewportCommand::Maximized(!is_max));
    }

    if paint_window_btn(ui, "Close", true, p, |painter, c, fg| {
        let d = 4.0;
        let s = Stroke::new(1.5, fg);
        painter.line_segment([pos2(c.x - d, c.y - d), pos2(c.x + d, c.y + d)], s);
        painter.line_segment([pos2(c.x + d, c.y - d), pos2(c.x - d, c.y + d)], s);
    }) {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
    }
}

fn paint_window_btn(
    ui: &mut Ui,
    tooltip: &str,
    is_close: bool,
    p: &TwoUiPalette,
    glyph: impl FnOnce(&egui::Painter, Pos2, Color32),
) -> bool {
    let (rect, resp) = ui.allocate_exact_size(Vec2::splat(36.0), Sense::click());
    if ui.is_rect_visible(rect) {
        let (pt, ht) = interaction_factors(ui, &resp);
        let paint_r = recoil_rect(rect, pt, 0.7);
        let radius = CornerRadius::same(dimens::CAPSULE_CORNER_RADIUS);
        let fill = if is_close && (ht > 0.01 || pt > 0.01) {
            p.functional_red
                .gamma_multiply((ht * 0.85 + pt * 0.15).clamp(0.0, 1.0))
        } else if ht > 0.001 {
            interactive_fill(Color32::TRANSPARENT, p, ht, 0.0)
        } else {
            Color32::TRANSPARENT
        };
        if fill != Color32::TRANSPARENT {
            ui.painter().rect_filled(rect, radius, fill);
        }
        if !is_close && pt > 0.001 {
            paint_pill_press_ripple(ui, rect, radius, p, pt);
        }
        let fg = if is_close && ht > 0.3 {
            Color32::WHITE
        } else {
            p.text_primary
        };
        glyph(ui.painter(), paint_r.center(), fg);
    }
    resp.on_hover_text(tooltip).clicked()
}

/// Applies interactive 8-direction edge/corner window resizing and a `1px` window border outline
/// for borderless desktop windows (`ViewportBuilder::with_decorations(false)`).
pub fn apply_borderless_window_resize(ctx: &Context, p: &TwoUiPalette) {
    let is_max = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
    let screen = ctx.content_rect();
    if is_max {
        return;
    }

    ctx.layer_painter(egui::LayerId::new(
        egui::Order::Foreground,
        Id::unique("twoui_borderless_frame"),
    ))
    .rect_stroke(
        screen.shrink(0.5),
        CornerRadius::same(0),
        Stroke::new(1.0, p.divider),
        StrokeKind::Inside,
    );

    let pointer_pos = ctx.input(|i| i.pointer.hover_pos());
    let primary_pressed = ctx.input(|i| i.pointer.primary_pressed());
    let Some(pos) = pointer_pos else {
        return;
    };

    let edge = 6.0;
    let corner = 12.0;
    let on_left = pos.x <= screen.left() + edge;
    let on_right = pos.x >= screen.right() - edge;
    let on_top = pos.y <= screen.top() + edge;
    let on_bottom = pos.y >= screen.bottom() - edge;

    let near_left = pos.x <= screen.left() + corner;
    let near_right = pos.x >= screen.right() - corner;
    let near_top = pos.y <= screen.top() + corner;
    let near_bottom = pos.y >= screen.bottom() - corner;

    let dir = match (on_left, on_right, on_top, on_bottom) {
        (true, _, true, _) => Some((ResizeDirection::NorthWest, CursorIcon::ResizeNwSe)),
        (_, true, true, _) => Some((ResizeDirection::NorthEast, CursorIcon::ResizeNeSw)),
        (true, _, _, true) => Some((ResizeDirection::SouthWest, CursorIcon::ResizeNeSw)),
        (_, true, _, true) => Some((ResizeDirection::SouthEast, CursorIcon::ResizeNwSe)),
        _ if (on_left || on_top) && near_left && near_top => {
            Some((ResizeDirection::NorthWest, CursorIcon::ResizeNwSe))
        }
        _ if (on_right || on_top) && near_right && near_top => {
            Some((ResizeDirection::NorthEast, CursorIcon::ResizeNeSw))
        }
        _ if (on_left || on_bottom) && near_left && near_bottom => {
            Some((ResizeDirection::SouthWest, CursorIcon::ResizeNeSw))
        }
        _ if (on_right || on_bottom) && near_right && near_bottom => {
            Some((ResizeDirection::SouthEast, CursorIcon::ResizeNwSe))
        }
        (true, false, false, false) => Some((ResizeDirection::West, CursorIcon::ResizeHorizontal)),
        (false, true, false, false) => Some((ResizeDirection::East, CursorIcon::ResizeHorizontal)),
        (false, false, true, false) => Some((ResizeDirection::North, CursorIcon::ResizeVertical)),
        (false, false, false, true) => Some((ResizeDirection::South, CursorIcon::ResizeVertical)),
        _ => None,
    };

    if let Some((resize_dir, cursor)) = dir {
        ctx.set_cursor_icon(cursor);
        if primary_pressed {
            ctx.send_viewport_cmd(egui::ViewportCommand::BeginResize(resize_dir));
        }
    }
}
