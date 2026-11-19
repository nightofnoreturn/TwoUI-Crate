//! 24-hour Sleep / Start-End Time Range circular dial picker.

use std::f32::consts::{FRAC_PI_2, TAU};

use egui::{
    pos2, vec2, Align2, Color32, CornerRadius, FontFamily, FontId, Id, Mesh, Pos2, Rect, Sense,
    Shape, Stroke, Ui, Vec2,
};
use egui_twoui_icons::TwoUiIcon;
use egui_twoui_tokens::theme_from_ui;

use super::centered_column;

/// Interactive 24-hour Start/End dual-handle circular range dial.
pub struct TwoUiSleepTimePicker<'a> {
    id_salt: &'a str,
    bedtime_min: &'a mut i32,
    wakeup_min: &'a mut i32,
    is_24_hour: bool,
}

impl<'a> TwoUiSleepTimePicker<'a> {
    /// Creates a 24-hour circular sleep/range picker with minutes in `0..1440`.
    pub fn new(id_salt: &'a str, bedtime_min: &'a mut i32, wakeup_min: &'a mut i32) -> Self {
        *bedtime_min = (*bedtime_min).rem_euclid(1440);
        *wakeup_min = (*wakeup_min).rem_euclid(1440);
        Self {
            id_salt,
            bedtime_min,
            wakeup_min,
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

        let panel_w = ui.available_width().clamp(220.0, 440.0);
        centered_column(ui, panel_w, |ui| {
            ui.horizontal(|ui| {
                let pill_w = (panel_w - 10.0) * 0.5;
                for (idx, (icon, label, mins, col)) in [
                    (TwoUiIcon::Moon, "Bedtime", *self.bedtime_min, p.primary_high),
                    (
                        TwoUiIcon::Sun,
                        "Wake-up",
                        *self.wakeup_min,
                        p.functional_orange,
                    ),
                ]
                .into_iter()
                .enumerate()
                {
                    if idx == 1 {
                        ui.add_space(10.0);
                    }
                    let (r, _) = ui.allocate_exact_size(vec2(pill_w, 50.0), Sense::hover());
                    if ui.is_rect_visible(r) {
                        ui.painter()
                            .rect_filled(r, CornerRadius::same(16), p.surface_container);
                        icon.paint(
                            ui,
                            Rect::from_center_size(
                                pos2(r.left() + 18.0, r.center().y),
                                Vec2::splat(16.0),
                            ),
                            col,
                        );
                        ui.painter().text(
                            pos2(r.left() + 32.0, r.top() + 14.0),
                            Align2::LEFT_CENTER,
                            label,
                            FontId::new(11.5, FontFamily::Proportional),
                            p.text_muted,
                        );
                        ui.painter().text(
                            pos2(r.left() + 32.0, r.bottom() - 15.0),
                            Align2::LEFT_CENTER,
                            format_minutes_label(mins, self.is_24_hour),
                            FontId::new(15.5, FontFamily::Proportional),
                            p.text_primary,
                        );
                    }
                }
            });

            ui.add_space(10.0);

            let dial_d = panel_w.clamp(190.0, 256.0);
            let (dial_rect, resp) =
                ui.allocate_exact_size(vec2(panel_w, dial_d), Sense::click_and_drag());
            let center = dial_rect.center();
            let ring_r = (dial_d * 0.5 - 16.0).max(52.0);
            let ring_w = 22.0;

            let bed_ang = -FRAC_PI_2 + (*self.bedtime_min as f32 / 1440.0) * TAU;
            let wake_ang = -FRAC_PI_2 + (*self.wakeup_min as f32 / 1440.0) * TAU;
            let bed_pos = center + vec2(bed_ang.cos(), bed_ang.sin()) * ring_r;
            let wake_pos = center + vec2(wake_ang.cos(), wake_ang.sin()) * ring_r;

            let drag_which_id = Id::unique((self.id_salt, "sleep_drag"));
            let active_handle: Option<u8> = ui
                .ctx()
                .data(|d| d.get_temp::<Option<u8>>(drag_which_id))
                .flatten();

            if resp.drag_started() || resp.clicked() {
                if let Some(pos) = resp.interact_pointer_pos() {
                    let d_bed = pos.distance_sq(bed_pos);
                    let d_wake = pos.distance_sq(wake_pos);
                    let picked = Some(if d_bed <= d_wake { 0 } else { 1 });
                    ui.ctx().data_mut(|d| d.insert_temp(drag_which_id, picked));
                }
            }
            if (resp.dragged() || resp.clicked()) && resp.interact_pointer_pos().is_some() {
                if let Some(pos) = resp.interact_pointer_pos() {
                    let which = ui
                        .ctx()
                        .data(|d| d.get_temp::<Option<u8>>(drag_which_id))
                        .flatten()
                        .unwrap_or_else(|| {
                            if pos.distance_sq(bed_pos) <= pos.distance_sq(wake_pos) {
                                0
                            } else {
                                1
                            }
                        });
                    let v = pos - center;
                    let ang = (v.y.atan2(v.x) + FRAC_PI_2).rem_euclid(TAU);
                    let snapped = (((ang / TAU) * 1440.0 / 5.0).round() as i32 * 5).rem_euclid(1440);
                    if which == 0 {
                        if *self.bedtime_min != snapped {
                            *self.bedtime_min = snapped;
                            changed = true;
                        }
                    } else if *self.wakeup_min != snapped {
                        *self.wakeup_min = snapped;
                        changed = true;
                    }
                }
            } else if !resp.is_pointer_button_down_on() && active_handle.is_some() {
                ui.ctx()
                    .data_mut(|d| d.insert_temp::<Option<u8>>(drag_which_id, None));
            }

            if ui.is_rect_visible(dial_rect) {
                paint_dial_ring_arc(ui, center, ring_r, ring_w, -FRAC_PI_2, TAU, p.progress_track);

                let dur_min = (*self.wakeup_min - *self.bedtime_min).rem_euclid(1440);
                let sweep = (dur_min as f32 / 1440.0) * TAU;
                let bed_ang = -FRAC_PI_2 + (*self.bedtime_min as f32 / 1440.0) * TAU;
                let wake_ang = bed_ang + sweep;
                if dur_min > 5 {
                    paint_dial_ring_arc(ui, center, ring_r, ring_w, bed_ang, sweep, p.primary);
                }

                for hr in 0..24 {
                    let a = -FRAC_PI_2 + (hr as f32 / 24.0) * TAU;
                    let pt = center + vec2(a.cos(), a.sin()) * (ring_r - ring_w * 0.5 - 7.0);
                    let r_dot = if hr % 6 == 0 { 1.8 } else { 1.0 };
                    ui.painter().circle_filled(pt, r_dot, p.divider);
                }

                let num_r = ring_r - ring_w * 0.5 - 19.0;
                for (hr, txt) in [(0, "0"), (6, "6"), (12, "12"), (18, "18")] {
                    let a = -FRAC_PI_2 + (hr as f32 / 24.0) * TAU;
                    ui.painter().text(
                        center + vec2(a.cos(), a.sin()) * num_r,
                        Align2::CENTER_CENTER,
                        txt,
                        FontId::new(11.5, FontFamily::Proportional),
                        p.text_muted,
                    );
                }

                let hrs = dur_min / 60;
                let mins = dur_min % 60;
                ui.painter().text(
                    pos2(center.x, center.y - 7.0),
                    Align2::CENTER_CENTER,
                    format!("{hrs}h {mins:02}m"),
                    FontId::new(17.5, FontFamily::Proportional),
                    p.text_primary,
                );
                ui.painter().text(
                    pos2(center.x, center.y + 12.0),
                    Align2::CENTER_CENTER,
                    "Duration",
                    FontId::new(11.5, FontFamily::Proportional),
                    p.text_muted,
                );

                let bed_pos = center + vec2(bed_ang.cos(), bed_ang.sin()) * ring_r;
                let wake_pos = center + vec2(wake_ang.cos(), wake_ang.sin()) * ring_r;
                for (pos, icon, accent) in [
                    (bed_pos, TwoUiIcon::Moon, p.primary_high),
                    (wake_pos, TwoUiIcon::Sun, p.functional_orange),
                ] {
                    ui.painter().circle_filled(pos, 12.0, p.surface);
                    ui.painter()
                        .circle_stroke(pos, 12.0, Stroke::new(2.2, accent));
                    icon.paint(ui, Rect::from_center_size(pos, Vec2::splat(13.0)), accent);
                }
            }
        });

        changed
    }
}

fn format_minutes_label(mins: i32, is_24_hour: bool) -> String {
    let m = mins.rem_euclid(1440);
    let h24 = m / 60;
    let mm = m % 60;
    if is_24_hour {
        format!("{h24:02}:{mm:02}")
    } else {
        let suffix = if h24 >= 12 { "PM" } else { "AM" };
        let h12 = match h24 % 12 {
            0 => 12,
            x => x,
        };
        format!("{h12}:{mm:02} {suffix}")
    }
}

fn paint_dial_ring_arc(
    ui: &Ui,
    center: Pos2,
    radius: f32,
    width: f32,
    start_angle: f32,
    sweep_angle: f32,
    color: Color32,
) {
    if sweep_angle.abs() <= 0.002 {
        return;
    }
    let half = width * 0.5;
    let r_in = (radius - half).max(1.0);
    let r_out = radius + half;
    let steps = ((sweep_angle.abs() / TAU) * 72.0).ceil().clamp(6.0, 72.0) as usize;
    let mut mesh = Mesh::default();
    for i in 0..=steps {
        let a = start_angle + sweep_angle * (i as f32 / steps as f32);
        let u = vec2(a.cos(), a.sin());
        mesh.colored_vertex(center + u * r_in, color);
        mesh.colored_vertex(center + u * r_out, color);
    }
    for i in 0..steps {
        let b = (i * 2) as u32;
        mesh.add_triangle(b, b + 1, b + 2);
        mesh.add_triangle(b + 1, b + 3, b + 2);
    }
    ui.painter().add(Shape::mesh(mesh));
    if sweep_angle.abs() < TAU - 0.01 {
        let end = start_angle + sweep_angle;
        ui.painter().circle_filled(
            center + vec2(start_angle.cos(), start_angle.sin()) * radius,
            half,
            color,
        );
        ui.painter()
            .circle_filled(center + vec2(end.cos(), end.sin()) * radius, half, color);
    }
}
