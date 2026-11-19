//! Vertical spinning-wheel number picker.

use egui::{pos2, vec2, Align2, CornerRadius, FontFamily, FontId, Id, Rect, Response, Sense, Ui, Widget};
use egui_twoui_tokens::{interaction_factors, theme_from_ui};

/// Vertical wheel number picker (`144dp` height).
pub struct TwoUiNumberPicker<'a> {
    id_salt: &'a str,
    value: &'a mut i32,
    min: i32,
    max: i32,
    width: f32,
    height: f32,
    min_digits: usize,
    wrap_around: bool,
    labels: Option<&'a [&'a str]>,
    text_size: f32,
}

impl<'a> TwoUiNumberPicker<'a> {
    pub fn new(id_salt: &'a str, value: &'a mut i32, min: i32, max: i32) -> Self {
        let lo = min.min(max);
        let hi = min.max(max);
        *value = (*value).clamp(lo, hi);
        Self {
            id_salt,
            value,
            min: lo,
            max: hi,
            width: 82.0,
            height: 144.0,
            min_digits: 1,
            wrap_around: true,
            labels: None,
            text_size: 25.0,
        }
    }

    pub fn text_size(mut self, size: f32) -> Self {
        self.text_size = size.clamp(14.0, 34.0);
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width.clamp(52.0, 180.0);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height.clamp(112.0, 240.0);
        self
    }

    pub fn min_digits(mut self, digits: usize) -> Self {
        self.min_digits = digits.max(1);
        self
    }

    pub fn wrap_around(mut self, wrap: bool) -> Self {
        self.wrap_around = wrap;
        self
    }

    pub fn displayed_labels(mut self, labels: &'a [&'a str]) -> Self {
        self.labels = Some(labels);
        self
    }

    fn step_value(&self, v: i32, delta: i32) -> i32 {
        let span = self.max - self.min + 1;
        if span <= 1 {
            return self.min;
        }
        if self.wrap_around {
            self.min + (v - self.min + delta).rem_euclid(span)
        } else {
            (v + delta).clamp(self.min, self.max)
        }
    }

    fn format_val(&self, v: i32) -> String {
        if let Some(labels) = self.labels {
            let idx = (v - self.min) as usize;
            if let Some(lbl) = labels.get(idx) {
                return (*lbl).to_owned();
            }
        }
        if self.min_digits > 1 && v >= 0 {
            format!("{:0width$}", v, width = self.min_digits)
        } else {
            v.to_string()
        }
    }
}

impl Widget for TwoUiNumberPicker<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let (rect, mut response) =
            ui.allocate_exact_size(vec2(self.width, self.height), Sense::click_and_drag());

        let drag_mem_id = Id::unique(("twoui_num_drag", self.id_salt));
        let mut accum_dy = ui.ctx().data(|d| d.get_temp::<f32>(drag_mem_id)).unwrap_or(0.0);

        let row_h = self.height / 3.0;
        let prev_val = *self.value;

        if response.dragged() {
            accum_dy += response.drag_delta().y;
            let threshold = 22.0;
            while accum_dy <= -threshold {
                *self.value = self.step_value(*self.value, 1);
                accum_dy += threshold;
            }
            while accum_dy >= threshold {
                *self.value = self.step_value(*self.value, -1);
                accum_dy -= threshold;
            }
        } else {
            accum_dy = 0.0;
        }
        ui.ctx().data_mut(|d| d.insert_temp(drag_mem_id, accum_dy));

        if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                if pos.y < rect.top() + row_h {
                    *self.value = self.step_value(*self.value, -1);
                } else if pos.y > rect.bottom() - row_h {
                    *self.value = self.step_value(*self.value, 1);
                }
            }
        }

        if response.hovered() {
            let wheel_y = ui.input(|i| i.smooth_scroll_delta.y);
            if wheel_y.abs() > 14.0 {
                let dir = if wheel_y < 0.0 { 1 } else { -1 };
                *self.value = self.step_value(*self.value, dir);
            }
        }

        if *self.value != prev_val {
            response.mark_changed();
        }

        if ui.is_rect_visible(rect) {
            let (press_t, hover_t) = interaction_factors(ui, &response);
            let center_band = Rect::from_center_size(
                rect.center(),
                vec2((rect.width() - 8.0).max(20.0), row_h - 4.0),
            );
            let band_fill = p
                .tone_on_tone
                .lerp_to_gamma(p.primary.gamma_multiply(0.16), (hover_t * 0.4 + press_t * 0.6).min(1.0));
            ui.painter()
                .rect_filled(center_band, CornerRadius::same(16), band_fill);

            let top_neighbor = if self.wrap_around || *self.value > self.min {
                Some(self.step_value(*self.value, -1))
            } else {
                None
            };
            let bot_neighbor = if self.wrap_around || *self.value < self.max {
                Some(self.step_value(*self.value, 1))
            } else {
                None
            };

            let neighbor_size = (self.text_size * 0.64).max(14.0);
            if let Some(v) = top_neighbor {
                ui.painter().text(
                    pos2(rect.center().x, rect.top() + row_h * 0.5),
                    Align2::CENTER_CENTER,
                    self.format_val(v),
                    FontId::new(neighbor_size, FontFamily::Proportional),
                    p.text_muted.gamma_multiply(0.68),
                );
            }
            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                self.format_val(*self.value),
                FontId::new(self.text_size, FontFamily::Proportional),
                p.text_primary,
            );
            if let Some(v) = bot_neighbor {
                ui.painter().text(
                    pos2(rect.center().x, rect.bottom() - row_h * 0.5),
                    Align2::CENTER_CENTER,
                    self.format_val(v),
                    FontId::new(neighbor_size, FontFamily::Proportional),
                    p.text_muted.gamma_multiply(0.68),
                );
            }
        }

        response
    }
}
