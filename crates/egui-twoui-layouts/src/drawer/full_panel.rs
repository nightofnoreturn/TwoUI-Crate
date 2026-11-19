//! Full expanded floating drawer card for [`super::TwoUiDrawer`].

use egui::{
    pos2, scroll_area::ScrollBarVisibility, vec2, Align, Color32, CornerRadius, FontFamily,
    FontId, Frame, Layout, Panel, Rect, ScrollArea, Sense, Ui, UiBuilder, Vec2,
};
use egui_twoui_icons::TwoUiIcon;
use egui_twoui_tokens::{
    concentric_radius, dimens, font_id, interaction_factors, paint_frosted_surface,
    paint_pill_press_ripple, ElevationTier, TwoUiFontWeight, TwoUiTheme,
};
use egui_twoui_widgets::button::TwoUiIconButton;

use super::{paint_drawer_dots, DrawerEntry};

#[allow(clippy::too_many_arguments)]
pub(crate) fn paint_full_drawer_panel(
    ui: &mut Ui,
    theme: &TwoUiTheme,
    id_salt: &str,
    header_title: &str,
    entries: &[DrawerEntry<'_>],
    selected_id: &mut usize,
    panel_w: f32,
    inset: f32,
    card_vis: f32,
    interpolated_alpha: f32,
    header_action: Option<(TwoUiIcon, &'static str)>,
) -> (bool, bool) {
    let p = &theme.palette;
    let dark = theme.mode.is_dark();
    let item_radius = concentric_radius(dimens::DRAWER_CORNER_RADIUS, 4.0);
    let panel_frame = Frame::NONE.fill(p.round_and_bg);
    let mut selection_changed = false;
    let mut action_clicked = false;

    Panel::left(id_salt)
        .exact_size(panel_w)
        .resizable(false)
        .show_separator_line(false)
        .frame(panel_frame)
        .show(ui, |ui| {
            let avail = ui.available_rect_before_wrap();
            ui.expand_to_include_rect(avail);

            let margin_scale = (panel_w / 24.0).clamp(0.0, 1.0);
            let left_gap = inset * margin_scale;
            let right_gap = 4.0 * margin_scale;
            let card_w = (avail.width() - left_gap - right_gap).max(0.0);
            let card_h = (avail.height() - inset * 2.0).max(24.0);
            let card_rect = Rect::from_min_size(
                pos2(avail.left() + left_gap, avail.top() + inset),
                vec2(card_w, card_h),
            );

            if card_w > 2.0 {
                let tier = if card_vis > 0.35 {
                    ElevationTier::Sm
                } else {
                    ElevationTier::Zero
                };
                paint_frosted_surface(
                    ui,
                    card_rect,
                    CornerRadius::same(dimens::DRAWER_CORNER_RADIUS),
                    p.drawer_surface.gamma_multiply(card_vis),
                    p.floating_stroke.gamma_multiply(card_vis),
                    tier,
                    dark,
                );
            }

            if card_w >= 56.0 && interpolated_alpha > 0.01 {
                let inner_rect = card_rect.shrink2(vec2(10.0, 12.0));
                let panel_clip = card_rect.shrink(4.0).intersect(ui.clip_rect());
                let mut inner_ui = ui.new_child(
                    UiBuilder::new()
                        .max_rect(inner_rect)
                        .layout(Layout::top_down(Align::LEFT)),
                );
                inner_ui.set_clip_rect(panel_clip);
                inner_ui.spacing_mut().item_spacing.y = 4.0;

                let hdr_w = inner_ui.available_width().max(1.0);
                let (hdr_rect, _) =
                    inner_ui.allocate_exact_size(vec2(hdr_w, 44.0), Sense::hover());
                if inner_ui.is_rect_visible(hdr_rect) {
                    let title_col = p.text_primary.gamma_multiply(interpolated_alpha);
                    let title_galley = inner_ui.painter().layout_no_wrap(
                        header_title.to_owned(),
                        font_id(&inner_ui, 19.0, TwoUiFontWeight::SemiBold),
                        title_col,
                    );
                    inner_ui.painter().with_clip_rect(panel_clip).galley(
                        pos2(
                            hdr_rect.left() + 10.0,
                            hdr_rect.center().y - title_galley.size().y * 0.5,
                        ),
                        title_galley,
                        title_col,
                    );
                }
                if let Some((icon, tooltip)) = header_action {
                    let cx_open = (hdr_rect.right() - 18.0).max(hdr_rect.left() + 18.0);
                    let btn_center = pos2(cx_open, hdr_rect.center().y);
                    let btn_rect = Rect::from_center_size(btn_center, Vec2::splat(36.0));
                    let mut child_ui = inner_ui.new_child(UiBuilder::new().max_rect(btn_rect));
                    child_ui.set_clip_rect(child_ui.clip_rect().intersect(panel_clip));
                    if child_ui
                        .add(
                            TwoUiIconButton::new(icon)
                                .size(36.0)
                                .icon_size(20.0)
                                .tint(p.text_muted.gamma_multiply(interpolated_alpha))
                                .tooltip(tooltip),
                        )
                        .clicked()
                        && interpolated_alpha > 0.25
                    {
                        action_clicked = true;
                    }
                }

                inner_ui.add_space(6.0);

                ScrollArea::vertical()
                    .id_salt((id_salt, "twoui_drawer_scroll"))
                    .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                    .auto_shrink([false, false])
                    .show(&mut inner_ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        let scroll_clip = card_rect.shrink(4.0).intersect(ui.clip_rect());
                        ui.set_clip_rect(scroll_clip);

                        for entry in entries {
                            match entry {
                                DrawerEntry::DottedDivider => {
                                    let w = ui.available_width().max(1.0);
                                    let (div_rect, _) =
                                        ui.allocate_exact_size(vec2(w, 16.0), Sense::hover());
                                    if ui.is_rect_visible(div_rect) {
                                        paint_drawer_dots(
                                            ui,
                                            div_rect,
                                            dark,
                                            14.0,
                                            interpolated_alpha,
                                        );
                                    }
                                }
                                DrawerEntry::SectionLabel(title) => {
                                    let w = ui.available_width().max(1.0);
                                    let (s_rect, _) =
                                        ui.allocate_exact_size(vec2(w, 26.0), Sense::hover());
                                    if ui.is_rect_visible(s_rect) {
                                        let col = p.text_muted.gamma_multiply(interpolated_alpha);
                                        let g = ui.painter().layout_no_wrap(
                                            (*title).to_owned(),
                                            FontId::new(12.5, FontFamily::Proportional),
                                            col,
                                        );
                                        ui.painter().with_clip_rect(scroll_clip).galley(
                                            pos2(
                                                s_rect.left() + 14.0,
                                                s_rect.center().y - g.size().y * 0.5,
                                            ),
                                            g,
                                            col,
                                        );
                                    }
                                }
                                DrawerEntry::Item {
                                    id,
                                    label,
                                    icon,
                                    badge_count,
                                } => {
                                    let is_sel = *selected_id == *id;
                                    let w = ui.available_width().max(1.0);
                                    let (rect, resp) =
                                        ui.allocate_exact_size(vec2(w, 46.0), Sense::click());

                                    if interpolated_alpha > 0.25
                                        && resp.clicked()
                                        && *selected_id != *id
                                    {
                                        *selected_id = *id;
                                        selection_changed = true;
                                    }

                                    if ui.is_rect_visible(rect) {
                                        let (press_t, hover_t) = interaction_factors(ui, &resp);

                                        if is_sel {
                                            ui.painter().rect_filled(
                                                rect,
                                                item_radius,
                                                p.selected_bg
                                                    .gamma_multiply(interpolated_alpha * 0.95),
                                            );
                                        }
                                        if hover_t > 0.001 && !is_sel {
                                            ui.painter().rect_filled(
                                                rect,
                                                item_radius,
                                                p.hover_overlay
                                                    .gamma_multiply(hover_t * interpolated_alpha),
                                            );
                                        }
                                        paint_pill_press_ripple(
                                            ui,
                                            rect,
                                            item_radius,
                                            p,
                                            press_t * interpolated_alpha,
                                        );

                                        let icon_col = (if is_sel {
                                            p.primary_high
                                        } else {
                                            p.text_muted
                                        })
                                        .gamma_multiply(interpolated_alpha);
                                        let icon_rect = Rect::from_center_size(
                                            pos2(rect.left() + 24.0, rect.center().y),
                                            Vec2::splat(20.0),
                                        );
                                        icon.paint(ui, icon_rect, icon_col);

                                        let text_col = (if is_sel {
                                            p.text_primary
                                        } else {
                                            p.text_secondary
                                        })
                                        .gamma_multiply(interpolated_alpha);
                                        let item_font = if is_sel {
                                            font_id(ui, 15.5, TwoUiFontWeight::SemiBold)
                                        } else {
                                            FontId::new(15.5, FontFamily::Proportional)
                                        };
                                        let galley = ui.painter().layout_no_wrap(
                                            (*label).to_owned(),
                                            item_font,
                                            text_col,
                                        );
                                        ui.painter().with_clip_rect(scroll_clip).galley(
                                            pos2(
                                                rect.left() + 48.0,
                                                rect.center().y - galley.size().y * 0.5,
                                            ),
                                            galley,
                                            text_col,
                                        );

                                        if let Some(badge) = badge_count {
                                            let bg_col = (if dark {
                                                Color32::from_rgb(0x99, 0x99, 0x99)
                                            } else {
                                                Color32::from_rgb(0x8C, 0x8C, 0x8C)
                                            })
                                            .gamma_multiply(interpolated_alpha);
                                            let bgal = ui.painter().layout_no_wrap(
                                                (*badge).to_owned(),
                                                FontId::new(12.5, FontFamily::Proportional),
                                                bg_col,
                                            );
                                            ui.painter().with_clip_rect(scroll_clip).galley(
                                                pos2(
                                                    rect.right() - 16.0 - bgal.size().x,
                                                    rect.center().y - bgal.size().y * 0.5,
                                                ),
                                                bgal,
                                                bg_col,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    });
            }
        });

    (selection_changed, action_clicked)
}
