use egui::{pos2, vec2, CornerRadius, FontFamily, FontId, Margin, Sense, Ui};
use egui_twoui_tokens::{interaction_factors, paint_state_overlay, recoil_rect, theme_from_ui};

use super::CardSection;

/// Related navigation links card.
pub struct RelativeLinksCard<'a> {
    title: &'a str,
    links: &'a [&'a str],
}

impl<'a> RelativeLinksCard<'a> {
    pub fn new(links: &'a [&'a str]) -> Self {
        Self {
            title: "Looking for something else?",
            links,
        }
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }

    /// Renders the card and returns the index of the clicked link, if any.
    pub fn show(self, ui: &mut Ui) -> Option<usize> {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let mut clicked = None;

        CardSection::new()
            .inner_margin(Margin::symmetric(8, 20))
            .show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                    ui.spacing_mut().item_spacing.y = 4.0;
                    let avail_w = ui.available_width().max(120.0);
                    let title_galley = ui.painter().layout_no_wrap(
                        self.title.to_owned(),
                        FontId::new(18.0, FontFamily::Proportional),
                        p.text_primary,
                    );
                    let (title_r, _) = ui.allocate_exact_size(
                        vec2(avail_w, title_galley.size().y + 6.0),
                        Sense::hover(),
                    );
                    ui.painter().galley(
                        pos2(title_r.left() + 16.0, title_r.top()),
                        title_galley,
                        p.text_primary,
                    );

                    for (idx, link) in self.links.iter().enumerate() {
                        let font_id = FontId::new(17.0, FontFamily::Proportional);
                        let galley = ui.painter().layout_no_wrap(
                            (*link).to_owned(),
                            font_id,
                            p.primary_high,
                        );
                        let row_size = vec2(
                            (galley.size().x + 32.0).min(avail_w),
                            galley.size().y + 12.0,
                        );
                        let (rect, resp) = ui.allocate_exact_size(row_size, Sense::click());
                        if ui.is_rect_visible(rect) {
                            let (press_t, hover_t) = interaction_factors(ui, &resp);
                            let paint_rect = recoil_rect(rect, press_t, 1.0);
                            paint_state_overlay(
                                ui,
                                paint_rect,
                                CornerRadius::same(26),
                                p,
                                hover_t,
                                press_t,
                            );
                            ui.painter().galley(
                                pos2(
                                    paint_rect.left() + 16.0,
                                    paint_rect.center().y - galley.size().y * 0.5,
                                ),
                                galley,
                                p.primary_high,
                            );
                        }
                        if resp.clicked() {
                            clicked = Some(idx);
                        }
                    }
                });
            });

        clicked
    }
}
