//! About TwoUI-Crate & AppInfo showcase page.

use egui::{vec2, Margin, RichText, Ui};
use egui_twoui::prelude::*;

use crate::app::ShowcaseApp;

impl ShowcaseApp {
    pub(crate) fn render_app_info_page(&mut self, ui: &mut Ui) {
        let now = ui.input(|i| i.time);
        if let Some(until) = self.update_check_until {
            if now < until {
                ui.ctx().request_repaint();
            } else {
                self.update_check_until = None;
                self.update_state = AppUpdateState::LatestInstalled;
                self.toast
                    .show_message(ui.ctx(), "Latest version installed!", 2.0);
            }
        }

        ui.horizontal(|ui| {
            ui.twoui_switch(&mut self.custom_about_style, "Custom About style");
        });

        if self.custom_about_style {
            ui.add_space(12.0);
            CardSection::new()
                .inner_margin(Margin::symmetric(24, 28))
                .show(ui, |ui| {
                    let p = theme_from_ui(ui).palette;
                    ui.vertical_centered(|ui| {
                        TwoUiIcon::Apps.show(ui, 56.0, p.primary);
                        ui.add_space(12.0);
                        ui.label(
                            RichText::new("TwoUI-Crate")
                                .size(28.0)
                                .strong()
                                .color(p.text_primary),
                        );
                        ui.label(
                            RichText::new(concat!("Custom About • Version ", env!("CARGO_PKG_VERSION")))
                                .size(13.5)
                                .color(p.text_muted),
                        );
                        ui.add_space(14.0);
                        ui.horizontal_wrapped(|ui| {
                            ui.spacing_mut().item_spacing = vec2(10.0, 8.0);
                            if ui.add(TwoUiButton::primary("Open Source Licenses")).clicked() {
                                self.licenses_sheet_open = true;
                            }
                            if ui.add(TwoUiButton::outline("Back to AppInfo")).clicked() {
                                self.custom_about_style = false;
                            }
                        });
                    });
                });
            ui.add_space(14.0);
            let links = ["GitHub Repository", "Documentation", "Licenses"];
            if let Some(idx) = RelativeLinksCard::new(&links).show(ui) {
                if idx == 2 {
                    self.licenses_sheet_open = true;
                } else {
                    self.toast
                        .show_message(ui.ctx(), "github.com/nightofnoreturn/TwoUI-Crate", 2.0);
                }
            }
        } else {
            let (update_clicked, link_clicked) =
                AppInfoView::new("TwoUI-Crate", env!("CARGO_PKG_VERSION"))
                .update_state(self.update_state)
                .links(&["Open source licenses", "Repository on GitHub"])
                .show(ui);

            if update_clicked {
                self.update_state = AppUpdateState::Checking;
                self.update_check_until = Some(now + 1.8);
            }
            if let Some(idx) = link_clicked {
                if idx == 0 {
                    self.licenses_sheet_open = true;
                } else {
                    self.toast
                        .show_message(ui.ctx(), "github.com/nightofnoreturn/TwoUI-Crate", 2.4);
                }
            }
        }
    }
}
