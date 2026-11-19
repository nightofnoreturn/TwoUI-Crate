//! Switches, buttons, chips, search bars, list items, and overlay triggers showcase page.

use egui::{vec2, Margin, Ui};
use egui_twoui::prelude::*;

use crate::{app::ShowcaseApp, data::Page};

impl ShowcaseApp {
    pub(crate) fn render_widgets_page(&mut self, ui: &mut Ui) {
        let now = ui.input(|i| i.time);
        let is_busy = self.master_busy_until.is_some_and(|t| now < t);
        if is_busy {
            ui.ctx().request_repaint();
        } else {
            self.master_busy_until = None;
        }

        if ui
            .add(TwoUiSwitchBar::new(&mut self.master_switch).show_progress(is_busy))
            .changed()
        {
            self.master_busy_until = Some(now + 1.5);
            self.toast.show_message(
                ui.ctx(),
                if self.master_switch {
                    "Master switch On"
                } else {
                    "Master switch Off"
                },
                2.0,
            );
        }

        SectionHeader::new("Compound Buttons").show(ui);
        CardSection::new()
            .inner_margin(Margin::symmetric(20, 14))
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 10.0;
                ui.twoui_switch(&mut self.compound_switch, "Switch");
                ui.twoui_checkbox(&mut self.compound_check, "CheckBox");
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 24.0;
                    if ui.twoui_radio(self.compound_radio == 0, "Radio 1").clicked() {
                        self.compound_radio = 0;
                    }
                    if ui.twoui_radio(self.compound_radio == 1, "Radio 2").clicked() {
                        self.compound_radio = 1;
                    }
                });
            });

        SectionHeader::new("Buttons & ProgressButton").show(ui);
        CardSection::new()
            .inner_margin(Margin::symmetric(20, 14))
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 10.0;
                let btn_busy = self.progress_btn_busy_until.is_some_and(|t| now < t);
                if btn_busy {
                    ui.ctx().request_repaint();
                } else {
                    self.progress_btn_busy_until = None;
                }
                ui.vertical_centered(|ui| {
                    for (label, style) in [
                        ("Default Button", TwoUiButtonStyle::Default),
                        ("Colored Primary Button", TwoUiButtonStyle::Primary),
                        ("Transparent Button", TwoUiButtonStyle::Transparent),
                        ("Outline Button", TwoUiButtonStyle::Outline),
                        ("Contained Button", TwoUiButtonStyle::Contained),
                        ("Contained Primary CTA", TwoUiButtonStyle::ContainedPrimary),
                        ("Contained Transparent", TwoUiButtonStyle::ContainedTransparent),
                    ] {
                        if ui.add(TwoUiButton::new(label).style(style)).clicked() {
                            self.toast
                                .show_message(ui.ctx(), format!("{label} clicked"), 1.8);
                        }
                    }
                    if ui
                        .add(TwoUiButton::contained_primary("ProgressButton").loading(btn_busy))
                        .clicked()
                    {
                        self.progress_btn_busy_until = Some(now + 2.0);
                    }
                });
            });

        SectionHeader::new("Chips, Dropdown & SearchBar").show(ui);
        CardSection::new()
            .inner_margin(Margin::symmetric(16, 14))
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 12.0;
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = vec2(8.0, 8.0);
                    let chip_labels = [
                        ("Trending", Some(TwoUiIcon::Star)),
                        ("Bluetooth", Some(TwoUiIcon::Bluetooth)),
                        ("Alerts", Some(TwoUiIcon::Bell)),
                        ("Recent", None),
                    ];
                    for (idx, (lbl, ic)) in chip_labels.into_iter().enumerate() {
                        let mut chip = TwoUiChip::new(lbl).selected(self.chip_selected[idx]);
                        if let Some(icon) = ic {
                            chip = chip.icon(icon);
                        }
                        if ui.add(chip).clicked() {
                            self.chip_selected[idx] = !self.chip_selected[idx];
                        }
                    }
                });

                let entries = ["Item 1", "Item 2", "Item 3", "Item 4"];
                if TwoUiDropdown::new("widgets_spinner", &entries, &mut self.dropdown_idx)
                    .show(ui)
                    .changed()
                {
                    self.toast.show_message(
                        ui.ctx(),
                        format!("Selected {}", entries[self.dropdown_idx]),
                        1.8,
                    );
                }
                let has_q = !self.search_query.is_empty();
                let (_, back_clicked) = TwoUiSearchBar::new(&mut self.search_query)
                    .hint("Standard search bar...")
                    .show_back_button(has_q)
                    .show(ui);
                if back_clicked {
                    self.search_query.clear();
                }

                let has_fq = !self.floating_search_query.is_empty();
                let (_, fback) = TwoUiSearchBar::new(&mut self.floating_search_query)
                    .hint("Floating search pill...")
                    .show_back_button(has_fq)
                    .floating(true)
                    .show(ui);
                if fback {
                    self.floating_search_query.clear();
                }
            });

        SectionHeader::new("Custom Item Views").show(ui);
        CardSection::new().show(ui, |ui| {
            ui.add(CardItem::new("CardItemView"));
            ui.add(
                CardItem::new("CardItemView with Chevron")
                    .summary("Subtext description")
                    .badge("N")
                    .chevron()
                    .top_divider(true),
            );
            SwitchItem::new(&mut self.separate_switch_val, "Separate SwitchItemView")
                .summary_states("Tap body or switch independently", "Off")
                .separate_switch(true)
                .user_updatable_summary(true)
                .top_divider(true)
                .show(ui);
            ui.add(
                CardItem::new("CardItemView with Icon")
                    .icon(TwoUiIcon::Star)
                    .icon_tint(theme_from_ui(ui).palette.primary_high)
                    .trailing_text("Detail")
                    .full_width_divider(true)
                    .top_divider(true),
            );
            SwitchItem::new(&mut self.icon_switch_val, "SwitchItemView with Icon")
                .icon(TwoUiIcon::Bell)
                .summary_states("Notifications active", "Muted")
                .user_updatable_summary(true)
                .top_divider(true)
                .show(ui);
            for i in 0..3 {
                if ui
                    .add(
                        RadioItem::new(
                            self.card_radio_idx == i,
                            format!("RadioItemView {}", i + 1),
                        )
                        .top_divider(true),
                    )
                    .clicked()
                {
                    self.card_radio_idx = i;
                }
            }
        });

        SectionHeader::new("Dialogs, SuggestiveDialog, Sheets & SnackBar").show(ui);
        CardSection::new()
            .inner_margin(Margin::symmetric(20, 16))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = vec2(10.0, 10.0);
                    if ui.add(TwoUiButton::primary("Show Dialog")).clicked() {
                        self.dialog_open = true;
                    }
                    if ui.add(TwoUiButton::primary("Show SuggestiveDialog")).clicked() {
                        self.suggestive_dialog_open = true;
                    }
                    if ui.add(TwoUiButton::new("Show BottomSheet")).clicked() {
                        self.bottom_sheet_open = true;
                    }
                    let tip_btn = ui.add(TwoUiButton::new("Show TipPopup"));
                    if tip_btn.clicked() {
                        self.tip_anchor = tip_btn.rect.center_top();
                        self.tip_popup_open = !self.tip_popup_open;
                    }
                    if ui.add(TwoUiButton::new("Show Toast")).clicked() {
                        self.toast
                            .show_message(ui.ctx(), "TwoUI Toast!", 2.2);
                    }
                    if ui.add(TwoUiButton::outline("Suggestive SnackBar")).clicked() {
                        self.snackbar
                            .show_message(ui.ctx(), "Changes saved to workspace", Some("Undo"));
                    }
                });
            });

        ui.add_space(14.0);
        let links = ["Pickers Demo", "QRCode Studio", "Preferences"];
        if let Some(idx) = RelativeLinksCard::new(&links).show(ui) {
            self.page_id = match idx {
                0 => Page::Pickers.id(),
                1 => Page::QrCode.id(),
                _ => Page::Preferences.id(),
            };
        }

        if TwoUiPreferenceCaption::new(
            "Built with TwoUI-Crate.",
        )
        .link("About TwoUI-Crate")
        .show(ui)
        {
            self.page_id = Page::AppInfo.id();
        }
        ui.add_space(88.0);
    }
}
