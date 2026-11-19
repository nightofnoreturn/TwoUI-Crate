use egui::{vec2, Color32, Margin, Sense, Ui, Vec2};
use egui_twoui_icons::TwoUiIcon;
use egui_twoui_tokens::{theme_from_ui, tokens::dimens};

use super::CardSection;
use crate::button::{TwoUiButton, TwoUiButtonStyle, TwoUiIconButton};

/// Outcome returned from [`TipsCard::show`].
pub struct TipsCardResponse {
    pub dismissed: bool,
    pub clicked_action: Option<usize>,
}

/// Dismissible tip card container.
pub struct TipsCard<'a> {
    title: Option<&'a str>,
    summary: &'a str,
    actions: Vec<&'a str>,
    dismissible: bool,
}

impl<'a> TipsCard<'a> {
    pub fn new(summary: &'a str) -> Self {
        Self {
            title: None,
            summary,
            actions: Vec::new(),
            dismissible: true,
        }
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    pub fn action(mut self, label: &'a str) -> Self {
        self.actions.push(label);
        self
    }

    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    pub fn show(self, ui: &mut Ui) -> TipsCardResponse {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let mut dismissed = false;
        let mut clicked_action = None;

        CardSection::new()
            .inner_margin(Margin::symmetric(20, 16))
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 8.0;

                if self.title.is_some() || self.dismissible {
                    ui.horizontal(|ui| {
                        if let Some(title) = self.title {
                            ui.label(
                                egui::RichText::new(title)
                                    .size(17.0)
                                    .strong()
                                    .color(p.text_primary),
                            );
                        }
                        if self.dismissible {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui
                                    .add(
                                        TwoUiIconButton::new(TwoUiIcon::Close)
                                            .size(28.0)
                                            .icon_size(14.0)
                                            .tint(p.text_muted),
                                    )
                                    .clicked()
                                {
                                    dismissed = true;
                                }
                            });
                        }
                    });
                }

                ui.label(
                    egui::RichText::new(self.summary)
                        .size(14.0)
                        .color(p.text_secondary),
                );

                if !self.actions.is_empty() {
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.set_width(ui.available_width());
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            for (idx, label) in self.actions.iter().enumerate().rev() {
                                if ui
                                    .add(
                                        TwoUiButton::new(*label)
                                            .style(TwoUiButtonStyle::Transparent)
                                            .min_size(vec2(64.0, 32.0)),
                                    )
                                    .clicked()
                                {
                                    clicked_action = Some(idx);
                                }
                            }
                        });
                    });
                }
            });

        TipsCardResponse {
            dismissed,
            clicked_action,
        }
    }
}

/// Outcome returned from [`SuggestionCard::show`].
pub struct SuggestionCardResponse {
    pub dismissed: bool,
    pub action_clicked: bool,
}

/// Suggestion card (`26dp` corners, `15dp` title, `13dp` subtitle, close pill, and optional pager dots).
pub struct SuggestionCard<'a> {
    icon: Option<TwoUiIcon>,
    title: &'a str,
    subtitle: Option<&'a str>,
    summary: &'a str,
    action_label: &'a str,
    dismissible: bool,
    page_index: Option<&'a mut usize>,
    page_count: usize,
}

impl<'a> SuggestionCard<'a> {
    pub fn new(title: &'a str, summary: &'a str, action_label: &'a str) -> Self {
        Self {
            icon: None,
            title,
            subtitle: None,
            summary,
            action_label,
            dismissible: true,
            page_index: None,
            page_count: 1,
        }
    }

    pub fn icon(mut self, icon: TwoUiIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Optional secondary subtitle (`13dp`).
    pub fn subtitle(mut self, subtitle: &'a str) -> Self {
        self.subtitle = Some(subtitle);
        self
    }

    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    /// Attaches pager indicator dots so multiple suggestion pages can be cycled.
    pub fn pager(mut self, current_page: &'a mut usize, total_pages: usize) -> Self {
        self.page_index = Some(current_page);
        self.page_count = total_pages.max(1);
        self
    }

    pub fn show(mut self, ui: &mut Ui) -> SuggestionCardResponse {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let mut dismissed = false;
        let mut action_clicked = false;

        CardSection::new()
            .inner_margin(Margin::symmetric(20, 16))
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 8.0;

                ui.horizontal(|ui| {
                    if let Some(icon) = self.icon {
                        icon.show(ui, 22.0, p.text_primary);
                        ui.add_space(4.0);
                    }
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.label(
                            egui::RichText::new(self.title)
                                .size(dimens::APPBAR_SUGGEST_TITLE)
                                .strong()
                                .color(p.text_primary),
                        );
                        if let Some(sub) = self.subtitle {
                            ui.label(
                                egui::RichText::new(sub)
                                    .size(dimens::APPBAR_SUGGEST_SUBTITLE)
                                    .color(p.text_secondary),
                            );
                        }
                    });
                    if self.dismissible {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let close_bg = if theme.mode.is_dark() {
                                p.surface_container
                            } else {
                                Color32::from_rgba_unmultiplied(0x69, 0x69, 0x6C, 110)
                            };
                            if ui
                                .add(
                                    TwoUiIconButton::new(TwoUiIcon::Close)
                                        .size(20.0)
                                        .icon_size(11.0)
                                        .fill(close_bg)
                                        .tint(Color32::from_rgb(0xFA, 0xFA, 0xFF)),
                                )
                                .clicked()
                            {
                                dismissed = true;
                            }
                        });
                    }
                });

                ui.label(
                    egui::RichText::new(self.summary)
                        .size(14.5)
                        .color(p.text_secondary),
                );

                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    if let Some(page_idx) = self.page_index.as_deref_mut() {
                        if self.page_count > 1 {
                            *page_idx = (*page_idx).min(self.page_count - 1);
                            for idx in 0..self.page_count {
                                let is_cur = *page_idx == idx;
                                let d = if is_cur { 7.0 } else { 5.0 };
                                let (dr, dresp) =
                                    ui.allocate_exact_size(Vec2::splat(16.0), Sense::click());
                                if dresp.clicked() {
                                    *page_idx = idx;
                                }
                                let col = if is_cur {
                                    p.primary_high
                                } else {
                                    p.text_muted.gamma_multiply(0.45)
                                };
                                ui.painter().circle_filled(dr.center(), d * 0.5, col);
                            }
                        }
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .add(
                                TwoUiButton::primary(self.action_label)
                                    .min_size(vec2(118.0, 36.0)),
                            )
                            .clicked()
                        {
                            action_clicked = true;
                        }
                    });
                });
            });

        SuggestionCardResponse {
            dismissed,
            action_clicked,
        }
    }
}
