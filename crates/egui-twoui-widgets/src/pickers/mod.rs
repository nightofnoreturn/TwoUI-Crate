//! Interactive picker widgets: NumberPicker, TimePicker, DatePicker, SleepTimePicker, and ColorPicker.

mod color;
mod date;
mod number;
mod sleep;
mod time;

pub use color::{color32_to_hsva, hsva_to_color32, TwoUiColorPicker};
pub use date::{days_in_month, is_leap_year, weekday_sunday0, DatePickerMode, TwoUiDatePicker};
pub use number::TwoUiNumberPicker;
pub use sleep::TwoUiSleepTimePicker;
pub use time::TwoUiTimePicker;

use egui::Ui;

pub(crate) fn centered_column<R>(
    ui: &mut Ui,
    desired_w: f32,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> R {
    let orig_spacing_x = ui.spacing().item_spacing.x;
    let avail_w = ui.available_width().max(1.0);
    let w = desired_w.min(avail_w);
    let pad = ((avail_w - w) * 0.5).max(0.0);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        if pad > 0.0 {
            ui.add_space(pad);
        }
        let inner = ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.x = orig_spacing_x;
            ui.set_width(w);
            ui.set_min_width(w);
            add_contents(ui)
        });
        if pad > 0.0 {
            ui.add_space(pad);
        }
        inner.inner
    })
    .inner
}
