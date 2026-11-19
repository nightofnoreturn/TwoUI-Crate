//! Horizontal, vertical, center-based, and preference SeekBars showcase page.

use egui::{vec2, Align, Layout, Margin, RichText, Ui};
use egui_twoui::prelude::*;

use crate::app::ShowcaseApp;

impl ShowcaseApp {
    pub(crate) fn render_seekbars_page(&mut self, ui: &mut Ui) {
        SectionHeader::new("Horizontal SeekBars").show(ui);
        CardSection::new()
            .inner_margin(Margin::symmetric(22, 16))
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 12.0;
                let muted = theme_from_ui(ui).palette.text_muted;

                ui.label(RichText::new(format!("Standard — 3.5 -> 13dp ({:.0}%)", self.seek_standard)).size(13.0).color(muted));
                ui.twoui_seekbar(&mut self.seek_standard, 0.0..=100.0, SeekBarMode::Standard);

                ui.label(RichText::new(format!("Expand — 14 -> 22dp ({:.0}%)", self.seek_expand)).size(13.0).color(muted));
                ui.twoui_seekbar(&mut self.seek_expand, 0.0..=100.0, SeekBarMode::Expand);

                ui.label(RichText::new(format!("Dual-Color Overlap > 70% ({:.0}%)", self.seek_overlap)).size(13.0).color(muted));
                ui.add(
                    TwoUiSeekBar::new(&mut self.seek_overlap, 0.0..=100.0)
                        .mode(SeekBarMode::Expand)
                        .overlap_threshold(0.70),
                );

                ui.label(RichText::new(format!("LevelBar Discrete 10..20 ({:.0})", self.seek_level_stepped)).size(13.0).color(muted));
                ui.add(
                    TwoUiSeekBar::new(&mut self.seek_level_stepped, 10.0..=20.0)
                        .mode(SeekBarMode::Level { steps: 10 })
                        .seamless(false),
                );

                ui.label(RichText::new(format!("LevelBar Seamless 0..7 ({:.1})", self.seek_level_seamless)).size(13.0).color(muted));
                ui.add(
                    TwoUiSeekBar::new(&mut self.seek_level_seamless, 0.0..=7.0)
                        .mode(SeekBarMode::Level { steps: 7 })
                        .seamless(true),
                );

                ui.label(RichText::new(format!("SeekBarPlus Center-Based Seamless 0..20 ({:.1})", self.seek_plus_seamless)).size(13.0).color(muted));
                ui.add(TwoUiSeekBarPlus::new(&mut self.seek_plus_seamless, 0.0..=20.0).seamless(true));

                ui.label(RichText::new(format!("SeekBarPlus Center-Based Discrete 0..2 ({:.0})", self.seek_plus_stepped)).size(13.0).color(muted));
                ui.add(TwoUiSeekBarPlus::new(&mut self.seek_plus_stepped, 0.0..=2.0).seamless(false));
            });

        SectionHeader::new("Vertical SeekBars").show(ui);
        CardSection::new()
            .inner_margin(Margin::symmetric(22, 16))
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.allocate_ui_with_layout(
                        vec2(210.0, 204.0),
                        Layout::left_to_right(Align::Min),
                        |ui| {
                            ui.spacing_mut().item_spacing.x = 40.0;
                            ui.vertical_centered(|ui| {
                                ui.set_width(85.0);
                                ui.add(
                                    TwoUiSeekBar::new(&mut self.seek_vert_standard, 0.0..=100.0)
                                        .mode(SeekBarMode::Standard)
                                        .vertical(true)
                                        .length(175.0),
                                );
                                ui.label(format!("Std {:.0}%", self.seek_vert_standard));
                            });
                            ui.vertical_centered(|ui| {
                                ui.set_width(85.0);
                                ui.add(
                                    TwoUiSeekBar::new(&mut self.seek_vert_expand, 0.0..=100.0)
                                        .mode(SeekBarMode::Expand)
                                        .vertical(true)
                                        .length(175.0),
                                );
                                ui.label(format!("Expand {:.0}%", self.seek_vert_expand));
                            });
                        },
                    );
                });
            });

        SectionHeader::new("SeekBarPreferencePro").show(ui);
        CardSection::new()
            .inner_margin(Margin::symmetric(20, 14))
            .show(ui, |ui| {
                ui.add(
                    TwoUiSeekBarPro::new("Media volume", &mut self.seek_pro, 0.0..=100.0)
                        .step(5.0)
                        .unit("%")
                        .end_labels("Mute", "Max"),
                );
            });
    }
}
