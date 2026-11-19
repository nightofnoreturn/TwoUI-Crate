//! HSV + Alpha ColorPicker widget.

use std::f32::consts::FRAC_PI_2;

use egui::{pos2, vec2, Color32, CornerRadius, Id, Mesh, Rect, RichText, Sense, Shape, Stroke, StrokeKind, Ui};
use egui_twoui_tokens::theme_from_ui;

use crate::tabs::SubTabBar;

use super::centered_column;

/// Interactive HSV + Alpha color picker with spectrum panel, sliders, and curated swatches.
pub struct TwoUiColorPicker<'a> {
    id_salt: &'a str,
    color: &'a mut Color32,
    show_alpha: bool,
}

impl<'a> TwoUiColorPicker<'a> {
    pub fn new(id_salt: &'a str, color: &'a mut Color32) -> Self {
        Self {
            id_salt,
            color,
            show_alpha: true,
        }
    }

    pub fn show_alpha(mut self, enabled: bool) -> Self {
        self.show_alpha = enabled;
        self
    }

    pub fn show(self, ui: &mut Ui) -> bool {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let mut changed = false;
        let (mut h, mut s, mut v, mut a) = color32_to_hsva(*self.color);

        let w = ui.available_width().clamp(210.0, 480.0);
        let tab_mem_id = Id::unique((self.id_salt, "cp_tab"));
        let mut cp_tab = ui.ctx().data(|d| d.get_temp::<usize>(tab_mem_id)).unwrap_or(0);

        centered_column(ui, w, |ui| {
            SubTabBar::new(self.id_salt, &["Spectrum", "Swatches"], &mut cp_tab).show(ui);
            ui.ctx().data_mut(|d| d.insert_temp(tab_mem_id, cp_tab));
            ui.add_space(10.0);

            if cp_tab == 0 {
                let spec_h = 134.0;
                let (spec_rect, spec_resp) =
                    ui.allocate_exact_size(vec2(w, spec_h), Sense::click_and_drag());
                if (spec_resp.dragged() || spec_resp.clicked())
                    && spec_resp.interact_pointer_pos().is_some()
                {
                    if let Some(pos) = spec_resp.interact_pointer_pos() {
                        h = ((pos.x - spec_rect.left()) / spec_rect.width()).clamp(0.0, 1.0);
                        s = (1.0 - (pos.y - spec_rect.top()) / spec_rect.height()).clamp(0.0, 1.0);
                        *self.color = hsva_to_color32(h, s, v, a);
                        changed = true;
                    }
                }

                if ui.is_rect_visible(spec_rect) {
                    paint_rounded_hue_sat_mesh(ui, spec_rect, v, 12.0);
                    ui.painter().rect_stroke(
                        spec_rect,
                        CornerRadius::same(12),
                        Stroke::new(1.0, p.divider),
                        StrokeKind::Inside,
                    );
                    let knob = pos2(
                        spec_rect.left() + h * spec_rect.width(),
                        spec_rect.top() + (1.0 - s) * spec_rect.height(),
                    );
                    let drag_t = ui.ctx().animate_bool_responsive(
                        Id::unique((self.id_salt, "spec_knob")),
                        spec_resp.is_pointer_button_down_on(),
                    );
                    let knob_r = egui::lerp(9.0..=11.5, drag_t);
                    ui.painter()
                        .circle_filled(knob, knob_r, hsva_to_color32(h, s, v, 1.0));
                    ui.painter()
                        .circle_stroke(knob, knob_r, Stroke::new(2.5, Color32::WHITE));
                }

                ui.add_space(10.0);
                if value_bar(ui, w, &mut v, h, s, false, a) {
                    *self.color = hsva_to_color32(h, s, v, a);
                    changed = true;
                }

                if self.show_alpha {
                    ui.add_space(8.0);
                    if value_bar(ui, w, &mut a, h, s, true, v) {
                        *self.color = hsva_to_color32(h, s, v, a);
                        changed = true;
                    }
                }
            } else {
                let palette_grid: [Color32; 24] = [
                    Color32::from_rgb(0xFF, 0xFF, 0xFF),
                    Color32::from_rgb(0xC6, 0xC6, 0xCC),
                    Color32::from_rgb(0x76, 0x76, 0x7A),
                    Color32::from_rgb(0x3A, 0x3A, 0x3D),
                    Color32::from_rgb(0x17, 0x17, 0x1A),
                    Color32::from_rgb(0x00, 0x00, 0x00),
                    Color32::from_rgb(0xD9, 0x3E, 0x3A),
                    Color32::from_rgb(0xE1, 0x4B, 0x6E),
                    Color32::from_rgb(0xEC, 0x40, 0x7A),
                    Color32::from_rgb(0xE6, 0x5B, 0x17),
                    Color32::from_rgb(0xFF, 0x70, 0x43),
                    Color32::from_rgb(0xFF, 0xB3, 0x00),
                    Color32::from_rgb(0x11, 0xA8, 0x5F),
                    Color32::from_rgb(0x23, 0xC4, 0x92),
                    Color32::from_rgb(0x00, 0xBF, 0xA5),
                    Color32::from_rgb(0x3B, 0xA3, 0xC3),
                    Color32::from_rgb(0x03, 0x81, 0xFE),
                    Color32::from_rgb(0x38, 0x7A, 0xFF),
                    Color32::from_rgb(0x5B, 0x6B, 0xC0),
                    Color32::from_rgb(0x7C, 0x5C, 0xFF),
                    Color32::from_rgb(0x9C, 0x27, 0xB0),
                    Color32::from_rgb(0x79, 0x86, 0xCB),
                    Color32::from_rgb(0x4D, 0xD0, 0xE1),
                    Color32::from_rgb(0xAE, 0xD5, 0x81),
                ];
                let cols = 6usize;
                let cell_w = w / cols as f32;
                let row_h = 38.0;
                for row in 0..4 {
                    let (row_r, _) = ui.allocate_exact_size(vec2(w, row_h), Sense::hover());
                    for col_i in 0..cols {
                        let idx = row * cols + col_i;
                        let swatch = palette_grid[idx];
                        let sr = Rect::from_min_size(
                            pos2(row_r.left() + col_i as f32 * cell_w, row_r.top()),
                            vec2(cell_w, row_h),
                        );
                        let resp = ui.interact(
                            sr,
                            Id::unique((self.id_salt, "sw_cell", idx)),
                            Sense::click(),
                        );
                        if resp.clicked() {
                            *self.color = swatch;
                            changed = true;
                        }
                        if ui.is_rect_visible(sr) {
                            ui.painter().circle_filled(sr.center(), 13.5, swatch);
                            if *self.color == swatch {
                                ui.painter().circle_stroke(
                                    sr.center(),
                                    15.8,
                                    Stroke::new(2.2, p.text_primary),
                                );
                            } else {
                                ui.painter().circle_stroke(
                                    sr.center(),
                                    13.5,
                                    Stroke::new(0.8, p.divider),
                                );
                            }
                        }
                    }
                }
            }

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                let (prev_r, _) = ui.allocate_exact_size(vec2(52.0, 26.0), Sense::hover());
                ui.painter()
                    .rect_filled(prev_r, CornerRadius::same(13), *self.color);
                ui.painter().rect_stroke(
                    prev_r,
                    CornerRadius::same(13),
                    Stroke::new(1.0, p.divider),
                    StrokeKind::Inside,
                );
                ui.add_space(6.0);
                ui.label(
                    RichText::new(format!(
                        "#{:02X}{:02X}{:02X}{:02X}",
                        self.color.a(),
                        self.color.r(),
                        self.color.g(),
                        self.color.b()
                    ))
                    .size(13.5)
                    .strong()
                    .color(p.text_primary),
                );
            });
        });

        changed
    }
}

fn value_bar(
    ui: &mut Ui,
    width: f32,
    slider: &mut f32,
    h: f32,
    s: f32,
    is_alpha: bool,
    other: f32,
) -> bool {
    let (rect, resp) = ui.allocate_exact_size(vec2(width, 22.0), Sense::click_and_drag());
    let mut changed = false;
    if (resp.dragged() || resp.clicked()) && resp.interact_pointer_pos().is_some() {
        if let Some(pos) = resp.interact_pointer_pos() {
            *slider = ((pos.x - rect.left()) / rect.width()).clamp(0.0, 1.0);
            changed = true;
        }
    }
    if ui.is_rect_visible(rect) {
        let bar_rect = Rect::from_center_size(rect.center(), vec2(width, 14.0));
        let r = bar_rect.height() * 0.5;
        let cy = bar_rect.center().y;
        let mut slices: Vec<(f32, f32, f32)> = Vec::with_capacity(32);
        let cx_l = bar_rect.left() + r;
        for i in 0..=8 {
            let a = (i as f32 / 8.0) * FRAC_PI_2;
            slices.push((cx_l - r * a.cos(), a.sin(), 0.0));
        }
        let body_w = (bar_rect.width() - 2.0 * r).max(0.0);
        for i in 1..16 {
            let x = cx_l + body_w * (i as f32 / 16.0);
            slices.push((x, 1.0, 0.0));
        }
        let cx_r = bar_rect.right() - r;
        for i in 0..=8 {
            let a = FRAC_PI_2 * (1.0 - i as f32 / 8.0);
            slices.push((cx_r + r * a.cos(), a.sin(), 0.0));
        }

        let mut mesh = Mesh::default();
        for &(x, dy, _) in &slices {
            let t = ((x - bar_rect.left()) / bar_rect.width()).clamp(0.0, 1.0);
            let col = if is_alpha {
                hsva_to_color32(h, s, other, t)
            } else {
                hsva_to_color32(h, s, t, 1.0)
            };
            let hy = (r * dy).max(0.1);
            mesh.colored_vertex(pos2(x, cy - hy), col);
            mesh.colored_vertex(pos2(x, cy + hy), col);
        }
        for i in 0..(slices.len().saturating_sub(1)) {
            let b = (i * 2) as u32;
            mesh.add_triangle(b, b + 1, b + 2);
            mesh.add_triangle(b + 1, b + 3, b + 2);
        }
        ui.painter().add(Shape::mesh(mesh));
        let kx = bar_rect.left() + bar_rect.width() * (*slider);
        ui.painter()
            .circle_filled(pos2(kx, bar_rect.center().y), 9.0, Color32::WHITE);
        ui.painter().circle_stroke(
            pos2(kx, bar_rect.center().y),
            9.0,
            Stroke::new(2.0, Color32::from_gray(90)),
        );
    }
    changed
}

fn paint_rounded_hue_sat_mesh(ui: &Ui, rect: Rect, value: f32, r: f32) {
    let w = rect.width();
    let h = rect.height();
    let r = r.min(w * 0.5).min(h * 0.5);
    let mut xs: Vec<f32> = Vec::with_capacity(40);
    for i in 0..=8 {
        let a = (i as f32 / 8.0) * FRAC_PI_2;
        xs.push(rect.left() + r * (1.0 - a.cos()));
    }
    let body_w = (w - 2.0 * r).max(0.0);
    for i in 1..20 {
        xs.push(rect.left() + r + body_w * (i as f32 / 20.0));
    }
    for i in 0..=8 {
        let a = FRAC_PI_2 * (1.0 - i as f32 / 8.0);
        xs.push(rect.right() - r + r * a.cos());
    }

    let mut mesh = Mesh::default();
    for &x in &xs {
        let hue = ((x - rect.left()) / w).clamp(0.0, 1.0);
        let inset_y = if x < rect.left() + r {
            let dx = (rect.left() + r) - x;
            r - (r * r - dx * dx).max(0.0).sqrt()
        } else if x > rect.right() - r {
            let dx = x - (rect.right() - r);
            r - (r * r - dx * dx).max(0.0).sqrt()
        } else {
            0.0
        };
        let top_y = (rect.top() + inset_y).min(rect.center().y);
        let bot_y = (rect.bottom() - inset_y).max(rect.center().y);
        let sat_top = (1.0 - (top_y - rect.top()) / h).clamp(0.0, 1.0);
        let sat_bot = (1.0 - (bot_y - rect.top()) / h).clamp(0.0, 1.0);
        mesh.colored_vertex(pos2(x, top_y), hsva_to_color32(hue, sat_top, value, 1.0));
        mesh.colored_vertex(pos2(x, bot_y), hsva_to_color32(hue, sat_bot, value, 1.0));
    }
    for i in 0..(xs.len().saturating_sub(1)) {
        let b = (i * 2) as u32;
        mesh.add_triangle(b, b + 1, b + 2);
        mesh.add_triangle(b + 1, b + 3, b + 2);
    }
    ui.painter().add(Shape::mesh(mesh));
}

/// Converts normalized `(h, s, v, a)` (`0.0..=1.0`) into `Color32`.
pub fn hsva_to_color32(h: f32, s: f32, v: f32, a: f32) -> Color32 {
    let h6 = (h.rem_euclid(1.0)) * 6.0;
    let i = h6.floor() as i32;
    let f = h6 - i as f32;
    let p = v * (1.0 - s);
    let q = v * (1.0 - f * s);
    let t = v * (1.0 - (1.0 - f) * s);
    let (r, g, b) = match i.rem_euclid(6) {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    };
    Color32::from_rgba_unmultiplied(
        (r * 255.0).round().clamp(0.0, 255.0) as u8,
        (g * 255.0).round().clamp(0.0, 255.0) as u8,
        (b * 255.0).round().clamp(0.0, 255.0) as u8,
        (a * 255.0).round().clamp(0.0, 255.0) as u8,
    )
}

/// Converts `Color32` into normalized `(h, s, v, a)` (`0.0..=1.0`).
pub fn color32_to_hsva(c: Color32) -> (f32, f32, f32, f32) {
    let r = f32::from(c.r()) / 255.0;
    let g = f32::from(c.g()) / 255.0;
    let b = f32::from(c.b()) / 255.0;
    let a = f32::from(c.a()) / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let d = max - min;
    let s = if max <= 0.0 { 0.0 } else { d / max };
    let h = if d <= f32::EPSILON {
        0.0
    } else if (max - r).abs() <= f32::EPSILON {
        ((g - b) / d + if g < b { 6.0 } else { 0.0 }) / 6.0
    } else if (max - g).abs() <= f32::EPSILON {
        ((b - r) / d + 2.0) / 6.0
    } else {
        ((r - g) / d + 4.0) / 6.0
    };
    (h.clamp(0.0, 1.0), s.clamp(0.0, 1.0), max.clamp(0.0, 1.0), a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hsv_roundtrip_preserves_primary() {
        let primary = Color32::from_rgb(0x03, 0x81, 0xFE);
        let (h, s, v, a) = color32_to_hsva(primary);
        let restored = hsva_to_color32(h, s, v, a);
        assert_eq!(restored, primary);
    }
}
