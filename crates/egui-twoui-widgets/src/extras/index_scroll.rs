use egui::{pos2, vec2, Align2, Color32, CornerRadius, FontFamily, FontId, Id, Rect, Sense, Ui};
use egui_twoui_tokens::{paint_frosted_surface, theme_from_ui, tokens::gray, ElevationTier};

/// Sticky A–Z fast-scroll index rail.
pub struct TwoUiIndexScroll<'a> {
    id_salt: &'a str,
    letters: &'a [char],
    active: Option<char>,
}

impl<'a> TwoUiIndexScroll<'a> {
    pub fn new(id_salt: &'a str, letters: &'a [char]) -> Self {
        Self {
            id_salt,
            letters,
            active: None,
        }
    }

    pub fn active_letter(mut self, letter: Option<char>) -> Self {
        self.active = letter;
        self
    }

    /// Renders the vertical A–Z index rail inside `ui`; returns newly scrubbed/clicked letter.
    pub fn show(self, ui: &mut Ui) -> Option<char> {
        if self.letters.is_empty() {
            return None;
        }
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let dark = theme.mode.is_dark();
        let rail_w = 24.0;
        let rail_h = ui.available_height().clamp(140.0, 480.0);
        let (rect, response) =
            ui.allocate_exact_size(vec2(rail_w, rail_h), Sense::click_and_drag());

        let mut picked = None;
        let scrubbing = response.dragged() || response.is_pointer_button_down_on();
        if (scrubbing || response.clicked()) && response.interact_pointer_pos().is_some() {
            if let Some(pos) = response.interact_pointer_pos() {
                let frac = ((pos.y - (rect.top() + 8.0)) / (rect.height() - 16.0).max(1.0))
                    .clamp(0.0, 0.9999);
                let idx = (frac * self.letters.len() as f32).floor() as usize;
                picked = self.letters.get(idx).copied();
            }
        }

        if ui.is_rect_visible(rect) {
            paint_frosted_surface(
                ui,
                rect,
                CornerRadius::same(12),
                p.floating_surface,
                Color32::TRANSPARENT,
                ElevationTier::Sm,
                dark,
            );

            let step = (rect.height() - 12.0) / self.letters.len() as f32;
            let current = picked.or(self.active);
            for (i, ch) in self.letters.iter().enumerate() {
                let y = rect.top() + 6.0 + (i as f32 + 0.5) * step;
                let is_sel = current == Some(*ch);
                if is_sel {
                    ui.painter().circle_filled(
                        pos2(rect.center().x, y),
                        8.5,
                        p.primary.gamma_multiply(0.20),
                    );
                }
                ui.painter().text(
                    pos2(rect.center().x, y),
                    Align2::CENTER_CENTER,
                    ch.to_string(),
                    FontId::new(if is_sel { 11.5 } else { 10.0 }, FontFamily::Proportional),
                    if is_sel {
                        p.primary_high
                    } else {
                        p.text_secondary
                    },
                );
            }

            let bubble_t = ui.ctx().animate_bool_with_time(
                Id::unique((self.id_salt, "twoui_idx_bubble")),
                scrubbing,
                0.12,
            );
            if bubble_t > 0.02 {
                if let Some(ch) = current {
                    let pointer_y = response
                        .interact_pointer_pos()
                        .map_or(rect.center().y, |pos| pos.y);
                    let bubble_center = pos2(
                        rect.left() - 42.0,
                        pointer_y.clamp(rect.top() + 30.0, rect.bottom() - 30.0),
                    );
                    let br = 30.0 * bubble_t;
                    let bubble_fill = if dark { gray::GRAY_85 } else { gray::GRAY_01 }
                        .gamma_multiply(bubble_t);
                    let bubble_rect =
                        Rect::from_center_size(bubble_center, egui::Vec2::splat(br * 2.0));
                    paint_frosted_surface(
                        ui,
                        bubble_rect,
                        CornerRadius::same(br.ceil() as u8),
                        bubble_fill,
                        Color32::TRANSPARENT,
                        ElevationTier::Md,
                        dark,
                    );
                    ui.painter().text(
                        bubble_center,
                        Align2::CENTER_CENTER,
                        ch.to_string(),
                        FontId::new(28.0 * bubble_t, FontFamily::Proportional),
                        p.primary_high.gamma_multiply(bubble_t),
                    );
                }
            }
        }

        picked
    }
}
