//! Preferences, theme controls, SuggestionCard, TipsCard, and SeekBarPreferences page.

use egui::{vec2, Align, Frame, Layout, Margin, RichText, Ui};
use egui_twoui::prelude::*;

use crate::{
    app::ShowcaseApp,
    data::{Page, ACCENT_PRESETS},
};

impl ShowcaseApp {
    pub(crate) fn render_preferences_page(&mut self, ui: &mut Ui) {
        if self.show_suggestion_card {
            let pages = [
                (
                    "Accent & Style",
                    "Theme",
                    "Try Violet or Emerald accent with translucent capsules.",
                    "Apply Violet",
                ),
                (
                    "Frameless Window",
                    "Desktop",
                    "Drag empty toolbar space or resize any window edge.",
                    "Toggle Theme",
                ),
                (
                    "Split Capsules",
                    "Floating Actions",
                    "Grouped split capsule bars for quick actions.",
                    "Open Widgets",
                ),
            ];
            let (title, sub, summary, cta) = pages[self.suggestion_page.min(pages.len() - 1)];
            let sug_resp = SuggestionCard::new(title, summary, cta)
                .subtitle(sub)
                .icon(TwoUiIcon::Palette)
                .pager(&mut self.suggestion_page, pages.len())
                .show(ui);
            if sug_resp.dismissed {
                self.show_suggestion_card = false;
            } else if sug_resp.action_clicked {
                match self.suggestion_page {
                    0 => {
                        self.accent_index = 2;
                        self.custom_accent = ACCENT_PRESETS[2].1;
                        self.sync_theme(ui.ctx());
                    }
                    1 => {
                        self.dark_mode = !self.dark_mode;
                        self.sync_theme(ui.ctx());
                    }
                    _ => self.page_id = Page::Widgets.id(),
                }
            }
            ui.add_space(10.0);
        }

        if self.show_tips_card {
            let tip_resp = TipsCard::new("Dismissible tip card container.")
                .title("TipsCard")
                .action("Dismiss")
                .show(ui);
            if tip_resp.dismissed || tip_resp.clicked_action.is_some() {
                self.show_tips_card = false;
            }
            ui.add_space(10.0);
        }

        SectionHeader::new("Dark Mode & Appearance").show(ui);
        CardSection::new().show(ui, |ui| {
            let mode_icon = if self.dark_mode {
                TwoUiIcon::Moon
            } else {
                TwoUiIcon::Sun
            };
            let dark_sw = SwitchItem::new(&mut self.dark_mode, "Dark mode")
                .icon(mode_icon)
                .summary_states("OLED Dark (#010102)", "Clean Light (#F3F3F5)")
                .user_updatable_summary(true)
                .show(ui);
            if dark_sw.toggled {
                self.follow_system = false;
                self.sync_theme(ui.ctx());
            }
            let sys_sw = SwitchItem::new(&mut self.follow_system, "Use system default")
                .top_divider(true)
                .show(ui);
            if sys_sw.toggled {
                self.sync_theme(ui.ctx());
                if self.follow_system {
                    if let Some(os_mode) = detect_system_mode(ui.ctx(), true) {
                        self.dark_mode = os_mode.is_dark();
                        self.sync_theme(ui.ctx());
                    } else if let Some(sys_t) = ui.ctx().system_theme() {
                        self.dark_mode = matches!(sys_t, egui::Theme::Dark);
                        self.sync_theme(ui.ctx());
                    }
                }
            }

            paint_divider(ui, DividerInset::WithLeading);
            Frame::NONE
                .inner_margin(Margin::symmetric(20, 12))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Accent presets & ColorPicker").size(15.0));
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if ui.add(TwoUiButton::primary("Custom Color")).clicked() {
                                self.color_dialog_draft = self.custom_accent;
                                self.color_dialog_open = true;
                            }
                        });
                    });
                    ui.add_space(8.0);
                    ui.horizontal_wrapped(|ui| {
                        for (idx, (name, col)) in ACCENT_PRESETS.iter().enumerate() {
                            let sel = self.accent_index == idx && self.custom_accent == *col;
                            if ui
                                .add(
                                    TwoUiButton::new(*name)
                                        .style(if sel {
                                            TwoUiButtonStyle::Primary
                                        } else {
                                            TwoUiButtonStyle::Default
                                        })
                                        .min_size(vec2(92.0, 34.0)),
                                )
                                .clicked()
                            {
                                self.accent_index = idx;
                                self.custom_accent = *col;
                                self.sync_theme(ui.ctx());
                            }
                        }
                    });
                });

            paint_divider(ui, DividerInset::WithLeading);
            Frame::NONE
                .inner_margin(Margin::symmetric(20, 12))
                .show(ui, |ui| {
                    let presets = [
                        "Roboto Flex Tuned",
                        "Roboto Flex Stock",
                        "Roboto Flex Wide",
                        "egui Default",
                    ];
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Typography (Roboto Flex)").size(15.0));
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if TwoUiDropdown::new(
                                "pref_typography_sel",
                                &presets,
                                &mut self.typography_idx,
                            )
                            .min_width(215.0)
                            .show(ui)
                            .changed()
                            {
                                match self.typography_idx {
                                    0 => install_typography(
                                        ui.ctx(),
                                        TwoUiTypography::tuned(),
                                    ),
                                    1 => install_typography(
                                        ui.ctx(),
                                        TwoUiTypography::stock_roboto(),
                                    ),
                                    2 => install_typography(
                                        ui.ctx(),
                                        TwoUiTypography::rounded_wide(),
                                    ),
                                    _ => reset_default_fonts(ui.ctx()),
                                }
                            }
                        });
                    });
                    ui.add_space(6.0);
                    let muted = theme_from_ui(ui).palette.text_muted;
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing.x = 12.0;
                        ui.label(
                            RichText::new("Display")
                                .font(font_id(ui, 16.0, TwoUiFontWeight::Display)),
                        );
                        ui.label(
                            RichText::new("SemiBold")
                                .font(font_id(ui, 14.5, TwoUiFontWeight::SemiBold)),
                        );
                        ui.label(
                            RichText::new("Medium")
                                .font(font_id(ui, 14.0, TwoUiFontWeight::Medium)),
                        );
                        ui.label(
                            RichText::new("Regular • 0123456789")
                                .font(font_id(ui, 14.0, TwoUiFontWeight::Regular))
                                .color(muted),
                        );
                    });
                });
        });

        SectionHeader::new("Preference Controls").show(ui);
        CardSection::new().show(ui, |ui| {
            SwitchItem::new(&mut self.wifi_enabled, "Wi-Fi")
                .icon(TwoUiIcon::Wifi)
                .show(ui);
            SwitchItem::new(&mut self.bt_enabled, "Bluetooth")
                .icon(TwoUiIcon::Bluetooth)
                .top_divider(true)
                .show(ui);
            SwitchItem::new(&mut self.notifications_enabled, "Switch row")
                .icon(TwoUiIcon::Bell)
                .top_divider(true)
                .show(ui);
            paint_divider(ui, DividerInset::FullWidth);
            Frame::NONE
                .inner_margin(Margin::symmetric(20, 12))
                .show(ui, |ui| {
                    ui.twoui_checkbox(
                        &mut self.pref_check,
                        "CheckBox option",
                    );
                    ui.add_space(8.0);
                    TwoUiSearchBar::new(&mut self.pref_text)
                        .hint("Search input...")
                        .show(ui);
                    ui.add_space(8.0);
                    let choices = ["#00FFFF Cyan", "#F0FFFF Azure", "#387AFF Default Blue"];
                    TwoUiDropdown::new("pref_list", &choices, &mut self.pref_list_idx).show(ui);
                });
        });

        SectionHeader::new("SeekBar Preferences").show(ui);
        CardSection::new()
            .inner_margin(Margin::symmetric(20, 14))
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 14.0;
                ui.add(
                    TwoUiSeekBarPro::new("Expanded", &mut self.seek_pro_xp, 0.0..=100.0)
                        .mode(SeekBarMode::Expand),
                );
                ui.add(
                    TwoUiSeekBarPro::new("Stepped (2..10)", &mut self.seek_pro_level, 2.0..=10.0)
                        .mode(SeekBarMode::Level { steps: 8 })
                        .seamless(false),
                );
                ui.add(
                    TwoUiSeekBarPro::new("Centered", &mut self.seek_plus_seamless, 0.0..=20.0)
                        .mode(SeekBarMode::Level { steps: 2 })
                        .center_based_ticks(true)
                        .end_labels("Left", "Right"),
                );
            });

        TwoUiPreferenceCaption::new(
            "Caption spacing (8dp top / 16dp bottom, 12sp).",
        )
        .show(ui);
    }
}
