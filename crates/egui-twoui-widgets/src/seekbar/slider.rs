use std::ops::RangeInclusive;

use egui::{
    pos2, vec2, Color32, CornerRadius, Pos2, Rect, Response, Sense, Stroke, Ui, Widget, WidgetInfo,
};
use egui_twoui_tokens::{
    animate_toggle, cubic_bezier, ease_fast_out_slow_in, ease_sine_in_out_80, theme_from_ui,
    tokens::{colors, gray},
};

fn ease_sine_in_out_90(t: f32) -> f32 {
    cubic_bezier(0.33, 0.0, 0.1, 1.0, t)
}

/// SeekBar track & thumb visual mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SeekBarMode {
    /// Thin `3dp` track expanding to `13dp` on press while the `6.5dp` thumb fades down.
    #[default]
    Standard,
    /// Capsule `14dp -> 22dp` slider expanding on press while the `10dp` thumb fades down.
    Expand,
    /// Discrete stepped bar (`14dp` height) with `8dp` inner tick dots and persistent `10dp` thumb.
    Level { steps: u32 },
}

/// Configurable SeekBar slider widget supporting horizontal & vertical orientations.
pub struct TwoUiSeekBar<'a> {
    value: &'a mut f32,
    range: RangeInclusive<f32>,
    mode: SeekBarMode,
    seamless: bool,
    overlap_start: Option<f32>,
    center_ticks_only: bool,
    vertical: bool,
    vertical_length: f32,
    enabled: bool,
    label: Option<String>,
}

impl<'a> TwoUiSeekBar<'a> {
    pub fn new(value: &'a mut f32, range: RangeInclusive<f32>) -> Self {
        Self {
            value,
            range,
            mode: SeekBarMode::Standard,
            seamless: true,
            overlap_start: None,
            center_ticks_only: false,
            vertical: false,
            vertical_length: 170.0,
            enabled: true,
            label: None,
        }
    }

    pub fn mode(mut self, mode: SeekBarMode) -> Self {
        self.mode = mode;
        self
    }

    /// If `false` in `Level` mode, snaps value cleanly to discrete steps.
    pub fn seamless(mut self, seamless: bool) -> Self {
        self.seamless = seamless;
        self
    }

    /// Sets normalized threshold `0.0..=1.0` above which the track highlights in warning orange.
    pub fn overlap_threshold(mut self, normalized_threshold: f32) -> Self {
        self.overlap_start = Some(normalized_threshold.clamp(0.0, 1.0));
        self
    }

    /// Shows only 3 tick marks (`start`, `center`, `end`).
    pub fn center_based_ticks(mut self, enabled: bool) -> Self {
        self.center_ticks_only = enabled;
        self
    }

    /// Switches SeekBar to vertical bottom-to-top orientation.
    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }

    /// Length in logical points when `vertical(true)` is enabled.
    pub fn length(mut self, length: f32) -> Self {
        self.vertical_length = length.max(80.0);
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    fn normalized(&self) -> f32 {
        let start = *self.range.start();
        let end = *self.range.end();
        if (end - start).abs() <= f32::EPSILON {
            0.0
        } else {
            ((*self.value - start) / (end - start)).clamp(0.0, 1.0)
        }
    }

    fn set_from_normalized(&mut self, mut norm: f32) {
        norm = norm.clamp(0.0, 1.0);
        if let SeekBarMode::Level { steps } = self.mode {
            if !self.seamless && steps > 0 {
                norm = (norm * steps as f32).round() / steps as f32;
            }
        }
        let start = *self.range.start();
        let end = *self.range.end();
        *self.value = start + (end - start) * norm;
    }

    fn paint_thumb(ui: &Ui, center: Pos2, radius: f32, fill: Color32, stroke_color: Color32) {
        if radius <= 0.4 {
            return;
        }
        let stroke_w = 2.0_f32.min(radius * 1.5);
        ui.painter().circle_filled(center, radius, fill);
        ui.painter()
            .circle_stroke(center, radius, Stroke::new(stroke_w, stroke_color));
    }

    fn paint_horizontal(
        &self,
        ui: &Ui,
        rect: Rect,
        expand_t: f32,
        thumb_vis: f32,
        alpha: f32,
    ) {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let dark = theme.mode.is_dark();
        let cy = rect.center().y;
        let side_pad = match self.mode {
            SeekBarMode::Standard => 10.0,
            SeekBarMode::Expand | SeekBarMode::Level { .. } => 11.0,
        };
        let track_left = rect.left() + side_pad;
        let track_right = rect.right() - side_pad;
        let norm = self.normalized();

        let active_overlap = if dark {
            colors::ORANGE_DARK_TEXT
        } else {
            Color32::from_rgb(0xF1, 0x46, 0x2F)
        };
        let tail_overlap = if dark {
            p.surface.lerp_to_gamma(active_overlap, 0.36)
        } else {
            Color32::from_rgb(0xF7, 0xC0, 0xBD)
        };
        let is_overlapped = self.overlap_start.is_some_and(|th| norm > th);
        let active_fill = if is_overlapped {
            active_overlap
        } else {
            p.primary
        }
        .gamma_multiply(alpha);
        let thumb_fill = (if dark { gray::GRAY_100 } else { gray::GRAY_01 }).gamma_multiply(alpha);
        let tick_color = (if dark { gray::GRAY_60 } else { gray::GRAY_40 }).gamma_multiply(alpha);
        let base_track_fill = p.progress_track.gamma_multiply(alpha);
        let overlap_tail_fill = tail_overlap.gamma_multiply(alpha);

        let paint_clipped_h_track = |track_h: f32| {
            let radius = CornerRadius::same((track_h * 0.5).ceil() as u8);
            let track_rect = Rect::from_min_max(
                pos2(track_left, cy - track_h * 0.5),
                pos2(track_right, cy + track_h * 0.5),
            );
            ui.painter()
                .rect_filled(track_rect, radius, base_track_fill);

            if norm > 0.001 {
                let clip_x = if norm >= 0.999 {
                    track_rect.right() + 1.0
                } else {
                    egui::lerp(track_left..=track_right, norm)
                };
                let active_clip = Rect::from_min_max(
                    pos2(track_rect.left() - 1.0, track_rect.top() - 1.0),
                    pos2(clip_x, track_rect.bottom() + 1.0),
                )
                .intersect(ui.clip_rect());
                if active_clip.width() > 0.0 && active_clip.height() > 0.0 {
                    ui.painter()
                        .with_clip_rect(active_clip)
                        .rect_filled(track_rect, radius, active_fill);
                }
            }

            if let Some(th) = self.overlap_start {
                let overlap_x = egui::lerp(track_left..=track_right, norm.max(th));
                let overlap_clip = Rect::from_min_max(
                    pos2(overlap_x, track_rect.top() - 1.0),
                    pos2(track_rect.right() + 1.0, track_rect.bottom() + 1.0),
                )
                .intersect(ui.clip_rect());
                if overlap_clip.width() > 0.0 && overlap_clip.height() > 0.0 {
                    ui.painter().with_clip_rect(overlap_clip).rect_filled(
                        track_rect,
                        radius,
                        overlap_tail_fill,
                    );
                }
            }
        };

        match self.mode {
            SeekBarMode::Standard => {
                let track_h = egui::lerp(3.0..=13.0, expand_t);
                paint_clipped_h_track(track_h);
                let thumb_x = egui::lerp(track_left..=track_right, norm);
                Self::paint_thumb(
                    ui,
                    pos2(thumb_x, cy),
                    6.5 * thumb_vis,
                    thumb_fill,
                    active_fill,
                );
            }
            SeekBarMode::Expand => {
                let track_h = egui::lerp(14.0..=22.0, expand_t);
                paint_clipped_h_track(track_h);
                let level_pad = 7.0;
                let thumb_x = egui::lerp((track_left + level_pad)..=(track_right - level_pad), norm);
                Self::paint_thumb(
                    ui,
                    pos2(thumb_x, cy),
                    10.0 * thumb_vis,
                    thumb_fill,
                    active_fill,
                );
            }
            SeekBarMode::Level { steps } => {
                let track_h = 14.0;
                let radius = CornerRadius::same(7);
                let track_rect = Rect::from_min_max(
                    pos2(track_left, cy - track_h * 0.5),
                    pos2(track_right, cy + track_h * 0.5),
                );
                ui.painter()
                    .rect_filled(track_rect, radius, base_track_fill);

                if let Some(th) = self.overlap_start {
                    let overlap_x = egui::lerp(track_left..=track_right, th);
                    let overlap_clip = Rect::from_min_max(
                        pos2(overlap_x, track_rect.top() - 1.0),
                        pos2(track_rect.right() + 1.0, track_rect.bottom() + 1.0),
                    )
                    .intersect(ui.clip_rect());
                    if overlap_clip.width() > 0.0 && overlap_clip.height() > 0.0 {
                        ui.painter().with_clip_rect(overlap_clip).rect_filled(
                            track_rect,
                            radius,
                            overlap_tail_fill,
                        );
                    }
                }

                let tick_count = if self.center_ticks_only {
                    2
                } else {
                    steps.max(1)
                };
                for i in 0..=tick_count {
                    let t = i as f32 / tick_count as f32;
                    let x = egui::lerp((track_left + 7.0)..=(track_right - 7.0), t);
                    ui.painter().circle_filled(pos2(x, cy), 4.0, tick_color);
                }

                let thumb_x = egui::lerp((track_left + 7.0)..=(track_right - 7.0), norm);
                Self::paint_thumb(ui, pos2(thumb_x, cy), 10.0, thumb_fill, active_fill);
            }
        }
    }

    fn paint_vertical(
        &self,
        ui: &Ui,
        rect: Rect,
        expand_t: f32,
        thumb_vis: f32,
        vert_std_press_t: f32,
        alpha: f32,
    ) {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let dark = theme.mode.is_dark();
        let cx = rect.center().x;
        let pad = match self.mode {
            SeekBarMode::Standard | SeekBarMode::Expand => 10.0,
            SeekBarMode::Level { .. } => 11.0,
        };
        let top = rect.top() + pad;
        let bottom = rect.bottom() - pad;
        let norm = self.normalized();

        let active_overlap = if dark {
            colors::ORANGE_DARK_TEXT
        } else {
            Color32::from_rgb(0xF1, 0x46, 0x2F)
        };
        let tail_overlap = if dark {
            p.surface.lerp_to_gamma(active_overlap, 0.36)
        } else {
            Color32::from_rgb(0xF7, 0xC0, 0xBD)
        };
        let is_overlapped = self.overlap_start.is_some_and(|th| norm > th);
        let active_fill = if is_overlapped {
            active_overlap
        } else {
            p.primary
        }
        .gamma_multiply(alpha);
        let thumb_fill = (if dark { gray::GRAY_100 } else { gray::GRAY_01 }).gamma_multiply(alpha);
        let tick_color = (if dark { gray::GRAY_60 } else { gray::GRAY_40 }).gamma_multiply(alpha);
        let base_track_fill = p.progress_track.gamma_multiply(alpha);
        let overlap_tail_fill = tail_overlap.gamma_multiply(alpha);

        let paint_clipped_v_track = |track_w: f32| {
            let radius = CornerRadius::same((track_w * 0.5).ceil() as u8);
            let track_rect = Rect::from_min_max(
                pos2(cx - track_w * 0.5, top),
                pos2(cx + track_w * 0.5, bottom),
            );
            ui.painter()
                .rect_filled(track_rect, radius, base_track_fill);

            if norm > 0.001 {
                let clip_y = if norm >= 0.999 {
                    track_rect.top() - 1.0
                } else {
                    egui::lerp(bottom..=top, norm)
                };
                let active_clip = Rect::from_min_max(
                    pos2(track_rect.left() - 1.0, clip_y),
                    pos2(track_rect.right() + 1.0, track_rect.bottom() + 1.0),
                )
                .intersect(ui.clip_rect());
                if active_clip.width() > 0.0 && active_clip.height() > 0.0 {
                    ui.painter()
                        .with_clip_rect(active_clip)
                        .rect_filled(track_rect, radius, active_fill);
                }
            }

            if let Some(th) = self.overlap_start {
                let overlap_y = egui::lerp(bottom..=top, norm.max(th));
                let overlap_clip = Rect::from_min_max(
                    pos2(track_rect.left() - 1.0, track_rect.top() - 1.0),
                    pos2(track_rect.right() + 1.0, overlap_y),
                )
                .intersect(ui.clip_rect());
                if overlap_clip.width() > 0.0 && overlap_clip.height() > 0.0 {
                    ui.painter().with_clip_rect(overlap_clip).rect_filled(
                        track_rect,
                        radius,
                        overlap_tail_fill,
                    );
                }
            }
        };

        let thumb_y = egui::lerp(bottom..=top, norm);
        match self.mode {
            SeekBarMode::Standard => {
                paint_clipped_v_track(3.0);
                let thumb_r = 6.5 * (1.0 + 0.097 * vert_std_press_t);
                Self::paint_thumb(ui, pos2(cx, thumb_y), thumb_r, thumb_fill, active_fill);
            }
            SeekBarMode::Expand => {
                let track_w = egui::lerp(3.0..=13.0, expand_t);
                paint_clipped_v_track(track_w);
                Self::paint_thumb(
                    ui,
                    pos2(cx, thumb_y),
                    6.5 * thumb_vis,
                    thumb_fill,
                    active_fill,
                );
            }
            SeekBarMode::Level { steps } => {
                let track_w = 14.0;
                let radius = CornerRadius::same(7);
                let track_rect = Rect::from_min_max(
                    pos2(cx - track_w * 0.5, top),
                    pos2(cx + track_w * 0.5, bottom),
                );
                ui.painter()
                    .rect_filled(track_rect, radius, base_track_fill);

                if let Some(th) = self.overlap_start {
                    let overlap_y = egui::lerp(bottom..=top, th);
                    let overlap_clip = Rect::from_min_max(
                        pos2(track_rect.left() - 1.0, track_rect.top() - 1.0),
                        pos2(track_rect.right() + 1.0, overlap_y),
                    )
                    .intersect(ui.clip_rect());
                    if overlap_clip.width() > 0.0 && overlap_clip.height() > 0.0 {
                        ui.painter().with_clip_rect(overlap_clip).rect_filled(
                            track_rect,
                            radius,
                            overlap_tail_fill,
                        );
                    }
                }

                let tick_count = if self.center_ticks_only {
                    2
                } else {
                    steps.max(1)
                };
                for i in 0..=tick_count {
                    let t = i as f32 / tick_count as f32;
                    let y = egui::lerp((bottom - 7.0)..=(top + 7.0), t);
                    ui.painter().circle_filled(pos2(cx, y), 4.0, tick_color);
                }

                let level_thumb_y = egui::lerp((bottom - 7.0)..=(top + 7.0), norm);
                Self::paint_thumb(
                    ui,
                    pos2(cx, level_thumb_y),
                    10.0,
                    thumb_fill,
                    active_fill,
                );
            }
        }
    }
}

impl Widget for TwoUiSeekBar<'_> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let is_enabled = self.enabled && ui.is_enabled();
        let sense = if is_enabled {
            Sense::click_and_drag()
        } else {
            Sense::hover()
        };

        let size = if self.vertical {
            vec2(32.0, self.vertical_length)
        } else {
            let width = ui.available_width().max(120.0);
            let h = match self.mode {
                SeekBarMode::Expand => 30.0,
                SeekBarMode::Standard | SeekBarMode::Level { .. } => 28.0,
            };
            vec2(width, h)
        };

        let (rect, mut response) = ui.allocate_exact_size(size, sense);
        if is_enabled {
            if let Some(pointer_pos) = response.interact_pointer_pos() {
                if response.dragged() || response.clicked() || response.drag_started() {
                    let prev = *self.value;
                    let norm = if self.vertical {
                        let pad = match self.mode {
                            SeekBarMode::Standard | SeekBarMode::Expand => 10.0,
                            SeekBarMode::Level { .. } => 18.0,
                        };
                        let top = rect.top() + pad;
                        let bottom = rect.bottom() - pad;
                        (bottom - pointer_pos.y) / (bottom - top).max(1.0)
                    } else {
                        let pad = match self.mode {
                            SeekBarMode::Standard => 10.0,
                            SeekBarMode::Expand => 11.0,
                            SeekBarMode::Level { .. } => 18.0,
                        };
                        let left = rect.left() + pad;
                        let right = rect.right() - pad;
                        (pointer_pos.x - left) / (right - left).max(1.0)
                    };
                    self.set_from_normalized(norm);
                    if (*self.value - prev).abs() > f32::EPSILON {
                        response.mark_changed();
                    }
                }
            }
        }

        if ui.is_rect_visible(rect) {
            let pressed = is_enabled && (response.is_pointer_button_down_on() || response.dragged());
            let should_expand = pressed
                && match self.mode {
                    SeekBarMode::Standard => !self.vertical,
                    SeekBarMode::Expand => true,
                    SeekBarMode::Level { .. } => false,
                };
            let expand_t = animate_toggle(
                ui,
                response.id.with("sb_expand"),
                should_expand,
                0.25,
                ease_sine_in_out_80,
            );
            let thumb_hide_dur = if pressed { 0.10 } else { 0.30 };
            let thumb_linear = ui.ctx().animate_bool_with_time(
                response.id.with("sb_thumb_hide"),
                pressed,
                thumb_hide_dur,
            );
            let thumb_vis = if pressed {
                1.0 - thumb_linear
            } else {
                ease_sine_in_out_90(1.0 - thumb_linear)
            };
            let vert_std_press_t = if self.vertical && matches!(self.mode, SeekBarMode::Standard) {
                animate_toggle(
                    ui,
                    response.id.with("sb_vpress"),
                    pressed,
                    if pressed { 0.533 } else { 0.133 },
                    ease_fast_out_slow_in,
                )
            } else {
                0.0
            };
            let alpha = if is_enabled { 1.0 } else { 0.42 };

            if self.vertical {
                self.paint_vertical(ui, rect, expand_t, thumb_vis, vert_std_press_t, alpha);
            } else {
                self.paint_horizontal(ui, rect, expand_t, thumb_vis, alpha);
            }
        }

        let current_val = f64::from(*self.value);
        let acc_label = self.label.as_deref().unwrap_or("SeekBar");
        response.widget_info(|| WidgetInfo::slider(is_enabled, current_val, acc_label));
        response
    }
}
