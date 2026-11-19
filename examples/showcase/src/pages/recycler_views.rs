//! Icons, Contacts, and AppPicker RecyclerViews showcase page.

use egui::{
    pos2, vec2, Align, Align2, Area, Color32, CornerRadius, FontFamily, FontId, Id, Layout,
    Margin, Order, RichText, Sense, Ui, Vec2,
};
use egui_twoui::{
    feedback::{interaction_factors, paint_state_overlay, recoil_rect},
    prelude::*,
};

use crate::{app::ShowcaseApp, data::CONTACTS};

impl ShowcaseApp {
    pub(crate) fn render_recyclerview_page(&mut self, ui: &mut Ui) {
        SubTabBar::new("rv_subtabs", &["Icons", "Contacts", "Apps"], &mut self.rv_tab_idx).show(ui);
        ui.add_space(10.0);

        match self.rv_tab_idx {
            0 => self.render_rv_icons_tab(ui),
            1 => self.render_rv_contacts_tab(ui),
            _ => self.render_rv_apps_tab(ui),
        }
    }

    fn render_rv_icons_tab(&mut self, ui: &mut Ui) {
        let had_query = !self.icon_search.is_empty();
        let (_, back) = TwoUiSearchBar::new(&mut self.icon_search)
            .hint(format!("Search {} icons...", TwoUiIcon::ALL.len()))
            .show_back_button(had_query)
            .show(ui);
        if back {
            self.icon_search.clear();
        }
        ui.add_space(8.0);

        let q = self.icon_search.trim().to_ascii_lowercase();
        let mut pairs: Vec<(usize, &'static str, TwoUiIcon)> = TwoUiIcon::ALL
            .iter()
            .enumerate()
            .map(|(idx, ic)| (idx, ic.name(), *ic))
            .filter(|(_, name, _)| q.is_empty() || name.to_ascii_lowercase().contains(&q))
            .collect();
        pairs.sort_by_key(|(_, name, _)| *name);

        let letters: Vec<char> = ('A'..='Z').collect();
        let view_rect = ui.clip_rect();
        let sticky_id = Id::unique("icons_az_jump_pending");
        let mut jump_letter = ui
            .ctx()
            .data(|d| d.get_temp::<Option<char>>(sticky_id))
            .flatten();

        let list_w = (ui.available_width() - 48.0).max(180.0);
        let mut list_right = view_rect.right() - 56.0;
        ui.vertical(|ui| {
            ui.set_width(list_w);
            CardSection::new().show(ui, |ui| {
                let theme = theme_from_ui(ui);
                let target_row_idx = jump_letter.and_then(|ch| {
                    if pairs.is_empty() {
                        None
                    } else {
                        Some(
                            pairs
                                .iter()
                                .position(|(_, name, _)| {
                                    name.chars().next().map(|c| c.to_ascii_uppercase()) >= Some(ch)
                                })
                                .unwrap_or(pairs.len() - 1),
                        )
                    }
                });
                for (row_idx, (orig_idx, name, icon)) in pairs.iter().enumerate() {
                    if row_idx > 0 {
                        paint_divider(ui, DividerInset::WithLeading);
                    }
                    let is_sel = self.icon_selected[*orig_idx];
                    let is_jump_match = self.active_index_letter.is_some_and(|ch| {
                        name.chars().next().map(|c| c.to_ascii_uppercase()) == Some(ch)
                    });
                    let row_resp = ui
                        .horizontal(|ui| {
                            ui.add_space(16.0);
                            icon.show(ui, 24.0, theme.palette.primary_high);
                            ui.add_space(12.0);
                            ui.label(
                                RichText::new(*name)
                                    .size(15.0)
                                    .color(if is_jump_match {
                                        theme.palette.primary_high
                                    } else {
                                        theme.palette.text_primary
                                    }),
                            );
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                ui.add_space(12.0);
                                let mut checked = is_sel;
                                if ui.add(TwoUiCheckBox::new(&mut checked)).changed() {
                                    self.icon_selected[*orig_idx] = checked;
                                }
                            });
                        })
                        .response;
                    if target_row_idx == Some(row_idx) {
                        row_resp.scroll_to_me(Some(Align::Center));
                    }
                }
                if jump_letter.is_some() {
                    jump_letter = None;
                }
            });
            list_right = ui.min_rect().right();
        });
        ui.ctx().data_mut(|d| d.insert_temp(sticky_id, jump_letter));

        let rail_top = view_rect.top() + 68.0;
        let rail_bot = (view_rect.bottom() - 88.0).max(rail_top + 160.0);
        let rail_h = (rail_bot - rail_top).clamp(160.0, 440.0);
        let rail_x = (list_right + 10.0)
            .min(view_rect.right() - 56.0)
            .max(view_rect.left() + 120.0);
        let rail_pos = pos2(rail_x, (rail_top + rail_bot - rail_h) * 0.5);
        Area::new(Id::unique("icons_az_sticky_rail"))
            .fixed_pos(rail_pos)
            .movable(false)
            .constrain(false)
            .order(Order::Middle)
            .show(ui.ctx(), |ui| {
                ui.set_height(rail_h);
                if let Some(ch) = TwoUiIndexScroll::new("icons_az_rail", &letters)
                    .active_letter(self.active_index_letter)
                    .show(ui)
                {
                    let changed = self.active_index_letter != Some(ch);
                    self.active_index_letter = Some(ch);
                    ui.ctx().data_mut(|d| d.insert_temp(sticky_id, Some(ch)));
                    if changed {
                        self.toast
                            .show_message(ui.ctx(), format!("Index -> {ch}"), 1.1);
                    }
                }
            });
        ui.add_space(72.0);
    }

    fn render_rv_contacts_tab(&mut self, ui: &mut Ui) {
        let now = ui.input(|i| i.time);
        let refreshing = self.contacts_refresh_until.is_some_and(|t| now < t);
        if refreshing {
            ui.ctx().request_repaint();
        } else {
            self.contacts_refresh_until = None;
        }

        ui.horizontal(|ui| {
            if ui.add(TwoUiButton::new("Simulate Refresh")).clicked() {
                self.contacts_refresh_until = Some(now + 1.6);
            }
            if refreshing {
                ui.add_space(12.0);
                ui.add(TwoUiSpinner::indeterminate(SpinnerSize::Small));
                ui.label("Refreshing stargazers...");
            }
        });
        ui.add_space(12.0);

        CardSection::new().show(ui, |ui| {
            let p = theme_from_ui(ui).palette;
            let row_w = ui.available_width().max(160.0);
            for (idx, c) in CONTACTS.iter().enumerate() {
                if idx > 0 {
                    paint_divider(ui, DividerInset::WithLeading);
                }
                let (row_r, resp) = ui.allocate_exact_size(vec2(row_w, 60.0), Sense::click());
                if ui.is_rect_visible(row_r) {
                    let (press_t, hover_t) = interaction_factors(ui, &resp);
                    let paint_r = recoil_rect(row_r, press_t, 1.0);
                    paint_state_overlay(
                        ui,
                        paint_r.shrink2(vec2(6.0, 2.0)),
                        CornerRadius::same(20),
                        &p,
                        hover_t,
                        press_t,
                    );

                    let av_center = pos2(paint_r.left() + 38.0, paint_r.center().y);
                    ui.painter().circle_filled(av_center, 18.0, c.color);
                    let initial = c
                        .name
                        .chars()
                        .next()
                        .unwrap_or('?')
                        .to_ascii_uppercase()
                        .to_string();
                    ui.painter().text(
                        av_center,
                        Align2::CENTER_CENTER,
                        initial,
                        FontId::new(15.0, FontFamily::Proportional),
                        Color32::WHITE,
                    );

                    let name_g = ui.painter().layout_no_wrap(
                        c.name.to_owned(),
                        FontId::new(15.5, FontFamily::Proportional),
                        p.text_primary,
                    );
                    let sub_g = ui.painter().layout_no_wrap(
                        c.subtitle.to_owned(),
                        FontId::new(12.5, FontFamily::Proportional),
                        p.text_secondary,
                    );
                    let text_x = av_center.x + 18.0 + 16.0;
                    let block_h = name_g.size().y + 2.0 + sub_g.size().y;
                    let top_y = paint_r.center().y - block_h * 0.5;
                    let name_h = name_g.size().y;
                    ui.painter()
                        .galley(pos2(text_x, top_y), name_g, p.text_primary);
                    ui.painter()
                        .galley(pos2(text_x, top_y + name_h + 2.0), sub_g, p.text_secondary);
                }
                if resp.clicked() {
                    self.toast
                        .show_message(ui.ctx(), format!("Opened profile: {}", c.name), 2.0);
                }
            }
        });
    }

    fn render_rv_apps_tab(&mut self, ui: &mut Ui) {
        let picker_modes = ["List", "CheckBox", "Switch", "Grid"];
        SubTabBar::new("app_picker_modes", &picker_modes, &mut self.app_picker_mode_idx).show(ui);
        ui.add_space(10.0);

        if self.app_picker_mode_idx == 3 {
            let now = ui.input(|i| i.time);
            CardSection::new()
                .inner_margin(Margin::symmetric(16, 16))
                .show(ui, |ui| {
                    let p = theme_from_ui(ui).palette;
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing = vec2(14.0, 14.0);
                        for app in &mut self.apps {
                            let (cell_r, resp) =
                                ui.allocate_exact_size(vec2(104.0, 104.0), Sense::click());
                            if resp.clicked() {
                                app.selected = !app.selected;
                            }
                            if ui.is_rect_visible(cell_r) {
                                let (pt, ht) = interaction_factors(ui, &resp);
                                let paint_r = recoil_rect(cell_r, pt, 1.2);
                                let bg = if app.selected {
                                    p.primary.gamma_multiply(0.18)
                                } else {
                                    p.surface_container
                                };
                                ui.painter()
                                    .rect_filled(paint_r, CornerRadius::same(18), bg);
                                paint_state_overlay(
                                    ui,
                                    paint_r,
                                    CornerRadius::same(18),
                                    &p,
                                    ht,
                                    pt,
                                );
                                if app.selected {
                                    let badge_c = pos2(paint_r.right() - 14.0, paint_r.top() + 14.0);
                                    ui.painter().circle_filled(badge_c, 7.5, p.primary);
                                    TwoUiIcon::Check.paint(
                                        ui,
                                        egui::Rect::from_center_size(badge_c, Vec2::splat(9.5)),
                                        p.on_primary,
                                    );
                                }
                                app.icon.paint(
                                    ui,
                                    egui::Rect::from_center_size(
                                        paint_r.center() - vec2(0.0, 11.0),
                                        Vec2::splat(32.0),
                                    ),
                                    p.primary_high,
                                );

                                let base_clip = egui::Rect::from_min_max(
                                    pos2(cell_r.left() + 8.0, paint_r.bottom() - 29.0),
                                    pos2(cell_r.right() - 8.0, paint_r.bottom() - 8.0),
                                );
                                let galley = ui.painter().layout_no_wrap(
                                    app.name.to_owned(),
                                    FontId::new(11.5, FontFamily::Proportional),
                                    p.text_primary,
                                );
                                let text_w = galley.size().x;
                                let text_h = galley.size().y;
                                let avail_w = base_clip.width().max(1.0);
                                let text_x = if text_w <= avail_w {
                                    base_clip.center().x - text_w * 0.5
                                } else {
                                    ui.ctx().request_repaint();
                                    let overflow = text_w - avail_w;
                                    let speed = 26.0_f64;
                                    let travel_t = (f64::from(overflow) / speed).max(0.1);
                                    let pause_t = 1.0_f64;
                                    let half_cycle = pause_t + travel_t + pause_t;
                                    let phase = now % (half_cycle * 2.0);
                                    let u = if phase < half_cycle {
                                        ((phase - pause_t) / travel_t).clamp(0.0, 1.0)
                                    } else {
                                        let rev = phase - half_cycle;
                                        (1.0 - (rev - pause_t) / travel_t).clamp(0.0, 1.0)
                                    };
                                    base_clip.left() - overflow * (u as f32)
                                };
                                let clip_rect = base_clip.intersect(paint_r.shrink2(vec2(6.0, 4.0)));
                                ui.painter().with_clip_rect(clip_rect).galley(
                                    pos2(text_x, base_clip.center().y - text_h * 0.5),
                                    galley,
                                    p.text_primary,
                                );
                            }
                        }
                    });
                });
        } else {
            CardSection::new().show(ui, |ui| {
                let p = theme_from_ui(ui).palette;
                for (idx, app) in self.apps.iter_mut().enumerate() {
                    if idx > 0 {
                        paint_divider(ui, DividerInset::WithLeading);
                    }
                    ui.allocate_ui_with_layout(
                        vec2(ui.available_width().max(160.0), 60.0),
                        Layout::left_to_right(Align::Center),
                        |ui| {
                            ui.add_space(16.0);
                            app.icon.show(ui, 26.0, p.primary_high);
                            ui.add_space(12.0);
                            let text_w = (ui.available_width() - 78.0).max(60.0);
                            let (tr, _) = ui.allocate_exact_size(vec2(text_w, 48.0), Sense::hover());
                            let name_g = ui.painter().layout_no_wrap(
                                app.name.to_owned(),
                                FontId::new(15.0, FontFamily::Proportional),
                                p.text_primary,
                            );
                            let pkg_g = ui.painter().layout_no_wrap(
                                app.package.to_owned(),
                                FontId::new(12.0, FontFamily::Proportional),
                                p.text_muted,
                            );
                            let bh = name_g.size().y + 2.0 + pkg_g.size().y;
                            let ty = tr.center().y - bh * 0.5;
                            let nh = name_g.size().y;
                            ui.painter()
                                .with_clip_rect(tr)
                                .galley(pos2(tr.left(), ty), name_g, p.text_primary);
                            ui.painter().with_clip_rect(tr).galley(
                                pos2(tr.left(), ty + nh + 2.0),
                                pkg_g,
                                p.text_muted,
                            );
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                ui.add_space(14.0);
                                match self.app_picker_mode_idx {
                                    1 => {
                                        ui.add(TwoUiCheckBox::new(&mut app.selected));
                                    }
                                    2 => {
                                        ui.add(TwoUiSwitch::new(&mut app.selected));
                                    }
                                    _ => {}
                                }
                            });
                        },
                    );
                }
            });
        }
    }
}
