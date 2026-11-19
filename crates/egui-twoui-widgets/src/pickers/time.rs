//! Vertical spinning-wheel time picker (12h AM/PM or 24h).

use egui::{pos2, vec2, Align, FontFamily, FontId, Layout, Sense, Ui};
use egui_twoui_tokens::theme_from_ui;

use super::{centered_column, TwoUiNumberPicker};

/// Multi-wheel TimePicker (hour / minute / optional AM-PM).
pub struct TwoUiTimePicker<'a> {
    id_salt: &'a str,
    hour_24: &'a mut u8,
    minute: &'a mut u8,
    is_24_hour: bool,
}

impl<'a> TwoUiTimePicker<'a> {
    pub fn new(id_salt: &'a str, hour_24: &'a mut u8, minute: &'a mut u8) -> Self {
        *hour_24 %= 24;
        *minute %= 60;
        Self {
            id_salt,
            hour_24,
            minute,
            is_24_hour: false,
        }
    }

    pub fn is_24_hour(mut self, enabled: bool) -> Self {
        self.is_24_hour = enabled;
        self
    }

    pub fn show(self, ui: &mut Ui) -> bool {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let mut changed = false;

        let avail_w = ui.available_width().max(220.0);
        let wide = avail_w >= 380.0;
        let num_w = if wide { 104.0 } else { 82.0 };
        let ampm_w = if wide { 80.0 } else { 66.0 };
        let spacing = if wide { 16.0 } else { 8.0 };
        let wheel_h = 144.0;
        let colon_g = ui.painter().layout_no_wrap(
            ":".to_owned(),
            FontId::new(28.0, FontFamily::Proportional),
            p.text_primary,
        );
        let colon_w = colon_g.size().x.max(8.0);
        let content_w = if self.is_24_hour {
            num_w + spacing + colon_w + spacing + num_w
        } else {
            num_w + spacing + colon_w + spacing + num_w + spacing + ampm_w
        };

        centered_column(ui, content_w, |ui| {
            ui.allocate_ui_with_layout(
                vec2(content_w, wheel_h),
                Layout::left_to_right(Align::Center),
                |ui| {
                    ui.spacing_mut().item_spacing.x = spacing;
                    if self.is_24_hour {
                        let mut h = i32::from(*self.hour_24);
                        let mut m = i32::from(*self.minute);
                        if ui
                            .add(
                                TwoUiNumberPicker::new(self.id_salt, &mut h, 0, 23)
                                    .width(num_w)
                                    .min_digits(2),
                            )
                            .changed()
                        {
                            *self.hour_24 = h.rem_euclid(24) as u8;
                            changed = true;
                        }
                        let (cr, _) = ui.allocate_exact_size(vec2(colon_w, wheel_h), Sense::hover());
                        ui.painter().galley(
                            pos2(cr.left(), cr.center().y - colon_g.size().y * 0.5),
                            colon_g,
                            p.text_primary,
                        );
                        if ui
                            .add(
                                TwoUiNumberPicker::new("tp_min", &mut m, 0, 59)
                                    .width(num_w)
                                    .min_digits(2),
                            )
                            .changed()
                        {
                            *self.minute = m.rem_euclid(60) as u8;
                            changed = true;
                        }
                    } else {
                        let is_pm = *self.hour_24 >= 12;
                        let mut h12 = match *self.hour_24 % 12 {
                            0 => 12,
                            v => i32::from(v),
                        };
                        let mut m = i32::from(*self.minute);
                        let mut ampm_idx = if is_pm { 1 } else { 0 };

                        if ui
                            .add(
                                TwoUiNumberPicker::new(self.id_salt, &mut h12, 1, 12)
                                    .width(num_w)
                                    .min_digits(2),
                            )
                            .changed()
                        {
                            let base = if ampm_idx == 1 { 12 } else { 0 };
                            *self.hour_24 = (base + (h12 % 12)) as u8;
                            changed = true;
                        }
                        let (cr, _) = ui.allocate_exact_size(vec2(colon_w, wheel_h), Sense::hover());
                        ui.painter().galley(
                            pos2(cr.left(), cr.center().y - colon_g.size().y * 0.5),
                            colon_g,
                            p.text_primary,
                        );
                        if ui
                            .add(
                                TwoUiNumberPicker::new("tp_min", &mut m, 0, 59)
                                    .width(num_w)
                                    .min_digits(2),
                            )
                            .changed()
                        {
                            *self.minute = m.rem_euclid(60) as u8;
                            changed = true;
                        }
                        if ui
                            .add(
                                TwoUiNumberPicker::new("tp_ampm", &mut ampm_idx, 0, 1)
                                    .width(ampm_w)
                                    .text_size(21.0)
                                    .wrap_around(false)
                                    .displayed_labels(&["AM", "PM"]),
                            )
                            .changed()
                        {
                            let hour12_zero = *self.hour_24 % 12;
                            *self.hour_24 = hour12_zero + if ampm_idx == 1 { 12 } else { 0 };
                            changed = true;
                        }
                    }
                },
            );
        });

        changed
    }
}
