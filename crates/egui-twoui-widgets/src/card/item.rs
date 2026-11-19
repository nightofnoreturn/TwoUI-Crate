use egui::{
    pos2, vec2, Color32, CornerRadius, FontFamily, FontId, Rect, Response, Role, Sense, Ui, Vec2,
    Widget, WidgetInfo,
};
use egui_twoui_icons::TwoUiIcon;
use egui_twoui_tokens::{
    interaction_factors, paint_state_overlay, recoil_rect, theme_from_ui, tokens::dimens,
};

use super::{layout_row_galleys, paint_divider, DividerInset};
use crate::controls::paint_badge;

/// Clickable list row inside a [`super::CardSection`].
pub struct CardItem {
    title: String,
    summary: Option<String>,
    summary_highlight: bool,
    leading_icon: Option<TwoUiIcon>,
    icon_tint: Option<Color32>,
    trailing_icon: Option<TwoUiIcon>,
    trailing_text: Option<String>,
    badge: Option<String>,
    show_top_divider: bool,
    full_width_divider: bool,
    enabled: bool,
}

impl CardItem {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            summary: None,
            summary_highlight: false,
            leading_icon: None,
            icon_tint: None,
            trailing_icon: None,
            trailing_text: None,
            badge: None,
            show_top_divider: false,
            full_width_divider: false,
            enabled: true,
        }
    }

    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    /// Colors the summary with the theme accent color.
    pub fn summary_highlight(mut self, highlight: bool) -> Self {
        self.summary_highlight = highlight;
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

    pub fn trailing_icon(mut self, icon: TwoUiIcon) -> Self {
        self.trailing_icon = Some(icon);
        self
    }

    pub fn chevron(self) -> Self {
        self.trailing_icon(TwoUiIcon::ChevronRight)
    }

    pub fn trailing_text(mut self, text: impl Into<String>) -> Self {
        self.trailing_text = Some(text.into());
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

    pub fn full_width_divider(mut self, full: bool) -> Self {
        self.full_width_divider = full;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl Widget for CardItem {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let is_enabled = self.enabled && ui.is_enabled();

        if self.show_top_divider {
            let inset = if self.leading_icon.is_some() && !self.full_width_divider {
                DividerInset::WithLeading
            } else {
                DividerInset::Standard
            };
            paint_divider(ui, inset);
        }

        let width = ui.available_width().max(120.0);
        let left_pad = dimens::LIST_HORIZONTAL_PADDING;
        let right_pad = dimens::LIST_HORIZONTAL_PADDING;
        let icon_slot = if self.leading_icon.is_some() { 42.0 } else { 0.0 };
        let trailing_icon_slot = if self.trailing_icon.is_some() { 26.0 } else { 0.0 };

        let trailing_galley = self.trailing_text.as_ref().map(|t| {
            ui.painter().layout_no_wrap(
                t.clone(),
                FontId::new(14.0, FontFamily::Proportional),
                p.primary_high,
            )
        });
        let trailing_text_w = trailing_galley
            .as_ref()
            .map_or(0.0, |g| g.size().x + 10.0);
        let text_avail =
            (width - left_pad - right_pad - icon_slot - trailing_icon_slot - trailing_text_w)
                .max(60.0);

        let title_col = if is_enabled {
            p.text_primary
        } else {
            p.text_disabled
        };
        let summary_col = if !is_enabled {
            p.text_disabled
        } else if self.summary_highlight {
            p.primary_high
        } else {
            p.text_muted
        };
        let row_text = layout_row_galleys(
            ui,
            &self.title,
            title_col,
            self.summary.as_deref(),
            summary_col,
            text_avail,
        );
        let height = (row_text.block_height + 22.0).max(dimens::LIST_ITEM_MIN_HEIGHT_SMALL);

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

            paint_state_overlay(
                ui,
                recoil_rect(rect, press_t, 1.0).shrink2(vec2(6.0, 2.0)),
                CornerRadius::same(20),
                p,
                hover_t,
                press_t,
            );

            let content_rect = recoil_rect(rect, press_t, 1.0);
            let mut cursor_x = content_rect.left() + left_pad;

            if let Some(icon) = self.leading_icon {
                let icon_box = Rect::from_center_size(
                    pos2(cursor_x + 14.0, content_rect.center().y),
                    Vec2::splat(30.0),
                );
                let tint = self.icon_tint.unwrap_or(p.text_primary);
                let tint = if is_enabled {
                    tint
                } else {
                    tint.gamma_multiply(0.4)
                };
                icon.paint(
                    ui,
                    Rect::from_center_size(icon_box.center(), Vec2::splat(22.0)),
                    tint,
                );
                cursor_x += icon_slot;
            }

            let text_top = content_rect.center().y - row_text.block_height * 0.5;
            let title_size = row_text.title.size();
            ui.painter()
                .galley(pos2(cursor_x, text_top), row_text.title, title_col);

            if let Some(badge) = &self.badge {
                let bx =
                    (cursor_x + title_size.x + 14.0).min(content_rect.right() - right_pad - 12.0);
                paint_badge(ui, pos2(bx, text_top + title_size.y * 0.45), badge);
            }

            if let Some(sg) = row_text.summary {
                ui.painter().galley(
                    pos2(cursor_x, text_top + title_size.y + 3.0),
                    sg,
                    summary_col,
                );
            }

            let mut right_x = content_rect.right() - right_pad;
            if let Some(t_icon) = self.trailing_icon {
                let t_rect = Rect::from_center_size(
                    pos2(right_x - 8.0, content_rect.center().y),
                    Vec2::splat(16.0),
                );
                t_icon.paint(ui, t_rect, p.text_muted);
                right_x -= trailing_icon_slot;
            }

            if let Some(tg) = trailing_galley {
                let ts = tg.size();
                ui.painter().galley(
                    pos2(right_x - ts.x, content_rect.center().y - ts.y * 0.5),
                    tg,
                    if is_enabled {
                        p.primary_high
                    } else {
                        p.text_disabled
                    },
                );
            }
        }

        response.widget_info(|| WidgetInfo::labeled(Role::ListItem, is_enabled, &self.title));
        response
    }
}
