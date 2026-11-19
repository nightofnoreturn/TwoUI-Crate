use egui::{
    vec2, Align, Align2, Color32, CornerRadius, FontFamily, FontId, Frame, Layout, Margin, Rect,
    Response, Sense, Stroke, TextEdit, Ui, Vec2,
};
use egui_twoui_icons::TwoUiIcon;
use egui_twoui_tokens::{elevation_shadow, theme_from_ui, tokens::dimens, ElevationTier};

use crate::button::TwoUiIconButton;

/// Pill search bar (`52dp` plate height, `26dp` radius, optional floating elevated mode).
pub struct TwoUiSearchBar<'a> {
    query: &'a mut String,
    hint: String,
    show_back: bool,
    floating: bool,
    fill: Option<Color32>,
}

impl<'a> TwoUiSearchBar<'a> {
    pub fn new(query: &'a mut String) -> Self {
        Self {
            query,
            hint: "Search".to_owned(),
            show_back: false,
            floating: false,
            fill: None,
        }
    }

    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = hint.into();
        self
    }

    pub fn show_back_button(mut self, show: bool) -> Self {
        self.show_back = show;
        self
    }

    /// Styles the bar as an elevated floating search capsule (`ElevationTier::Md` + `0.5dp` stroke).
    pub fn floating(mut self, enabled: bool) -> Self {
        self.floating = enabled;
        self
    }

    pub fn fill(mut self, fill: Color32) -> Self {
        self.fill = Some(fill);
        self
    }

    /// Shows the search bar. Returns `(text_response, back_clicked)`.
    pub fn show(self, ui: &mut Ui) -> (Response, bool) {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let dark = theme.mode.is_dark();
        let mut back_clicked = false;

        let bg = self.fill.unwrap_or_else(|| {
            if self.floating {
                p.floating_surface
            } else {
                p.surface
            }
        });
        let stroke = Stroke::new(dimens::SEARCH_BOTTOM_STROKE, p.floating_stroke);
        let shadow = if self.floating {
            elevation_shadow(ElevationTier::Md, dark)
        } else {
            egui::epaint::Shadow::NONE
        };

        let plate_h = dimens::SEARCH_PLATE_HEIGHT;
        let v_pad = 9_i8;
        let row_h = (plate_h - f32::from(v_pad) * 2.0).max(28.0);
        let outer = Frame::NONE
            .fill(bg)
            .corner_radius(CornerRadius::same(dimens::SEARCH_PLATE_RADIUS))
            .stroke(stroke)
            .shadow(shadow)
            .inner_margin(Margin::symmetric(14, v_pad))
            .show(ui, |ui| {
                let row_w = ui.available_width();
                ui.allocate_ui_with_layout(
                    vec2(row_w, row_h),
                    Layout::left_to_right(Align::Center),
                    |ui| {
                        ui.spacing_mut().item_spacing.x = 8.0;

                        if self.show_back {
                            if ui
                                .add(
                                    TwoUiIconButton::new(TwoUiIcon::Back)
                                        .size(30.0)
                                        .icon_size(18.0)
                                        .tint(p.text_primary),
                                )
                                .clicked()
                            {
                                back_clicked = true;
                            }
                        } else {
                            let (icon_r, _) =
                                ui.allocate_exact_size(Vec2::splat(30.0), Sense::hover());
                            if ui.is_rect_visible(icon_r) {
                                TwoUiIcon::Search.paint(
                                    ui,
                                    Rect::from_center_size(icon_r.center(), Vec2::splat(18.0)),
                                    p.text_muted,
                                );
                            }
                        }

                        let clear_slot = if self.query.is_empty() { 0.0 } else { 32.0 };
                        let edit_w = (ui.available_width() - clear_slot).max(60.0);

                        let edit_resp = ui.allocate_ui_with_layout(
                            vec2(edit_w, row_h),
                            Layout::left_to_right(Align::Center),
                            |ui| {
                                ui.add(
                                    TextEdit::singleline(self.query)
                                        .frame(Frame::NONE)
                                        .margin(Margin::ZERO)
                                        .align(Align2::LEFT_CENTER)
                                        .desired_width(edit_w)
                                        .font(FontId::new(16.0, FontFamily::Proportional))
                                        .text_color(p.text_primary)
                                        .hint_text(
                                            egui::RichText::new(&self.hint)
                                                .size(16.0)
                                                .color(p.text_muted),
                                        ),
                                )
                            },
                        );

                        if !self.query.is_empty()
                            && ui
                                .add(
                                    TwoUiIconButton::new(TwoUiIcon::Close)
                                        .size(24.0)
                                        .icon_size(12.0)
                                        .fill(p.text_muted.gamma_multiply(0.25))
                                        .tint(p.text_primary),
                                )
                                .clicked()
                        {
                            self.query.clear();
                        }

                        edit_resp.inner
                    },
                )
                .inner
            });

        (outer.inner, back_clicked)
    }
}
