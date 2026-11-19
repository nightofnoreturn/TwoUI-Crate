use egui::{
    pos2, vec2, Color32, CornerRadius, Rect, Response, Role, Sense, Stroke, Ui, Vec2, Widget,
    WidgetInfo,
};
use egui_twoui_icons::TwoUiIcon;
use egui_twoui_tokens::{
    interaction_factors, paint_state_overlay, recoil_rect, theme_from_ui, tokens::dimens,
};

use super::{layout_row_galleys, paint_divider, DividerInset};
use crate::controls::{paint_badge, TwoUiRadioButton, TwoUiSwitch};

/// Response returned by [`SwitchItem`] so callers can inspect row clicks vs switch toggles
/// (especially when `separate_switch(true)` is enabled).
pub struct SwitchItemResponse {
    /// Combined row interaction response.
    pub response: Response,
    /// `true` if the switch value flipped this frame.
    pub toggled: bool,
    /// `true` if the body area (not the switch) was clicked when `separate_switch` is true.
    pub body_clicked: bool,
}

/// List row with title, optional summary/icon, and right-aligned toggle switch.
pub struct SwitchItem<'a> {
    checked: &'a mut bool,
    title: String,
    summary_on: Option<String>,
    summary_off: Option<String>,
    leading_icon: Option<TwoUiIcon>,
    icon_tint: Option<Color32>,
    badge: Option<String>,
    separate_switch: bool,
    user_updatable_summary: bool,
    show_top_divider: bool,
    enabled: bool,
}

impl<'a> SwitchItem<'a> {
    pub fn new(checked: &'a mut bool, title: impl Into<String>) -> Self {
        Self {
            checked,
            title: title.into(),
            summary_on: None,
            summary_off: None,
            leading_icon: None,
            icon_tint: None,
            badge: None,
            separate_switch: false,
            user_updatable_summary: false,
            show_top_divider: false,
            enabled: true,
        }
    }

    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        let s = summary.into();
        self.summary_on = Some(s.clone());
        self.summary_off = Some(s);
        self
    }

    pub fn summary_states(
        mut self,
        on_summary: impl Into<String>,
        off_summary: impl Into<String>,
    ) -> Self {
        self.summary_on = Some(on_summary.into());
        self.summary_off = Some(off_summary.into());
        self
    }

    /// Draws a vertical hairline divider between the item body and the switch (`separateSwitch`).
    pub fn separate_switch(mut self, separate: bool) -> Self {
        self.separate_switch = separate;
        self
    }

    /// Highlights summary text with the theme accent color when checked.
    pub fn user_updatable_summary(mut self, updatable: bool) -> Self {
        self.user_updatable_summary = updatable;
        self
    }

    pub fn icon(mut self, icon: TwoUiIcon) -> Self {
        self.leading_icon = Some(icon);
        self
    }

    pub fn icon_tint(mut self, color: Color32) -> Self {
        self.icon_tint = Some(color);
        self
    }

    pub fn badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    pub fn top_divider(mut self, show: bool) -> Self {
        self.show_top_divider = show;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn show(self, ui: &mut Ui) -> SwitchItemResponse {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let is_enabled = self.enabled && ui.is_enabled();

        if self.show_top_divider {
            let inset = if self.leading_icon.is_some() {
                DividerInset::WithLeading
            } else {
                DividerInset::Standard
            };
            paint_divider(ui, inset);
        }

        let width = ui.available_width().max(140.0);
        let left_pad = dimens::LIST_HORIZONTAL_PADDING;
        let right_pad = 16.0;
        let icon_slot = if self.leading_icon.is_some() { 42.0 } else { 0.0 };
        let switch_zone_w =
            TwoUiSwitch::DEFAULT_SIZE.x + if self.separate_switch { 28.0 } else { 16.0 };
        let text_avail = (width - left_pad - right_pad - icon_slot - switch_zone_w).max(60.0);

        let active_summary = if *self.checked {
            self.summary_on.as_deref()
        } else {
            self.summary_off.as_deref()
        };

        let title_col = if is_enabled {
            p.text_primary
        } else {
            p.text_disabled
        };
        let summary_col = if !is_enabled {
            p.text_disabled
        } else if self.user_updatable_summary && *self.checked {
            p.primary_high
        } else {
            p.text_muted
        };
        let row_text = layout_row_galleys(
            ui,
            &self.title,
            title_col,
            active_summary,
            summary_col,
            text_avail,
        );
        let height = (row_text.block_height + 22.0).max(dimens::LIST_ITEM_MIN_HEIGHT_SMALL);

        let (rect, mut row_resp) = ui.allocate_exact_size(vec2(width, height), Sense::hover());

        let switch_hit_rect = Rect::from_min_max(
            pos2(
                rect.right() - right_pad - TwoUiSwitch::DEFAULT_SIZE.x - 10.0,
                rect.top(),
            ),
            rect.max,
        );
        let body_hit_rect = if self.separate_switch {
            Rect::from_min_max(rect.min, pos2(switch_hit_rect.left(), rect.bottom()))
        } else {
            rect
        };

        let sense = if is_enabled {
            Sense::click()
        } else {
            Sense::hover()
        };
        let body_resp = ui.interact(body_hit_rect, row_resp.id.with("body"), sense);
        let switch_resp = if self.separate_switch {
            ui.interact(switch_hit_rect, row_resp.id.with("sw_hit"), sense)
        } else {
            body_resp.clone()
        };

        let mut toggled = false;
        let mut body_clicked = false;
        if is_enabled {
            if self.separate_switch {
                if switch_resp.clicked() {
                    *self.checked = !*self.checked;
                    toggled = true;
                    row_resp.mark_changed();
                } else if body_resp.clicked() {
                    body_clicked = true;
                }
            } else if body_resp.clicked() {
                *self.checked = !*self.checked;
                toggled = true;
                row_resp.mark_changed();
            }
        }

        if ui.is_rect_visible(rect) {
            let (press_t, hover_t) = if is_enabled {
                interaction_factors(ui, &body_resp)
            } else {
                (0.0, 0.0)
            };

            let paint_rect = recoil_rect(rect, press_t, 1.0);
            let overlay_raw = recoil_rect(body_hit_rect, press_t, 1.0);
            let overlay_rect = Rect::from_min_max(
                pos2(overlay_raw.left() + 6.0, overlay_raw.top() + 2.0),
                pos2(
                    overlay_raw.right() - if self.separate_switch { 2.0 } else { 6.0 },
                    overlay_raw.bottom() - 2.0,
                ),
            );
            let overlay_radius = if self.separate_switch {
                CornerRadius {
                    nw: 20,
                    ne: 10,
                    sw: 20,
                    se: 10,
                }
            } else {
                CornerRadius::same(20)
            };
            paint_state_overlay(ui, overlay_rect, overlay_radius, p, hover_t, press_t);

            let mut cursor_x = paint_rect.left() + left_pad;
            if let Some(icon) = self.leading_icon {
                let tint = self.icon_tint.unwrap_or(p.text_primary);
                let tint = if is_enabled {
                    tint
                } else {
                    tint.gamma_multiply(0.4)
                };
                icon.paint(
                    ui,
                    Rect::from_center_size(
                        pos2(cursor_x + 14.0, paint_rect.center().y),
                        Vec2::splat(22.0),
                    ),
                    tint,
                );
                cursor_x += icon_slot;
            }

            let text_top = paint_rect.center().y - row_text.block_height * 0.5;
            let title_size = row_text.title.size();
            ui.painter()
                .galley(pos2(cursor_x, text_top), row_text.title, title_col);

            if let Some(badge) = &self.badge {
                let bx = (cursor_x + title_size.x + 14.0).min(switch_hit_rect.left() - 12.0);
                paint_badge(ui, pos2(bx, text_top + title_size.y * 0.45), badge);
            }

            if let Some(sg) = row_text.summary {
                ui.painter().galley(
                    pos2(cursor_x, text_top + title_size.y + 3.0),
                    sg,
                    summary_col,
                );
            }

            if self.separate_switch {
                let div_x = switch_hit_rect.left() + 2.0;
                ui.painter().line_segment(
                    [
                        pos2(div_x, paint_rect.center().y - 11.0),
                        pos2(div_x, paint_rect.center().y + 11.0),
                    ],
                    Stroke::new(1.0, p.divider),
                );
            }

            let switch_press_t = if is_enabled {
                interaction_factors(ui, &switch_resp).0
            } else {
                0.0
            };
            let sw_rect = Rect::from_min_size(
                pos2(
                    paint_rect.right() - right_pad - TwoUiSwitch::DEFAULT_SIZE.x,
                    paint_rect.center().y - TwoUiSwitch::DEFAULT_SIZE.y * 0.5,
                ),
                TwoUiSwitch::DEFAULT_SIZE,
            );
            TwoUiSwitch::paint_only(
                ui,
                row_resp.id.with("sw"),
                sw_rect,
                *self.checked,
                is_enabled,
                switch_press_t,
            );
        }

        let checked_val = *self.checked;
        row_resp.widget_info(|| {
            WidgetInfo::selected(Role::Switch, is_enabled, checked_val, &self.title)
        });

        SwitchItemResponse {
            response: body_resp.union(switch_resp),
            toggled,
            body_clicked,
        }
    }
}

/// Full-row radio selection item.
pub struct RadioItem {
    selected: bool,
    title: String,
    summary: Option<String>,
    show_top_divider: bool,
    enabled: bool,
}

impl RadioItem {
    pub fn new(selected: bool, title: impl Into<String>) -> Self {
        Self {
            selected,
            title: title.into(),
            summary: None,
            show_top_divider: false,
            enabled: true,
        }
    }

    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    pub fn top_divider(mut self, show: bool) -> Self {
        self.show_top_divider = show;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl Widget for RadioItem {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let is_enabled = self.enabled && ui.is_enabled();

        if self.show_top_divider {
            paint_divider(ui, DividerInset::WithLeading);
        }

        let width = ui.available_width().max(120.0);
        let left_pad = dimens::LIST_HORIZONTAL_PADDING;
        let radio_slot = 38.0;
        let text_avail = (width - left_pad - dimens::LIST_HORIZONTAL_PADDING - radio_slot).max(60.0);

        let title_col = if is_enabled {
            p.text_primary
        } else {
            p.text_disabled
        };
        let summary_col = if is_enabled {
            p.text_muted
        } else {
            p.text_disabled
        };
        let row_text = layout_row_galleys(
            ui,
            &self.title,
            title_col,
            self.summary.as_deref(),
            summary_col,
            text_avail,
        );
        let height = (row_text.block_height + 24.0).max(dimens::LIST_ITEM_MIN_HEIGHT);

        let sense = if is_enabled {
            Sense::click()
        } else {
            Sense::hover()
        };
        let (rect, response) = ui.allocate_exact_size(vec2(width, height), sense);

        if ui.is_rect_visible(rect) {
            let (press_t, hover_t) = if is_enabled {
                interaction_factors(ui, &response)
            } else {
                (0.0, 0.0)
            };
            let paint_rect = recoil_rect(rect, press_t, 1.0);
            paint_state_overlay(
                ui,
                paint_rect.shrink2(vec2(6.0, 2.0)),
                CornerRadius::same(20),
                p,
                hover_t,
                press_t,
            );

            let radio_center = pos2(paint_rect.left() + left_pad + 10.0, paint_rect.center().y);
            TwoUiRadioButton::paint_only(
                ui,
                response.id.with("radio_row"),
                radio_center,
                self.selected,
                is_enabled,
                press_t,
            );

            let text_x = paint_rect.left() + left_pad + radio_slot;
            let text_top = paint_rect.center().y - row_text.block_height * 0.5;
            let title_h = row_text.title.size().y;
            ui.painter()
                .galley(pos2(text_x, text_top), row_text.title, title_col);
            if let Some(sg) = row_text.summary {
                ui.painter()
                    .galley(pos2(text_x, text_top + title_h + 3.0), sg, summary_col);
            }
        }

        response.widget_info(|| {
            WidgetInfo::selected(Role::RadioButton, is_enabled, self.selected, &self.title)
        });
        response
    }
}
