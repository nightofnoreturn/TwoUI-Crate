use std::ops::RangeInclusive;

use egui::{pos2, vec2, Align2, FontFamily, FontId, Response, Ui, Vec2, Widget};
use egui_twoui_icons::TwoUiIcon;
use egui_twoui_tokens::theme_from_ui;

use super::{SeekBarMode, TwoUiSeekBar};
use crate::button::TwoUiIconButton;

/// Center-anchored bidirectional SeekBar with optional stepped snap and 3 tick dots.
pub struct TwoUiSeekBarPlus<'a> {
    value: &'a mut f32,
    range: RangeInclusive<f32>,
    steps: u32,
    seamless: bool,
    center_based_bar: bool,
    enabled: bool,
}

impl<'a> TwoUiSeekBarPlus<'a> {
    pub fn new(value: &'a mut f32, range: RangeInclusive<f32>) -> Self {
        Self {
            value,
            range,
            steps: 2,
            seamless: true,
            center_based_bar: true,
            enabled: true,
        }
    }

    pub fn seamless(mut self, seamless: bool) -> Self {
        self.seamless = seamless;
        self
    }

    pub fn center_based_bar(mut self, center_based: bool) -> Self {
        self.center_based_bar = center_based;
        self
    }

    pub fn steps(mut self, steps: u32) -> Self {
        self.steps = steps.max(1);
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl Widget for TwoUiSeekBarPlus<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.add(
            TwoUiSeekBar::new(self.value, self.range)
                .mode(SeekBarMode::Level { steps: self.steps })
                .seamless(self.seamless)
                .center_based_ticks(self.center_based_bar)
                .enabled(self.enabled),
        )
    }
}

/// Labeled preference SeekBar row: title, centered value label, optional end labels, and `-` / `+` steppers.
pub struct TwoUiSeekBarPro<'a> {
    value: &'a mut f32,
    range: RangeInclusive<f32>,
    step: f32,
    title: String,
    unit: String,
    left_label: Option<String>,
    right_label: Option<String>,
    mode: Option<SeekBarMode>,
    center_based_ticks: bool,
    seamless: bool,
}

impl<'a> TwoUiSeekBarPro<'a> {
    pub fn new(
        title: impl Into<String>,
        value: &'a mut f32,
        range: RangeInclusive<f32>,
    ) -> Self {
        Self {
            value,
            range,
            step: 1.0,
            title: title.into(),
            unit: String::new(),
            left_label: None,
            right_label: None,
            mode: None,
            center_based_ticks: false,
            seamless: false,
        }
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = step.max(0.01);
        self
    }

    pub fn unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = unit.into();
        self
    }

    pub fn end_labels(mut self, left: impl Into<String>, right: impl Into<String>) -> Self {
        self.left_label = Some(left.into());
        self.right_label = Some(right.into());
        self
    }

    pub fn mode(mut self, mode: SeekBarMode) -> Self {
        self.mode = Some(mode);
        self
    }

    pub fn center_based_ticks(mut self, enabled: bool) -> Self {
        self.center_based_ticks = enabled;
        self
    }

    pub fn seamless(mut self, seamless: bool) -> Self {
        self.seamless = seamless;
        self
    }
}

impl Widget for TwoUiSeekBarPro<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let start = *self.range.start();
        let end = *self.range.end();
        let resolved_mode = self.mode.unwrap_or_else(|| {
            let raw = ((end - start).abs() / self.step).round();
            let steps = raw.clamp(1.0, 20.0) as u32;
            SeekBarMode::Level { steps }
        });
        let mut changed = false;

        let inner = ui.vertical(|ui| {
            ui.label(
                egui::RichText::new(&self.title)
                    .size(16.0)
                    .color(p.text_primary),
            );

            ui.add_space(6.0);

            let display = if (self.step - self.step.round()).abs() < 0.001 {
                format!("{:.0}{}", *self.value, self.unit)
            } else {
                format!("{:.1}{}", *self.value, self.unit)
            };
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new(display)
                        .size(14.0)
                        .color(p.primary_high),
                );
            });

            if self.left_label.is_some() || self.right_label.is_some() {
                ui.add_space(4.0);
                let label_rect = ui.allocate_space(vec2(ui.available_width(), 18.0)).1;
                if let Some(left) = &self.left_label {
                    ui.painter().text(
                        pos2(label_rect.left(), label_rect.center().y),
                        Align2::LEFT_CENTER,
                        left,
                        FontId::new(14.0, FontFamily::Proportional),
                        p.text_secondary,
                    );
                }
                if let Some(right) = &self.right_label {
                    ui.painter().text(
                        pos2(label_rect.right(), label_rect.center().y),
                        Align2::RIGHT_CENTER,
                        right,
                        FontId::new(14.0, FontFamily::Proportional),
                        p.text_secondary,
                    );
                }
            }

            ui.add_space(4.0);

            let resp = ui.horizontal(|ui| {
                let minus_resp = ui.add(
                    TwoUiIconButton::new(TwoUiIcon::Minus)
                        .size(32.0)
                        .icon_size(16.0),
                );
                if minus_resp.clicked() {
                    let next = (*self.value - self.step).clamp(start, end);
                    if (next - *self.value).abs() > f32::EPSILON {
                        *self.value = next;
                        changed = true;
                    }
                }

                let plus_resp_slot_w = 32.0 + ui.spacing().item_spacing.x;
                let slider_w = (ui.available_width() - plus_resp_slot_w).max(60.0);
                let mut sb_resp = None;
                ui.allocate_ui_with_layout(
                    Vec2::new(slider_w, 32.0),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        let r = ui.add(
                            TwoUiSeekBar::new(self.value, start..=end)
                                .mode(resolved_mode)
                                .seamless(self.seamless)
                                .center_based_ticks(self.center_based_ticks)
                                .label(&self.title),
                        );
                        if r.changed() {
                            changed = true;
                        }
                        sb_resp = Some(r);
                    },
                );

                let plus_resp = ui.add(
                    TwoUiIconButton::new(TwoUiIcon::Plus)
                        .size(32.0)
                        .icon_size(16.0),
                );
                if plus_resp.clicked() {
                    let next = (*self.value + self.step).clamp(start, end);
                    if (next - *self.value).abs() > f32::EPSILON {
                        *self.value = next;
                        changed = true;
                    }
                }
                sb_resp.unwrap_or(minus_resp)
            });

            resp.inner
        });

        let mut response = inner.inner;
        if changed {
            response.mark_changed();
        }
        response
    }
}
