//! Anchored bottom sheet modal (`26dp` top corners, `44x4dp` drag handle).

use egui::{
    vec2, Align2, Area, Color32, Context, CornerRadius, Frame, Id, Margin, Modal, Order, Rect,
    RichText, ScrollArea, Sense, Stroke, Ui,
};
use egui_twoui_tokens::{
    animate_toggle_ctx, dimens, ease_sine_out_80, elevation_shadow, font_id_for,
    theme_from_ctx, ElevationTier, TwoUiFontWeight,
};

/// Anchored bottom sheet modal (`26dp` top corners, elevated surface, `44x4dp` drag handle).
pub struct TwoUiBottomSheet<'a> {
    id_salt: &'a str,
    title: &'a str,
    max_width: f32,
    max_height: f32,
}

impl<'a> TwoUiBottomSheet<'a> {
    pub fn new(id_salt: &'a str, title: &'a str) -> Self {
        Self {
            id_salt,
            title,
            max_width: 560.0,
            max_height: 440.0,
        }
    }

    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = width;
        self
    }

    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = height;
        self
    }

    pub fn show(self, ctx: &Context, open: &mut bool, content: impl FnOnce(&mut Ui)) {
        let slide_id = Id::unique((self.id_salt, "twoui_sheet_y"));
        let alpha_id = Id::unique((self.id_salt, "twoui_sheet_a"));
        if !*open {
            ctx.animate_bool_with_time(slide_id, false, 0.0);
            ctx.animate_bool_with_time(alpha_id, false, 0.0);
            return;
        }

        let slide_t = animate_toggle_ctx(ctx, slide_id, true, 0.30, ease_sine_out_80);
        let alpha_t = animate_toggle_ctx(ctx, alpha_id, true, 0.15, ease_sine_out_80);
        let theme = theme_from_ctx(ctx);
        let dark = theme.mode.is_dark();
        let p = &theme.palette;

        let sheet_w = (ctx.content_rect().width() - 20.0)
            .min(self.max_width)
            .max(280.0);
        let mut sheet_frame = Frame::NONE
            .fill(p.surface_elevated.gamma_multiply(alpha_t))
            .corner_radius(CornerRadius {
                nw: dimens::DIALOG_CORNER_RADIUS,
                ne: dimens::DIALOG_CORNER_RADIUS,
                sw: 0,
                se: 0,
            })
            .stroke(Stroke::new(0.8, p.floating_stroke.gamma_multiply(alpha_t)))
            .inner_margin(Margin {
                left: 0,
                right: 0,
                top: 10,
                bottom: 20,
            });
        if alpha_t > 0.3 {
            sheet_frame = sheet_frame.shadow(elevation_shadow(ElevationTier::Lg, dark));
        }

        let area = Area::new(Id::unique(self.id_salt))
            .anchor(Align2::CENTER_BOTTOM, vec2(0.0, (1.0 - slide_t) * 56.0))
            .order(Order::Foreground);

        let modal_resp = Modal::new(Id::unique((self.id_salt, "modal")))
            .area(area)
            .backdrop_color(p.backdrop.gamma_multiply(alpha_t))
            .frame(sheet_frame)
            .show(ctx, |ui| {
                ui.multiply_opacity(alpha_t);
                ui.set_width(sheet_w);
                ui.set_max_height(self.max_height);

                let (h_rect, _) = ui.allocate_exact_size(vec2(sheet_w, 16.0), Sense::hover());
                let handle = Rect::from_center_size(h_rect.center(), vec2(44.0, 4.0));
                let handle_col = Color32::from_rgba_unmultiplied(0x84, 0x84, 0x87, 128);
                ui.painter()
                    .rect_filled(handle, CornerRadius::same(2), handle_col);

                Frame::NONE
                    .inner_margin(Margin::symmetric(24, 6))
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new(self.title)
                                .font(font_id_for(
                                    ui.ctx(),
                                    dimens::TEXT_DIALOG_TITLE,
                                    TwoUiFontWeight::SemiBold,
                                ))
                                .color(p.text_primary),
                        );
                        ui.add_space(10.0);
                        ScrollArea::vertical()
                            .id_salt((self.id_salt, "sheet_scroll"))
                            .max_height((self.max_height - 92.0).max(120.0))
                            .show(ui, content);
                    });
            });

        if modal_resp.backdrop_response.clicked() || ctx.input(|i| i.key_pressed(egui::Key::Escape))
        {
            *open = false;
        }
        if !*open {
            ctx.animate_bool_with_time(slide_id, false, 0.0);
            ctx.animate_bool_with_time(alpha_id, false, 0.0);
        }
    }
}
