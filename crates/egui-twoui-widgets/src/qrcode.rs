//! Rounded QR code card widget.

use egui::{vec2, Color32, CornerRadius, Rect, Response, Sense, Stroke, StrokeKind, Ui, Vec2, Widget};
use egui_twoui_icons::TwoUiIcon;
use egui_twoui_tokens::theme_from_ui;
use qrcode::{Color as QrModuleColor, EcLevel, QrCode};

/// Renders a scannable QR code card with customizable corner frame and center icon.
pub struct TwoUiQrCode<'a> {
    content: &'a str,
    size: f32,
    fg_color: Option<Color32>,
    bg_color: Color32,
    rounded_frame: bool,
    tint_anchor: bool,
    tint_frame: bool,
    center_icon: Option<TwoUiIcon>,
}

impl<'a> TwoUiQrCode<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            content,
            size: 184.0,
            fg_color: None,
            bg_color: Color32::WHITE,
            rounded_frame: true,
            tint_anchor: true,
            tint_frame: false,
            center_icon: None,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size.clamp(96.0, 360.0);
        self
    }

    pub fn foreground_color(mut self, color: Color32) -> Self {
        self.fg_color = Some(color);
        self
    }

    pub fn background_color(mut self, color: Color32) -> Self {
        self.bg_color = color;
        self
    }

    pub fn rounded_frame(mut self, rounded: bool) -> Self {
        self.rounded_frame = rounded;
        self
    }

    pub fn tint_anchor(mut self, tint: bool) -> Self {
        self.tint_anchor = tint;
        self
    }

    pub fn tint_frame(mut self, tint: bool) -> Self {
        self.tint_frame = tint;
        self
    }

    /// Places a centered circular badge with `icon` inside the QR code (`EcLevel::H`).
    pub fn icon(mut self, icon: TwoUiIcon) -> Self {
        self.center_icon = Some(icon);
        self
    }
}

impl Widget for TwoUiQrCode<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(self.size), Sense::hover());

        if !ui.is_rect_visible(rect) {
            return response;
        }

        let ec = if self.center_icon.is_some() {
            EcLevel::H
        } else {
            EcLevel::M
        };
        let Ok(code) = QrCode::with_error_correction_level(self.content.as_bytes(), ec) else {
            ui.painter()
                .rect_filled(rect, CornerRadius::same(18), self.bg_color);
            return response;
        };

        let fg = self.fg_color.unwrap_or(Color32::from_rgb(0x18, 0x18, 0x1A));
        let frame_radius = if self.rounded_frame {
            CornerRadius::same(22)
        } else {
            CornerRadius::ZERO
        };

        ui.painter().rect_filled(rect, frame_radius, self.bg_color);
        let border_col = if self.tint_frame { fg } else { p.divider };
        let border_w = if self.tint_frame { 2.0 } else { 1.0 };
        ui.painter().rect_stroke(
            rect,
            frame_radius,
            Stroke::new(border_w, border_col),
            StrokeKind::Inside,
        );

        let side_modules = code.width();
        let pad = (self.size * 0.11).round();
        let qr_rect = rect.shrink(pad);
        let cell = qr_rect.width() / side_modules as f32;
        let modules = code.to_colors();

        let anchor_color = if self.tint_anchor {
            fg
        } else {
            Color32::from_rgb(0x18, 0x18, 0x1A)
        };

        let icon_zone_half = if self.center_icon.is_some() {
            (side_modules as f32) * 0.15
        } else {
            0.0
        };
        let mid_idx = side_modules as f32 * 0.5;

        for row in 0..side_modules {
            for col in 0..side_modules {
                if is_in_finder_pattern(row, col, side_modules) {
                    continue;
                }
                if self.center_icon.is_some()
                    && (row as f32 + 0.5 - mid_idx).abs() < icon_zone_half
                    && (col as f32 + 0.5 - mid_idx).abs() < icon_zone_half
                {
                    continue;
                }
                if modules[row * side_modules + col] == QrModuleColor::Dark {
                    let mod_rect = Rect::from_min_size(
                        egui::pos2(
                            qr_rect.left() + col as f32 * cell,
                            qr_rect.top() + row as f32 * cell,
                        ),
                        vec2(cell, cell),
                    );
                    if self.rounded_frame {
                        ui.painter()
                            .circle_filled(mod_rect.center(), cell * 0.44, fg);
                    } else {
                        ui.painter().rect_filled(mod_rect, CornerRadius::ZERO, fg);
                    }
                }
            }
        }

        for (ar, ac) in [(0usize, 0usize), (0, side_modules - 7), (side_modules - 7, 0)] {
            let outer = Rect::from_min_size(
                egui::pos2(
                    qr_rect.left() + ac as f32 * cell,
                    qr_rect.top() + ar as f32 * cell,
                ),
                vec2(cell * 7.0, cell * 7.0),
            );
            let inner_bg = outer.shrink(cell);
            let core = outer.shrink(cell * 2.0);
            let r_outer = if self.rounded_frame {
                CornerRadius::same((cell * 1.8).round() as u8)
            } else {
                CornerRadius::ZERO
            };
            let r_inner = if self.rounded_frame {
                CornerRadius::same((cell * 1.1).round() as u8)
            } else {
                CornerRadius::ZERO
            };
            let r_core = if self.rounded_frame {
                CornerRadius::same((cell * 0.9).round() as u8)
            } else {
                CornerRadius::ZERO
            };

            ui.painter().rect_filled(outer, r_outer, anchor_color);
            ui.painter().rect_filled(inner_bg, r_inner, self.bg_color);
            ui.painter().rect_filled(core, r_core, anchor_color);
        }

        if let Some(icon) = self.center_icon {
            let badge_size = qr_rect.width() * 0.27;
            let badge_rect = Rect::from_center_size(qr_rect.center(), Vec2::splat(badge_size));
            ui.painter()
                .rect_filled(badge_rect, CornerRadius::same(10), self.bg_color);
            let icon_rect = badge_rect.shrink(badge_size * 0.18);
            icon.paint(ui, icon_rect, fg);
        }

        response
    }
}

fn is_in_finder_pattern(row: usize, col: usize, size: usize) -> bool {
    let top_left = row < 7 && col < 7;
    let top_right = row < 7 && col >= size.saturating_sub(7);
    let bottom_left = row >= size.saturating_sub(7) && col < 7;
    top_left || top_right || bottom_left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qr_encodes_valid_square_matrix() {
        let code =
            QrCode::with_error_correction_level(b"https://github.com/nightofnoreturn/TwoUI-Crate", EcLevel::H)
                .expect("qr generation");
        assert!(code.width() >= 21);
        assert_eq!(code.to_colors().len(), code.width() * code.width());
    }
}
