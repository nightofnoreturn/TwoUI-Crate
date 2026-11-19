use egui::{pos2, vec2, Color32, CornerRadius, Rect, Ui};
use egui_twoui_tokens::theme_from_ui;

/// Notification badge pill (`N` or numeric count).
pub fn paint_badge(ui: &Ui, center: egui::Pos2, text: &str) {
    let theme = theme_from_ui(ui);
    let p = &theme.palette;
    let font_id = egui::FontId::new(10.5, egui::FontFamily::Proportional);
    let galley = ui
        .painter()
        .layout_no_wrap(text.to_owned(), font_id, Color32::WHITE);
    let w = (galley.size().x + 8.0).max(17.0);
    let h = 17.0;
    let rect = Rect::from_center_size(center, vec2(w, h));
    ui.painter()
        .rect_filled(rect, CornerRadius::same(9), p.badge_bg);
    ui.painter().galley(
        pos2(center.x - galley.size().x * 0.5, center.y - galley.size().y * 0.5),
        galley,
        Color32::WHITE,
    );
}
