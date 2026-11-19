//! Collapsing AppBar scaffold with floating toolbar capsules, CSD window controls, and fast scrollbar.

mod scrollbar;
mod toolbar;
mod window_controls;

pub use window_controls::apply_borderless_window_resize;

use egui::{
    pos2, scroll_area::ScrollBarVisibility, vec2, Align, Align2, Area, Id, Layout, Order, Rect,
    ScrollArea, Sense, Ui,
};
use egui_twoui_tokens::{
    animate_toggle, dimens, ease_sine_out_80, font_id, theme_from_ui, TwoUiFontWeight,
};

/// Leading navigation button rendered on the left of the compact toolbar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScaffoldNavButton {
    #[default]
    None,
    Back,
    DrawerMenu,
}

/// Output returned by [`TwoUiScaffold::show`].
pub struct ScaffoldResponse {
    pub nav_clicked: bool,
}

/// Collapsing AppBar scaffold with floating toolbar pills, centered body column, and optional window controls.
pub struct TwoUiScaffold<'a> {
    id_salt: &'a str,
    title: &'a str,
    subtitle: Option<&'a str>,
    nav_button: ScaffoldNavButton,
    max_content_width: Option<f32>,
    extended_height: f32,
    show_window_controls: bool,
}

impl<'a> TwoUiScaffold<'a> {
    pub fn new(id_salt: &'a str, title: &'a str) -> Self {
        Self {
            id_salt,
            title,
            subtitle: None,
            nav_button: ScaffoldNavButton::None,
            max_content_width: Some(840.0),
            extended_height: dimens::EXTENDED_APPBAR_EXTRA_HEIGHT,
            show_window_controls: false,
        }
    }

    pub fn subtitle(mut self, subtitle: impl Into<Option<&'a str>>) -> Self {
        self.subtitle = subtitle.into();
        self
    }

    pub fn nav_button(mut self, nav: ScaffoldNavButton) -> Self {
        self.nav_button = nav;
        self
    }

    /// Sets max column width for content cards inside wide desktop windows (`None` = full width).
    pub fn max_content_width(mut self, max_width: Option<f32>) -> Self {
        self.max_content_width = max_width;
        self
    }

    pub fn extended_height(mut self, height: f32) -> Self {
        self.extended_height = height.clamp(0.0, 260.0);
        self
    }

    /// Integrates borderless desktop window dragging, double-click maximize/restore,
    /// and Minimize / Maximize-Restore / Close buttons inside the trailing toolbar capsule.
    pub fn window_controls(mut self, enabled: bool) -> Self {
        self.show_window_controls = enabled;
        self
    }

    /// Renders the scrollable centered scaffold with three-phase AppBar transitions
    /// (`Expanded` -> `Collapsed` -> floating toolbar pills).
    pub fn show(
        self,
        ui: &mut Ui,
        toolbar_actions: impl FnOnce(&mut Ui),
        body: impl FnOnce(&mut Ui),
    ) -> ScaffoldResponse {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let dark = theme.mode.is_dark();

        let scroll_mem_id = Id::unique((self.id_salt, "twoui_appbar_scroll"));
        let force_scroll_id = Id::unique((self.id_salt, "twoui_force_scroll_y"));
        let forced_scroll_y: Option<f32> = ui
            .ctx()
            .data_mut(|d| d.get_temp::<Option<f32>>(force_scroll_id))
            .flatten();
        if forced_scroll_y.is_some() {
            ui.ctx()
                .data_mut(|d| d.insert_temp::<Option<f32>>(force_scroll_id, None));
        }
        let prev_scroll_y: f32 = forced_scroll_y.unwrap_or_else(|| {
            ui.ctx()
                .data(|d| d.get_temp::<f32>(scroll_mem_id))
                .unwrap_or(0.0)
        });

        let header_h = dimens::TOOLBAR_HEIGHT;
        let top_inset = dimens::TOOLBAR_TOP_INSET;
        let row_h = dimens::TOOLBAR_ROW_HEIGHT;
        let ext_h = self.extended_height;

        let collapse_t = if ext_h <= 1.0 {
            1.0
        } else {
            (prev_scroll_y / ext_h).clamp(0.0, 1.0)
        };
        let hide_threshold = if ext_h > 1.0 { ext_h + 28.0 } else { 44.0 };
        let appbar_hidden = prev_scroll_y >= hide_threshold;
        let projection_t = animate_toggle(
            ui,
            Id::unique((self.id_salt, "twoui_proj_alpha")),
            appbar_hidden,
            0.18,
            ease_sine_out_80,
        );
        if matches!(self.nav_button, ScaffoldNavButton::DrawerMenu) {
            let sync_id = crate::toolbar_projection_state_id();
            let prev_sync: f32 = ui.ctx().data(|d| d.get_temp::<f32>(sync_id)).unwrap_or(0.0);
            if (prev_sync - projection_t).abs() > 0.0005 {
                ui.ctx().request_repaint();
            }
            ui.ctx().data_mut(|d| d.insert_temp(sync_id, projection_t));
        }
        let header_bar_vis = animate_toggle(
            ui,
            Id::unique((self.id_salt, "twoui_hdr_bar_vis")),
            collapse_t > 0.72 && !appbar_hidden,
            0.16,
            ease_sine_out_80,
        );
        let compact_title_alpha = if ext_h <= 1.0 {
            if appbar_hidden { 0.0 } else { 1.0 }
        } else {
            ((collapse_t - 0.62) / 0.34).clamp(0.0, 1.0) * (1.0 - projection_t)
        };

        let avail = ui.available_rect_before_wrap();
        let header_rect = Rect::from_min_size(avail.min, vec2(avail.width().max(160.0), header_h));

        let max_w = self.max_content_width;
        let title_str = self.title;
        let sub_str = self.subtitle;
        let rail_inset = if matches!(self.nav_button, ScaffoldNavButton::DrawerMenu) {
            ui.ctx()
                .data(|d| d.get_temp::<f32>(crate::compact_rail_inset_id()))
                .unwrap_or(0.0)
                .clamp(0.0, 120.0)
        } else {
            0.0
        };

        let mut scroll_area = ScrollArea::vertical()
            .id_salt((self.id_salt, "twoui_body_scroll"))
            .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
            .auto_shrink([false, false]);
        if let Some(target_y) = forced_scroll_y {
            scroll_area = scroll_area.vertical_scroll_offset(target_y);
        }
        let scroll_out = scroll_area.show(ui, |ui| {
            let full_w = ui.available_width().max(120.0);
            let margin = dimens::SECTION_HORIZONTAL_MARGIN;
            let left_guard = margin.max(rail_inset);
            let max_span_w = (full_w - left_guard - margin).max(120.0);
            let col_w = match max_w {
                Some(mw) => max_span_w.min(mw),
                None => max_span_w,
            };
            let col_left = ((full_w - col_w) * 0.5).max(left_guard);

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                if col_left > 0.5 {
                    ui.add_space(col_left);
                }
                ui.allocate_ui_with_layout(
                    vec2(col_w, 0.0),
                    Layout::top_down(Align::Center),
                    |ui| {
                        ui.set_width(col_w);
                        ui.add_space(top_inset + row_h + 6.0);

                        if ext_h > 1.0 {
                            let header_alpha = (1.0 - collapse_t * 1.45).clamp(0.0, 1.0);
                            let (ext_rect, _) =
                                ui.allocate_exact_size(vec2(col_w, ext_h), Sense::hover());
                            if ui.is_rect_visible(ext_rect) && header_alpha > 0.01 {
                                let t_col = p.text_primary.gamma_multiply(header_alpha);
                                let tg = ui.painter().layout(
                                    title_str.to_owned(),
                                    font_id(
                                        ui,
                                        dimens::TEXT_EXTENDED_TITLE,
                                        TwoUiFontWeight::Display,
                                    ),
                                    t_col,
                                    col_w - 32.0,
                                );
                                let sub_g = sub_str.map(|s| {
                                    let s_col = p.text_muted.gamma_multiply(header_alpha);
                                    (
                                        ui.painter().layout(
                                            s.to_owned(),
                                            font_id(ui, 15.0, TwoUiFontWeight::Regular),
                                            s_col,
                                            col_w - 32.0,
                                        ),
                                        s_col,
                                    )
                                });
                                let sub_h =
                                    sub_g.as_ref().map_or(0.0, |(g, _)| g.size().y + 8.0);
                                let total_h = tg.size().y + sub_h;
                                let base_y = ext_rect.center().y - total_h * 0.5;
                                let tg_h = tg.size().y;
                                ui.painter().galley(
                                    pos2(ext_rect.center().x - tg.size().x * 0.5, base_y),
                                    tg,
                                    t_col,
                                );
                                if let Some((sg, sc)) = sub_g {
                                    ui.painter().galley(
                                        pos2(
                                            ext_rect.center().x - sg.size().x * 0.5,
                                            base_y + tg_h + 8.0,
                                        ),
                                        sg,
                                        sc,
                                    );
                                }
                            }
                        }

                        body(ui);
                        ui.add_space(28.0);
                    },
                );
            });
        });

        let current_scroll_y = scroll_out.state.clamped_offset().y;
        let max_scroll = (scroll_out.content_size.y - scroll_out.inner_rect.height()).max(0.0);
        ui.ctx()
            .data_mut(|d| d.insert_temp(scroll_mem_id, current_scroll_y));

        scrollbar::paint_twoui_fast_scrollbar(
            ui,
            self.id_salt,
            avail,
            header_h,
            scroll_out.content_size.y,
            scroll_out.inner_rect.height(),
            current_scroll_y,
            max_scroll,
            prev_scroll_y,
            force_scroll_id,
            scroll_mem_id,
            p,
            dark,
        );

        let nav_clicked = toolbar::render_sticky_toolbar(
            ui,
            self.id_salt,
            title_str,
            self.nav_button,
            self.show_window_controls,
            header_rect,
            top_inset,
            row_h,
            projection_t,
            header_bar_vis,
            compact_title_alpha,
            p,
            dark,
            toolbar_actions,
        );

        ScaffoldResponse { nav_clicked }
    }
}

/// Positions a floating bottom container (e.g.
/// [`egui_twoui_widgets::TwoUiDividerButtonLayout`],
/// [`egui_twoui_widgets::BottomNavBar`], [`egui_twoui_widgets::MainTabBar`],
/// or floating search pill) centered above the bottom edge of `ui`.
pub fn show_bottom_floating_bar(
    ui: &Ui,
    bottom_offset: f32,
    id: Id,
    add_contents: impl FnOnce(&mut Ui),
) {
    let host_rect = ui.max_rect();
    let rail_inset = ui
        .ctx()
        .data(|d| d.get_temp::<f32>(crate::compact_rail_inset_id()))
        .unwrap_or(0.0)
        .clamp(0.0, 120.0);
    let max_bar_w = (host_rect.width() - rail_inset.max(16.0) - 16.0).max(200.0);
    let expected_w = max_bar_w.min(dimens::FLOATING_NAV_MAX_WIDTH);
    let sym_left = host_rect.center().x - expected_w * 0.5;
    let guard_shift = (host_rect.left() + rail_inset - sym_left).max(0.0);
    let pane_offset_x = host_rect.center().x + guard_shift - ui.ctx().content_rect().center().x;
    Area::new(id)
        .order(Order::Foreground)
        .anchor(Align2::CENTER_BOTTOM, vec2(pane_offset_x, -bottom_offset))
        .pivot(Align2::CENTER_BOTTOM)
        .show(ui.ctx(), |ui| {
            ui.set_max_width(max_bar_w);
            add_contents(ui);
        });
}
