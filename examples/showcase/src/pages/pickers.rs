//! Number, Time, Date, SleepTime, and Color pickers showcase page.

use egui::{vec2, Align, Layout, Margin, Ui};
use egui_twoui::prelude::*;

use crate::app::ShowcaseApp;

impl ShowcaseApp {
    pub(crate) fn render_pickers_page(&mut self, ui: &mut Ui) {
        let modes = [
            "NumberPicker",
            "TimePicker",
            "DatePicker (Calendar)",
            "DatePicker (Week)",
            "SpinningDatePicker",
            "SleepTimePicker",
            "ColorPicker",
        ];
        ui.horizontal_wrapped(|ui| {
            TwoUiDropdown::new("picker_mode_sel", &modes, &mut self.picker_type_idx).show(ui);
            if self.picker_type_idx == 1 || self.picker_type_idx == 5 {
                ui.add_space(10.0);
                ui.twoui_checkbox(&mut self.time_24h, "24-hour format");
            } else if matches!(self.picker_type_idx, 2..=4) {
                ui.add_space(10.0);
                let mut is_spin = self.date_spinner_mode || self.picker_type_idx == 4;
                if ui.twoui_checkbox(&mut is_spin, "Spinning wheel mode").changed() {
                    self.date_spinner_mode = is_spin;
                    self.picker_type_idx = if is_spin { 4 } else { 2 };
                }
            }
        });
        ui.add_space(8.0);

        CardSection::new()
            .inner_margin(Margin::symmetric(20, 20))
            .show(ui, |ui| {
                ui.vertical_centered(|ui| match self.picker_type_idx {
                    0 => {
                        let avail_w = ui.available_width().max(260.0);
                        let wide = avail_w >= 380.0;
                        let wheel_w: f32 = if wide { 104.0 } else { 82.0 };
                        let gap: f32 = if wide { 18.0 } else { 10.0 };
                        let row_w = wheel_w * 3.0 + gap * 2.0;
                        let pad = ((avail_w - row_w) * 0.5).max(0.0);
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;
                            if pad > 0.0 {
                                ui.add_space(pad);
                            }
                            ui.allocate_ui_with_layout(
                                vec2(row_w, 144.0),
                                Layout::left_to_right(Align::Center),
                                |ui| {
                                    ui.spacing_mut().item_spacing.x = gap;
                                    ui.add(
                                        TwoUiNumberPicker::new("np_1", &mut self.num_p1, 0, 99)
                                            .width(wheel_w)
                                            .min_digits(2),
                                    );
                                    ui.add(
                                        TwoUiNumberPicker::new("np_2", &mut self.num_p2, 1, 12)
                                            .width(wheel_w),
                                    );
                                    ui.add(
                                        TwoUiNumberPicker::new("np_3", &mut self.num_p3, 1990, 2040)
                                            .width(wheel_w)
                                            .wrap_around(false),
                                    );
                                },
                            );
                            if pad > 0.0 {
                                ui.add_space(pad);
                            }
                        });
                        ui.add_space(8.0);
                        ui.label(format!(
                            "Selected numbers: {:02} / {} / {}",
                            self.num_p1, self.num_p2, self.num_p3
                        ));
                    }
                    1 => {
                        TwoUiTimePicker::new("tp_main", &mut self.time_hour, &mut self.time_min)
                            .is_24_hour(self.time_24h)
                            .show(ui);
                        ui.add_space(8.0);
                        ui.label(format!(
                            "Selected time: {:02}:{:02}",
                            self.time_hour, self.time_min
                        ));
                    }
                    2 => {
                        TwoUiDatePicker::new(
                            "dp_cal",
                            &mut self.date_year,
                            &mut self.date_month,
                            &mut self.date_day,
                        )
                        .mode(DatePickerMode::Calendar)
                        .show(ui);
                        ui.add_space(8.0);
                        ui.label(format!(
                            "Selected date: {:04}-{:02}-{:02}",
                            self.date_year, self.date_month, self.date_day
                        ));
                    }
                    3 => {
                        TwoUiDatePicker::new(
                            "dp_week",
                            &mut self.date_year,
                            &mut self.date_month,
                            &mut self.date_day,
                        )
                        .mode(DatePickerMode::WeekSelect)
                        .show(ui);
                        ui.add_space(8.0);
                        ui.label(format!(
                            "Selected week anchor: {:04}-{:02}-{:02}",
                            self.date_year, self.date_month, self.date_day
                        ));
                    }
                    4 => {
                        TwoUiDatePicker::new(
                            "dp_spin",
                            &mut self.date_year,
                            &mut self.date_month,
                            &mut self.date_day,
                        )
                        .mode(DatePickerMode::Spinner)
                        .show(ui);
                        ui.add_space(8.0);
                        ui.label(format!(
                            "Selected date: {:04}-{:02}-{:02}",
                            self.date_year, self.date_month, self.date_day
                        ));
                    }
                    5 => {
                        TwoUiSleepTimePicker::new(
                            "sleep_inline",
                            &mut self.bedtime_min,
                            &mut self.wakeup_min,
                        )
                        .is_24_hour(self.time_24h)
                        .show(ui);
                    }
                    _ => {
                        TwoUiColorPicker::new("cp_inline", &mut self.inline_picker_color).show(ui);
                    }
                });
            });

        SectionHeader::new("Dialogs").show(ui);
        CardSection::new()
            .inner_margin(Margin::symmetric(20, 20))
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.spacing_mut().item_spacing.y = 10.0;
                    if ui
                        .add(
                            TwoUiButton::new("Date")
                                .style(TwoUiButtonStyle::Contained)
                                .min_size(vec2(250.0, 44.0)),
                        )
                        .clicked()
                    {
                        self.date_dialog_open = true;
                    }
                    if ui
                        .add(
                            TwoUiButton::new("Time")
                                .style(TwoUiButtonStyle::Contained)
                                .min_size(vec2(250.0, 44.0)),
                        )
                        .clicked()
                    {
                        self.time_dialog_open = true;
                    }
                    if ui
                        .add(
                            TwoUiButton::new("Start End Time")
                                .style(TwoUiButtonStyle::Contained)
                                .min_size(vec2(250.0, 44.0)),
                        )
                        .clicked()
                    {
                        self.sleep_dialog_open = true;
                    }
                    if ui
                        .add(
                            TwoUiButton::new("Color")
                                .style(TwoUiButtonStyle::Contained)
                                .min_size(vec2(250.0, 44.0)),
                        )
                        .clicked()
                    {
                        self.color_dialog_draft = self.custom_accent;
                        self.color_dialog_open = true;
                    }
                });
            });
    }
}
