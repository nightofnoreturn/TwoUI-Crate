//! QRCode live encoder and QrImageView variants showcase page.

use egui::{vec2, Color32, Margin, Ui};
use egui_twoui::prelude::*;

use crate::app::ShowcaseApp;

impl ShowcaseApp {
    pub(crate) fn render_qr_page(&mut self, ui: &mut Ui) {
        SectionHeader::new("Live QR Encoder").show(ui);
        CardSection::new()
            .inner_margin(Margin::symmetric(20, 14))
            .show(ui, |ui| {
                let has_text = !self.qr_text.is_empty();
                let (_, back) = TwoUiSearchBar::new(&mut self.qr_text)
                    .hint("Type text or URL for QRCode...")
                    .show_back_button(has_text)
                    .show(ui);
                if back {
                    self.qr_text.clear();
                }
            });

        SectionHeader::new("QRImageView Variations").show(ui);
        CardSection::new()
            .inner_margin(Margin::symmetric(20, 20))
            .show(ui, |ui| {
                let text = if self.qr_text.is_empty() {
                    "https://github.com/nightofnoreturn/TwoUI-Crate"
                } else {
                    self.qr_text.as_str()
                };
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = vec2(20.0, 20.0);
                    ui.vertical(|ui| {
                        ui.add(
                            TwoUiQrCode::new(text)
                                .size(176.0)
                                .icon(TwoUiIcon::Apps)
                                .rounded_frame(true),
                        );
                        ui.label("Rounded + Center Badge");
                    });
                    ui.vertical(|ui| {
                        ui.add(
                            TwoUiQrCode::new("https://github.com/emilk/egui")
                                .size(176.0)
                                .rounded_frame(false),
                        );
                        ui.label("Sharp Frame");
                    });
                    ui.vertical(|ui| {
                        ui.add(
                            TwoUiQrCode::new("https://www.rust-lang.org")
                                .size(176.0)
                                .foreground_color(Color32::from_rgb(0x66, 0x69, 0x99))
                                .tint_anchor(true)
                                .tint_frame(true)
                                .rounded_frame(true)
                                .icon(TwoUiIcon::QrCode),
                        );
                        ui.label("Tinted Frame & Anchors");
                    });
                });
            });
    }
}
