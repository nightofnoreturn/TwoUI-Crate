//! Sticky AppBar and floating toolbar pill row for [`super::TwoUiScaffold`].

use egui::{
    pos2, vec2, Align, Area, Color32, CornerRadius, Id, Layout, Order, Rect, Sense, Ui, UiBuilder,
};
use egui_twoui_icons::TwoUiIcon;
use egui_twoui_tokens::{
    dimens, font_id, paint_frosted_surface, ElevationTier, TwoUiFontWeight, TwoUiPalette,
};
use egui_twoui_widgets::button::TwoUiIconButton;

use super::{window_controls::paint_window_controls, ScaffoldNavButton};

#[allow(clippy::too_many_arguments)]
pub(crate) fn render_sticky_toolbar(
    ui: &Ui,
    id_salt: &str,
    title: &str,
    nav_button: ScaffoldNavButton,
    show_window_controls: bool,
    header_rect: Rect,
    top_inset: f32,
    row_h: f32,
    projection_t: f32,
    header_bar_vis: f32,
    compact_title_alpha: f32,
    p: &TwoUiPalette,
    dark: bool,
    toolbar_actions: impl FnOnce(&mut Ui),
) -> bool {
    let mut nav_clicked = false;
    let nav_icon = match nav_button {
        ScaffoldNavButton::None => None,
        ScaffoldNavButton::Back => Some((TwoUiIcon::Back, "Back")),
        ScaffoldNavButton::DrawerMenu => Some((TwoUiIcon::Menu, "Toggle Navigation")),
    };

    let cluster_w_id = Id::unique((id_salt, "twoui_trailing_w"));
    let prev_cluster_w: f32 = ui
        .ctx()
        .data(|d| d.get_temp::<f32>(cluster_w_id))
        .unwrap_or(if show_window_controls { 186.0 } else { 90.0 });

    Area::new(Id::unique((id_salt, "twoui_header_overlay")))
        .fixed_pos(header_rect.min)
        .movable(false)
        .constrain(false)
        .order(Order::Middle)
        .show(ui.ctx(), |ui| {
            ui.set_clip_rect(header_rect);
            ui.expand_to_include_rect(header_rect);

            if header_bar_vis > 0.01 {
                ui.painter().rect_filled(
                    header_rect,
                    CornerRadius::same(0),
                    p.round_and_bg.gamma_multiply(header_bar_vis),
                );
            }

            let row_rect = Rect::from_min_size(
                pos2(header_rect.left(), header_rect.top() + top_inset),
                vec2(header_rect.width(), row_h),
            );
            let mut left_cursor_x = row_rect.left() + 14.0;

            if let Some((icon, tip)) = nav_icon {
                let nav_outer =
                    Rect::from_min_size(pos2(left_cursor_x, row_rect.top()), vec2(44.0, row_h));
                if projection_t > 0.01 {
                    paint_frosted_surface(
                        ui,
                        nav_outer,
                        CornerRadius::same(dimens::CAPSULE_CORNER_RADIUS),
                        p.floating_projection.gamma_multiply(projection_t),
                        Color32::TRANSPARENT,
                        ElevationTier::Sm,
                        dark,
                    );
                }
                let mut btn_ui = ui.new_child(
                    UiBuilder::new()
                        .max_rect(nav_outer)
                        .layout(Layout::left_to_right(Align::Center)),
                );
                if btn_ui
                    .add(
                        TwoUiIconButton::new(icon)
                            .size(44.0)
                            .icon_size(20.0)
                            .tooltip(tip),
                    )
                    .clicked()
                {
                    nav_clicked = true;
                }
                left_cursor_x = nav_outer.right() + 10.0;
            }

            let right_edge = row_rect.right() - 12.0;
            let max_cluster_w = (right_edge - left_cursor_x - 8.0).max(80.0);
            let cluster_w = prev_cluster_w.clamp(48.0, max_cluster_w);
            let cluster_rect = Rect::from_min_size(
                pos2(right_edge - cluster_w, row_rect.top()),
                vec2(cluster_w, row_h),
            );

            if projection_t > 0.01 {
                paint_frosted_surface(
                    ui,
                    cluster_rect,
                    CornerRadius::same(dimens::CAPSULE_CORNER_RADIUS),
                    p.floating_projection.gamma_multiply(projection_t),
                    Color32::TRANSPARENT,
                    ElevationTier::Sm,
                    dark,
                );
            }

            let inner_pad_x = 4.0;
            let inner_rect = Rect::from_min_max(
                pos2(cluster_rect.left() + inner_pad_x, cluster_rect.top() + 2.0),
                pos2(
                    (header_rect.right() - 12.0 - inner_pad_x)
                        .max(cluster_rect.left() + inner_pad_x + 1.0),
                    cluster_rect.bottom() - 2.0,
                ),
            );
            let mut cluster_ui = ui.new_child(
                UiBuilder::new()
                    .max_rect(inner_rect)
                    .layout(Layout::left_to_right(Align::Center)),
            );
            cluster_ui.spacing_mut().item_spacing.x = 2.0;
            toolbar_actions(&mut cluster_ui);
            if show_window_controls {
                paint_window_controls(&mut cluster_ui, p);
            }
            let measured_w = cluster_ui.min_rect().width() + inner_pad_x * 2.0;
            if (measured_w - prev_cluster_w).abs() > 0.5 {
                ui.ctx()
                    .data_mut(|d| d.insert_temp(cluster_w_id, measured_w));
                ui.ctx().request_repaint();
            }

            let title_reserve = if compact_title_alpha > 0.02 {
                let avail_title_w = (cluster_rect.left() - 12.0 - left_cursor_x).max(0.0);
                if avail_title_w > 24.0 {
                    let title_col = p.text_primary.gamma_multiply(compact_title_alpha);
                    let tg = ui.painter().layout_no_wrap(
                        title.to_owned(),
                        font_id(ui, dimens::TEXT_TOOLBAR_TITLE, TwoUiFontWeight::SemiBold),
                        title_col,
                    );
                    let draw_w = tg.size().x.min(avail_title_w);
                    let tr = Rect::from_min_size(
                        pos2(left_cursor_x, row_rect.top()),
                        vec2(draw_w, row_h),
                    );
                    ui.painter().with_clip_rect(tr).galley(
                        pos2(tr.left(), tr.center().y - tg.size().y * 0.5),
                        tg,
                        title_col,
                    );
                    draw_w + 10.0
                } else {
                    0.0
                }
            } else {
                0.0
            };

            if show_window_controls {
                let drag_left = left_cursor_x + title_reserve + 6.0;
                let drag_right = cluster_rect.left() - 6.0;
                if drag_right > drag_left + 12.0 {
                    let drag_rect = Rect::from_min_max(
                        pos2(drag_left, row_rect.top().max(header_rect.top() + 6.0)),
                        pos2(drag_right, row_rect.bottom()),
                    );
                    let drag_resp = ui.interact(
                        drag_rect,
                        Id::unique((id_salt, "win_drag")),
                        Sense::click_and_drag(),
                    );
                    if drag_resp.drag_started() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
                    }
                    if drag_resp.double_clicked() {
                        let is_max = ui
                            .ctx()
                            .input(|i| i.viewport().maximized.unwrap_or(false));
                        ui.ctx()
                            .send_viewport_cmd(egui::ViewportCommand::Maximized(!is_max));
                    }
                }
            }
        });

    nav_clicked
}
