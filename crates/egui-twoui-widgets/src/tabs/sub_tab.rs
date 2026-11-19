use egui::{
    pos2, scroll_area::ScrollBarVisibility, vec2, CornerRadius, Id, Rect, Response, Role,
    ScrollArea, Sense, Ui, WidgetInfo,
};
use egui_twoui_tokens::{
    animate_toggle, ease_release, ease_sine_out_80, font_id, theme_from_ui,
    tokens::dimens, TwoUiFontWeight, TwoUiPalette,
};

/// Horizontal sub-tab pill bar (`40dp` height, `20dp` radius, per-tab fade + press spring,
/// and horizontal scroll when labels exceed width).
pub struct SubTabBar<'a> {
    id_salt: &'a str,
    tabs: &'a [&'a str],
    selected: &'a mut usize,
}

impl<'a> SubTabBar<'a> {
    pub fn new(id_salt: &'a str, tabs: &'a [&'a str], selected: &'a mut usize) -> Self {
        Self {
            id_salt,
            tabs,
            selected,
        }
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let count = self.tabs.len().max(1);
        *self.selected = (*self.selected).min(count - 1);

        let avail_w = ui.available_width().max(140.0);
        let height = dimens::SUBTAB_HEIGHT;
        let gap = 2.0;
        // Measure with SemiBold so switching selection never shifts tab slot widths.
        let measure_font = font_id(ui, 14.0, TwoUiFontWeight::SemiBold);

        let mut tab_widths = Vec::with_capacity(count);
        let mut sum_w = 0.0_f32;
        for (i, label) in self.tabs.iter().enumerate() {
            let g =
                ui.painter()
                    .layout_no_wrap((*label).to_owned(), measure_font.clone(), p.text_primary);
            let w = (g.size().x + 28.0).max(62.0);
            if i > 0 {
                sum_w += gap;
            }
            sum_w += w;
            tab_widths.push(w);
        }

        if sum_w + 8.0 > avail_w {
            let out = ScrollArea::horizontal()
                .id_salt((self.id_salt, "twoui_sub_scroll"))
                .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                .max_width(avail_w)
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    paint_sub_strip(
                        ui,
                        self.id_salt,
                        self.tabs,
                        self.selected,
                        sum_w + 8.0,
                        sum_w,
                        &tab_widths,
                        gap,
                        height,
                        p,
                    )
                });
            out.inner
        } else {
            paint_sub_strip(
                ui,
                self.id_salt,
                self.tabs,
                self.selected,
                avail_w,
                sum_w,
                &tab_widths,
                gap,
                height,
                p,
            )
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn paint_sub_strip(
    ui: &mut Ui,
    id_salt: &str,
    tabs: &[&str],
    selected: &mut usize,
    total_w: f32,
    sum_w: f32,
    tab_widths: &[f32],
    gap: f32,
    height: f32,
    p: &TwoUiPalette,
) -> Response {
    let (rect, mut response) = ui.allocate_exact_size(vec2(total_w, height), Sense::hover());
    let mut x = rect.left() + ((total_w - sum_w) * 0.5).max(4.0);
    let base_r = dimens::SUBTAB_RADIUS;

    for (i, label) in tabs.iter().enumerate() {
        let w = tab_widths[i];
        let pill_rect = Rect::from_min_size(pos2(x, rect.top()), vec2(w, height));
        let item_resp = ui.interact(
            pill_rect,
            Id::unique((id_salt, "twoui_sub", i)),
            Sense::click(),
        );
        if item_resp.clicked() && *selected != i {
            *selected = i;
            response.mark_changed();
        }
        let is_sel = *selected == i;
        item_resp.widget_info(|| WidgetInfo::selected(Role::Tab, true, is_sel, *label));

        if ui.is_rect_visible(pill_rect) {
            let pressed = item_resp.is_pointer_button_down_on();
            // Per-tab indicator: 100ms alpha fade + 250ms/350ms 0.95 scale spring.
            let pill_alpha = ui.ctx().animate_bool_with_time(
                Id::unique((id_salt, "sub_pill_a", i)),
                is_sel || pressed,
                0.10,
            );
            let scale_t = animate_toggle(
                ui,
                Id::unique((id_salt, "sub_pill_s", i)),
                pressed,
                if pressed { 0.25 } else { 0.35 },
                ease_release,
            );
            let scale = 1.0 - 0.05 * scale_t;
            let hover_t = animate_toggle(
                ui,
                Id::unique((id_salt, "sub_hover", i)),
                item_resp.hovered() && !pressed,
                0.15,
                ease_sine_out_80,
            );
            let ind_rect = Rect::from_center_size(pill_rect.center(), pill_rect.size() * scale);
            let scaled_r =
                CornerRadius::same(((f32::from(base_r) * scale).round() as u8).max(1));

            if hover_t > 0.001 && pill_alpha < 0.99 {
                ui.painter().rect_filled(
                    ind_rect,
                    scaled_r,
                    p.hover_overlay.gamma_multiply(hover_t * (1.0 - pill_alpha)),
                );
            }
            if pill_alpha > 0.001 {
                ui.painter().rect_filled(
                    ind_rect,
                    scaled_r,
                    p.surface_container.gamma_multiply(pill_alpha),
                );
            }

            let sel_t = animate_toggle(
                ui,
                Id::unique((id_salt, "sub_sel_c", i)),
                is_sel,
                0.15,
                ease_release,
            );
            let text_col = p
                .text_muted
                .lerp_to_gamma(p.text_primary, sel_t.max(hover_t * 0.45));
            let label_font = font_id(
                ui,
                14.0,
                if is_sel {
                    TwoUiFontWeight::SemiBold
                } else {
                    TwoUiFontWeight::Medium
                },
            );
            let g = ui
                .painter()
                .layout_no_wrap((*label).to_owned(), label_font, text_col);
            ui.painter().galley(
                pos2(
                    ind_rect.center().x - g.size().x * 0.5,
                    ind_rect.center().y - g.size().y * 0.5,
                ),
                g,
                text_col,
            );
        }
        x += w + gap;
    }

    response
}
