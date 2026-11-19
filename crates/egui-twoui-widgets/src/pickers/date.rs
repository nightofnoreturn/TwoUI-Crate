//! Interactive calendar, week-select, and spinning wheel DatePicker widget.

use egui::{pos2, vec2, Align, Align2, Color32, CornerRadius, FontFamily, FontId, Id, Layout, Rect, Sense, Ui, Vec2};
use egui_twoui_icons::TwoUiIcon;
use egui_twoui_tokens::{interaction_factors, theme_from_ui};

use super::{centered_column, TwoUiNumberPicker};

/// Visual layout for [`TwoUiDatePicker`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DatePickerMode {
    /// Traditional monthly calendar grid.
    #[default]
    Calendar,
    /// Full-week highlight selection band.
    WeekSelect,
    /// Three-column Year/Month/Day spinning wheels.
    Spinner,
}

/// Interactive DatePicker (`Calendar`, `WeekSelect`, or `Spinner` mode).
pub struct TwoUiDatePicker<'a> {
    id_salt: &'a str,
    year: &'a mut i32,
    month: &'a mut u8,
    day: &'a mut u8,
    mode: DatePickerMode,
}

impl<'a> TwoUiDatePicker<'a> {
    pub fn new(id_salt: &'a str, year: &'a mut i32, month: &'a mut u8, day: &'a mut u8) -> Self {
        *month = (*month).clamp(1, 12);
        let max_d = days_in_month(*year, *month);
        *day = (*day).clamp(1, max_d);
        Self {
            id_salt,
            year,
            month,
            day,
            mode: DatePickerMode::Calendar,
        }
    }

    pub fn mode(mut self, mode: DatePickerMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn show(self, ui: &mut Ui) -> bool {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let mut changed = false;

        const MONTHS_LONG: [&str; 12] = [
            "January", "February", "March", "April", "May", "June", "July", "August", "September",
            "October", "November", "December",
        ];
        const MONTHS_SHORT: [&str; 12] = [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ];

        match self.mode {
            DatePickerMode::Spinner => {
                let avail_w = ui.available_width().max(240.0);
                let wide = avail_w >= 380.0;
                let wheel_h = 144.0;
                let spacing = if wide { 16.0 } else { 8.0 };
                let y_w = if wide { 110.0 } else { 92.0 };
                let m_w = if wide { 92.0 } else { 78.0 };
                let d_w = if wide { 92.0 } else { 82.0 };
                let content_w = y_w + spacing + m_w + spacing + d_w;
                centered_column(ui, content_w, |ui| {
                    ui.allocate_ui_with_layout(
                        vec2(content_w, wheel_h),
                        Layout::left_to_right(Align::Center),
                        |ui| {
                            ui.spacing_mut().item_spacing.x = spacing;
                            let mut y = *self.year;
                            let mut m = i32::from(*self.month);
                            let mut d = i32::from(*self.day);

                            if ui
                                .add(
                                    TwoUiNumberPicker::new(self.id_salt, &mut y, 1970, 2099)
                                        .width(y_w)
                                        .wrap_around(false),
                                )
                                .changed()
                            {
                                *self.year = y;
                                changed = true;
                            }
                            if ui
                                .add(
                                    TwoUiNumberPicker::new("dp_m", &mut m, 1, 12)
                                        .width(m_w)
                                        .displayed_labels(&MONTHS_SHORT),
                                )
                                .changed()
                            {
                                *self.month = m as u8;
                                changed = true;
                            }
                            let max_d = i32::from(days_in_month(*self.year, *self.month));
                            d = d.clamp(1, max_d);
                            *self.day = d as u8;
                            if ui
                                .add(
                                    TwoUiNumberPicker::new("dp_d", &mut d, 1, max_d)
                                        .width(d_w)
                                        .min_digits(2),
                                )
                                .changed()
                            {
                                *self.day = d.clamp(1, max_d) as u8;
                                changed = true;
                            }
                        },
                    );
                });
            }
            DatePickerMode::Calendar | DatePickerMode::WeekSelect => {
                let week_mode = matches!(self.mode, DatePickerMode::WeekSelect);
                let cal_w = ui.available_width().clamp(240.0, 520.0);
                let ym_mem_id = Id::unique((self.id_salt, "ym_spin_mode"));
                let mut ym_spin_mode = ui.ctx().data(|d| d.get_temp::<bool>(ym_mem_id)).unwrap_or(false);

                centered_column(ui, cal_w, |ui| {
                    let (hdr_rect, _) =
                        ui.allocate_exact_size(vec2(cal_w, 36.0), Sense::hover());

                    let prev_rect = Rect::from_min_size(hdr_rect.min, vec2(36.0, 36.0));
                    let prev_resp =
                        ui.interact(prev_rect, Id::unique((self.id_salt, "cal_prev")), Sense::click());
                    if prev_resp.clicked() {
                        if *self.month == 1 {
                            *self.month = 12;
                            *self.year -= 1;
                        } else {
                            *self.month -= 1;
                        }
                        let max_d = days_in_month(*self.year, *self.month);
                        *self.day = (*self.day).min(max_d);
                        changed = true;
                    }
                    if ui.is_rect_visible(prev_rect) {
                        let (pt, ht) = interaction_factors(ui, &prev_resp);
                        if ht > 0.01 || pt > 0.01 {
                            ui.painter().circle_filled(
                                prev_rect.center(),
                                16.0,
                                p.hover_overlay
                                    .gamma_multiply(ht)
                                    .blend(p.press_overlay.gamma_multiply(pt)),
                            );
                        }
                        TwoUiIcon::Back.paint(
                            ui,
                            Rect::from_center_size(prev_rect.center(), Vec2::splat(18.0)),
                            p.text_primary,
                        );
                    }

                    let next_rect = Rect::from_min_size(
                        pos2(hdr_rect.right() - 36.0, hdr_rect.top()),
                        vec2(36.0, 36.0),
                    );
                    let next_resp =
                        ui.interact(next_rect, Id::unique((self.id_salt, "cal_next")), Sense::click());
                    if next_resp.clicked() {
                        if *self.month == 12 {
                            *self.month = 1;
                            *self.year += 1;
                        } else {
                            *self.month += 1;
                        }
                        let max_d = days_in_month(*self.year, *self.month);
                        *self.day = (*self.day).min(max_d);
                        changed = true;
                    }
                    if ui.is_rect_visible(next_rect) {
                        let (pt, ht) = interaction_factors(ui, &next_resp);
                        if ht > 0.01 || pt > 0.01 {
                            ui.painter().circle_filled(
                                next_rect.center(),
                                16.0,
                                p.hover_overlay
                                    .gamma_multiply(ht)
                                    .blend(p.press_overlay.gamma_multiply(pt)),
                            );
                        }
                        TwoUiIcon::ChevronRight.paint(
                            ui,
                            Rect::from_center_size(next_rect.center(), Vec2::splat(18.0)),
                            p.text_primary,
                        );
                    }

                    let header_label = format!(
                        "{} {}",
                        MONTHS_LONG[(*self.month - 1) as usize],
                        *self.year
                    );
                    let hdr_galley = ui.painter().layout_no_wrap(
                        header_label,
                        FontId::new(17.0, FontFamily::Proportional),
                        p.text_primary,
                    );
                    let pill_w = (hdr_galley.size().x + 34.0).min(cal_w - 84.0);
                    let pill_rect =
                        Rect::from_center_size(hdr_rect.center(), vec2(pill_w, 32.0));
                    let ym_resp =
                        ui.interact(pill_rect, Id::unique((self.id_salt, "cal_ym_hdr")), Sense::click());
                    if ym_resp.clicked() {
                        ym_spin_mode = !ym_spin_mode;
                        ui.ctx().data_mut(|d| d.insert_temp(ym_mem_id, ym_spin_mode));
                    }
                    if ui.is_rect_visible(pill_rect) {
                        let (pt, ht) = interaction_factors(ui, &ym_resp);
                        if ht > 0.01 || pt > 0.01 || ym_spin_mode {
                            let bg = if ym_spin_mode {
                                p.selected_bg
                            } else {
                                p.hover_overlay
                                    .gamma_multiply(ht)
                                    .blend(p.press_overlay.gamma_multiply(pt))
                            };
                            ui.painter()
                                .rect_filled(pill_rect, CornerRadius::same(16), bg);
                        }
                        let text_x = pill_rect.center().x - (hdr_galley.size().x + 18.0) * 0.5;
                        let text_right = text_x + hdr_galley.size().x;
                        ui.painter().galley(
                            pos2(text_x, pill_rect.center().y - hdr_galley.size().y * 0.5),
                            hdr_galley,
                            p.text_primary,
                        );
                        let caret_icon = if ym_spin_mode {
                            TwoUiIcon::ChevronUp
                        } else {
                            TwoUiIcon::ChevronDown
                        };
                        caret_icon.paint(
                            ui,
                            Rect::from_center_size(
                                pos2(text_right + 10.0, pill_rect.center().y),
                                Vec2::splat(14.0),
                            ),
                            p.text_secondary,
                        );
                    }

                    ui.add_space(8.0);
                    if ym_spin_mode {
                        let wheel_w = (104.0_f32 + 12.0 + 96.0).min(cal_w);
                        centered_column(ui, wheel_w, |ui| {
                            ui.allocate_ui_with_layout(
                                vec2(wheel_w, 144.0),
                                Layout::left_to_right(Align::Center),
                                |ui| {
                                    ui.spacing_mut().item_spacing.x = 12.0;
                                    let mut m = i32::from(*self.month);
                                    let mut y = *self.year;
                                    if ui
                                        .add(
                                            TwoUiNumberPicker::new("dp_ym_m", &mut m, 1, 12)
                                                .width(96.0)
                                                .displayed_labels(&MONTHS_SHORT),
                                        )
                                        .changed()
                                    {
                                        *self.month = m as u8;
                                        let max_d = days_in_month(*self.year, *self.month);
                                        *self.day = (*self.day).min(max_d);
                                        changed = true;
                                    }
                                    if ui
                                        .add(
                                            TwoUiNumberPicker::new("dp_ym_y", &mut y, 1970, 2099)
                                                .width(104.0)
                                                .wrap_around(false),
                                        )
                                        .changed()
                                    {
                                        *self.year = y;
                                        let max_d = days_in_month(*self.year, *self.month);
                                        *self.day = (*self.day).min(max_d);
                                        changed = true;
                                    }
                                },
                            );
                        });
                    } else {
                        let cell_w = cal_w / 7.0;
                        let weekdays = ["S", "M", "T", "W", "T", "F", "S"];
                        let (wk_rect, _) =
                            ui.allocate_exact_size(vec2(cal_w, 24.0), Sense::hover());
                        for (i, w) in weekdays.iter().enumerate() {
                            let col = if i == 0 {
                                Color32::from_rgb(0xD9, 0x3E, 0x3A)
                            } else if i == 6 {
                                p.primary_high
                            } else {
                                p.text_muted
                            };
                            ui.painter().text(
                                pos2(wk_rect.left() + (i as f32 + 0.5) * cell_w, wk_rect.center().y),
                                Align2::CENTER_CENTER,
                                *w,
                                FontId::new(11.5, FontFamily::Proportional),
                                col,
                            );
                        }

                        ui.add_space(4.0);
                        let first_dow = weekday_sunday0(*self.year, *self.month, 1) as usize;
                        let total_days = days_in_month(*self.year, *self.month);
                        let rows = (first_dow + usize::from(total_days)).div_ceil(7);
                        let cell_h = (cell_w * 0.56).clamp(32.0, 42.0);
                        let circle_r = (cell_h * 0.43).clamp(13.0, 17.5);

                        let selected_row = (first_dow + usize::from(*self.day) - 1) / 7;
                        for row in 0..rows {
                            let (row_rect, _) =
                                ui.allocate_exact_size(vec2(cal_w, cell_h), Sense::hover());
                            if week_mode && row == selected_row {
                                ui.painter().rect_filled(
                                    row_rect.shrink2(vec2(2.0, 2.0)),
                                    CornerRadius::same((cell_h * 0.44).round() as u8),
                                    p.primary.gamma_multiply(0.18),
                                );
                            }
                            for col_idx in 0..7 {
                                let cell_idx = row * 7 + col_idx;
                                if cell_idx < first_dow {
                                    continue;
                                }
                                let day_num = (cell_idx - first_dow + 1) as u8;
                                if day_num > total_days {
                                    continue;
                                }
                                let c_rect = Rect::from_min_size(
                                    pos2(row_rect.left() + col_idx as f32 * cell_w, row_rect.top()),
                                    vec2(cell_w, cell_h),
                                );
                                let resp = ui.interact(
                                    c_rect,
                                    Id::unique((self.id_salt, row, col_idx)),
                                    Sense::click(),
                                );
                                if resp.clicked() {
                                    *self.day = day_num;
                                    changed = true;
                                }
                                let is_sel = *self.day == day_num;
                                if is_sel {
                                    ui.painter().circle_filled(c_rect.center(), circle_r, p.primary);
                                } else if resp.hovered() {
                                    ui.painter().circle_filled(
                                        c_rect.center(),
                                        circle_r,
                                        p.hover_overlay,
                                    );
                                }
                                let fg = if is_sel {
                                    p.on_primary
                                } else if week_mode && row == selected_row {
                                    p.primary_high
                                } else if col_idx == 0 {
                                    Color32::from_rgb(0xD9, 0x3E, 0x3A)
                                } else if col_idx == 6 {
                                    p.primary_high
                                } else {
                                    p.text_primary
                                };
                                ui.painter().text(
                                    c_rect.center(),
                                    Align2::CENTER_CENTER,
                                    day_num.to_string(),
                                    FontId::new(14.5, FontFamily::Proportional),
                                    fg,
                                );
                            }
                        }
                    }
                });
            }
        }

        changed
    }
}

/// Returns `true` if `year` is a Gregorian leap year.
pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Returns the number of days (`28..=31`) in `(year, month)`.
pub fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

/// Returns weekday (`0 = Sunday .. 6 = Saturday`) via Tomohiko Sakamoto's algorithm.
pub fn weekday_sunday0(mut year: i32, month: u8, day: u8) -> u8 {
    const T: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let m = usize::from(month.clamp(1, 12));
    if m < 3 {
        year -= 1;
    }
    ((year + year / 4 - year / 100 + year / 400 + T[m - 1] + i32::from(day)).rem_euclid(7)) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leap_year_and_weekday_calculations_match_calendar() {
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(2025));
        assert!(!is_leap_year(1900));
        assert!(is_leap_year(2000));
        assert_eq!(days_in_month(2024, 2), 29);
        assert_eq!(days_in_month(2025, 2), 28);
        assert_eq!(weekday_sunday0(2026, 1, 1), 4);
    }
}
