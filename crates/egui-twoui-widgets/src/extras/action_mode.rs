use egui::{vec2, Align2, Area, Context, CornerRadius, Frame, Id, Margin, Order, Rect, Stroke};
use egui_twoui_icons::TwoUiIcon;
use egui_twoui_tokens::{elevation_shadow, theme_from_ctx, tokens::dimens, ElevationTier};

use crate::button::{TwoUiButton, TwoUiButtonStyle, TwoUiIconButton};

/// Floating multi-select `ActionMode` bottom bar shown when items are selected.
pub struct TwoUiActionModeBar {
    selected_count: usize,
    total_count: usize,
}

impl TwoUiActionModeBar {
    pub fn new(selected_count: usize, total_count: usize) -> Self {
        Self {
            selected_count,
            total_count,
        }
    }

    /// Shows the floating ActionMode capsule; returns `(toggle_select_all, delete_clicked, done_clicked)`.
    pub fn show(self, ctx: &Context, host_rect: Rect) -> (bool, bool, bool) {
        if self.selected_count == 0 {
            return (false, false, false);
        }
        let theme = theme_from_ctx(ctx);
        let p = &theme.palette;
        let dark = theme.mode.is_dark();
        let mut toggle_all = false;
        let mut delete_clicked = false;
        let mut done_clicked = false;

        let frame = Frame::NONE
            .fill(p.floating_surface)
            .corner_radius(CornerRadius::same(dimens::CAPSULE_CORNER_RADIUS))
            .stroke(Stroke::new(0.8, p.floating_stroke))
            .inner_margin(Margin::symmetric(14, 8))
            .shadow(elevation_shadow(ElevationTier::Md, dark));
        Area::new(Id::unique("twoui_action_mode"))
            .order(Order::Foreground)
            .anchor(Align2::CENTER_BOTTOM, vec2(0.0, -20.0))
            .pivot(Align2::CENTER_BOTTOM)
            .show(ctx, |ui| {
                ui.set_max_width((host_rect.width() - 32.0).max(200.0));
                frame.show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let all_lbl = if self.selected_count >= self.total_count {
                            "Clear all"
                        } else {
                            "All"
                        };
                        if ui
                            .add(
                                TwoUiButton::new(all_lbl)
                                    .style(TwoUiButtonStyle::Transparent)
                                    .min_size(vec2(64.0, 34.0)),
                            )
                            .clicked()
                        {
                            toggle_all = true;
                        }
                        ui.add_space(6.0);
                        ui.label(
                            egui::RichText::new(format!("{} selected", self.selected_count))
                                .size(14.5)
                                .strong()
                                .color(p.text_primary),
                        );
                        ui.add_space(10.0);
                        if ui
                            .add(
                                TwoUiIconButton::new(TwoUiIcon::Delete)
                                    .size(34.0)
                                    .icon_size(18.0)
                                    .tint(p.functional_red),
                            )
                            .clicked()
                        {
                            delete_clicked = true;
                        }
                        if ui
                            .add(
                                TwoUiButton::new("Done")
                                    .style(TwoUiButtonStyle::Primary)
                                    .min_size(vec2(68.0, 34.0)),
                            )
                            .clicked()
                        {
                            done_clicked = true;
                        }
                    });
                });
            });

        (toggle_all, delete_clicked, done_clicked)
    }
}
