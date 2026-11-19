//! Root application state, drawer/scaffold wiring, and floating bottom bars for [`ShowcaseApp`].

use egui::{Color32, Frame, Id, Pos2, Ui};
use egui_twoui::prelude::*;

use crate::data::{sample_apps, AppRowItem, Page, ACCENT_PRESETS};

pub(crate) struct ShowcaseApp {
    pub(crate) page_id: usize,
    pub(crate) drawer_open: bool,

    // Theme & typography controls
    pub(crate) dark_mode: bool,
    pub(crate) follow_system: bool,
    pub(crate) custom_accent: Color32,
    pub(crate) accent_index: usize,
    pub(crate) typography_idx: usize,
    pub(crate) color_dialog_open: bool,
    pub(crate) color_dialog_draft: Color32,

    // ProgressBar page state
    pub(crate) auto_animate_progress: bool,
    pub(crate) gradient_progress_mode: bool,
    pub(crate) determinate_progress: f32,

    // SeekBar page state
    pub(crate) seek_standard: f32,
    pub(crate) seek_expand: f32,
    pub(crate) seek_overlap: f32,
    pub(crate) seek_level_stepped: f32,
    pub(crate) seek_level_seamless: f32,
    pub(crate) seek_plus_seamless: f32,
    pub(crate) seek_plus_stepped: f32,
    pub(crate) seek_vert_standard: f32,
    pub(crate) seek_vert_expand: f32,
    pub(crate) seek_pro: f32,
    pub(crate) seek_pro_xp: f32,
    pub(crate) seek_pro_level: f32,

    // Pickers state
    pub(crate) picker_type_idx: usize,
    pub(crate) num_p1: i32,
    pub(crate) num_p2: i32,
    pub(crate) num_p3: i32,
    pub(crate) time_hour: u8,
    pub(crate) time_min: u8,
    pub(crate) time_24h: bool,
    pub(crate) date_year: i32,
    pub(crate) date_month: u8,
    pub(crate) date_day: u8,
    pub(crate) date_spinner_mode: bool,
    pub(crate) bedtime_min: i32,
    pub(crate) wakeup_min: i32,
    pub(crate) inline_picker_color: Color32,
    pub(crate) date_dialog_open: bool,
    pub(crate) time_dialog_open: bool,
    pub(crate) sleep_dialog_open: bool,

    // QRCode state
    pub(crate) qr_text: String,

    // Navigation & Overlays state
    pub(crate) main_tab_idx: usize,
    pub(crate) sub_tab_idx: usize,
    pub(crate) bottom_nav_icon_idx: usize,
    pub(crate) bottom_nav_icon_only_idx: usize,
    pub(crate) bottom_tab_idx: usize,
    pub(crate) div_btn_selected: usize,
    pub(crate) dialog_open: bool,
    pub(crate) dialog_dont_show_again: bool,
    pub(crate) suggestive_dialog_open: bool,
    pub(crate) bottom_sheet_open: bool,
    pub(crate) licenses_sheet_open: bool,
    pub(crate) tip_popup_open: bool,
    pub(crate) tip_anchor: Pos2,

    // RecyclerViews state
    pub(crate) rv_tab_idx: usize,
    pub(crate) icon_search: String,
    pub(crate) icon_selected: Vec<bool>,
    pub(crate) active_index_letter: Option<char>,
    pub(crate) contacts_refresh_until: Option<f64>,
    pub(crate) app_picker_mode_idx: usize,
    pub(crate) apps: Vec<AppRowItem>,

    // Widgets page state
    pub(crate) master_switch: bool,
    pub(crate) master_busy_until: Option<f64>,
    pub(crate) compound_switch: bool,
    pub(crate) compound_check: bool,
    pub(crate) compound_radio: usize,
    pub(crate) chip_selected: [bool; 4],
    pub(crate) dropdown_idx: usize,
    pub(crate) search_query: String,
    pub(crate) floating_search_query: String,
    pub(crate) separate_switch_val: bool,
    pub(crate) icon_switch_val: bool,
    pub(crate) card_radio_idx: usize,
    pub(crate) progress_btn_busy_until: Option<f64>,

    // Preferences page state
    pub(crate) show_tips_card: bool,
    pub(crate) show_suggestion_card: bool,
    pub(crate) suggestion_page: usize,
    pub(crate) wifi_enabled: bool,
    pub(crate) bt_enabled: bool,
    pub(crate) notifications_enabled: bool,
    pub(crate) pref_check: bool,
    pub(crate) pref_text: String,
    pub(crate) pref_list_idx: usize,

    // AppInfo state
    pub(crate) custom_about_style: bool,
    pub(crate) update_state: AppUpdateState,
    pub(crate) update_check_until: Option<f64>,

    // Global overlays
    pub(crate) toast: TwoUiToastState,
    pub(crate) snackbar: TwoUiSnackBarState,
}

impl ShowcaseApp {
    pub(crate) fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let initial_accent = ACCENT_PRESETS[0].1;
        let initial_mode = detect_system_mode(&cc.egui_ctx, true).unwrap_or(TwoUiMode::Dark);
        install_themes_with_preference(
            &cc.egui_ctx,
            Some(initial_accent),
            initial_mode,
            egui::ThemePreference::System,
        );

        Self {
            page_id: 0,
            drawer_open: true,

            dark_mode: initial_mode.is_dark(),
            follow_system: true,
            custom_accent: initial_accent,
            accent_index: 0,
            typography_idx: 0,
            color_dialog_open: false,
            color_dialog_draft: initial_accent,

            auto_animate_progress: true,
            gradient_progress_mode: true,
            determinate_progress: 0.42,

            seek_standard: 45.0,
            seek_expand: 62.0,
            seek_overlap: 82.0,
            seek_level_stepped: 15.0,
            seek_level_seamless: 4.2,
            seek_plus_seamless: 10.0,
            seek_plus_stepped: 1.0,
            seek_vert_standard: 58.0,
            seek_vert_expand: 72.0,
            seek_pro: 50.0,
            seek_pro_xp: 65.0,
            seek_pro_level: 6.0,

            picker_type_idx: 0,
            num_p1: 24,
            num_p2: 8,
            num_p3: 2026,
            time_hour: 10,
            time_min: 30,
            time_24h: false,
            date_year: 2026,
            date_month: 11,
            date_day: 17,
            date_spinner_mode: false,
            bedtime_min: 23 * 60,
            wakeup_min: 7 * 60,
            inline_picker_color: Color32::from_rgb(0x03, 0x81, 0xFE),
            date_dialog_open: false,
            time_dialog_open: false,
            sleep_dialog_open: false,

            qr_text: "https://github.com/nightofnoreturn/TwoUI-Crate".to_owned(),

            main_tab_idx: 0,
            sub_tab_idx: 1,
            bottom_nav_icon_idx: 0,
            bottom_nav_icon_only_idx: 1,
            bottom_tab_idx: 0,
            div_btn_selected: 0,
            dialog_open: false,
            dialog_dont_show_again: false,
            suggestive_dialog_open: false,
            bottom_sheet_open: false,
            licenses_sheet_open: false,
            tip_popup_open: false,
            tip_anchor: Pos2::new(360.0, 320.0),

            rv_tab_idx: 0,
            icon_search: String::new(),
            icon_selected: vec![false; TwoUiIcon::ALL.len()],
            active_index_letter: None,
            contacts_refresh_until: None,
            app_picker_mode_idx: 1,
            apps: sample_apps(),

            master_switch: true,
            master_busy_until: None,
            compound_switch: true,
            compound_check: true,
            compound_radio: 0,
            chip_selected: [true, false, true, false],
            dropdown_idx: 0,
            search_query: String::new(),
            floating_search_query: String::new(),
            separate_switch_val: true,
            icon_switch_val: false,
            card_radio_idx: 0,
            progress_btn_busy_until: None,

            show_tips_card: true,
            show_suggestion_card: true,
            suggestion_page: 0,
            wifi_enabled: true,
            bt_enabled: false,
            notifications_enabled: true,
            pref_check: true,
            pref_text: "TwoUI Desktop".to_owned(),
            pref_list_idx: 0,

            custom_about_style: false,
            update_state: AppUpdateState::UpdateAvailable,
            update_check_until: None,

            toast: TwoUiToastState::default(),
            snackbar: TwoUiSnackBarState::default(),
        }
    }

    pub(crate) fn sync_theme(&self, ctx: &egui::Context) {
        let mode = if self.dark_mode {
            TwoUiMode::Dark
        } else {
            TwoUiMode::Light
        };
        let preference = if self.follow_system {
            egui::ThemePreference::System
        } else if self.dark_mode {
            egui::ThemePreference::Dark
        } else {
            egui::ThemePreference::Light
        };
        install_themes_with_preference(ctx, Some(self.custom_accent), mode, preference);
    }
}

impl eframe::App for ShowcaseApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        if self.follow_system {
            if let Some(os_mode) = detect_system_mode(ui.ctx(), false) {
                if os_mode.is_dark() != self.dark_mode {
                    self.dark_mode = os_mode.is_dark();
                    self.sync_theme(ui.ctx());
                    ui.ctx().request_repaint();
                }
            }
        }
        let page = Page::from_id(self.page_id);
        let theme = theme_from_ui(ui);
        apply_borderless_window_resize(ui.ctx(), &theme.palette);

        let entries = [
            DrawerEntry::item(Page::Progress.id(), "ProgressBar", TwoUiIcon::Refresh),
            DrawerEntry::item(Page::SeekBars.id(), "SeekBar", TwoUiIcon::Filter),
            DrawerEntry::item(Page::Pickers.id(), "Pickers", TwoUiIcon::Calendar),
            DrawerEntry::item(Page::QrCode.id(), "QRCode", TwoUiIcon::QrCode),
            DrawerEntry::item(Page::Navigation.id(), "Navigation", TwoUiIcon::Apps),
            DrawerEntry::item(Page::RecyclerViews.id(), "RecyclerViews", TwoUiIcon::Star),
            DrawerEntry::item(Page::Widgets.id(), "Misc. Widgets", TwoUiIcon::Palette),
            DrawerEntry::DottedDivider,
            DrawerEntry::with_badge(
                Page::Preferences.id(),
                "Preferences",
                TwoUiIcon::Settings,
                "New",
            ),
            DrawerEntry::item(Page::AppInfo.id(), "About App", TwoUiIcon::Info),
        ];

        let (_nav_changed, settings_clicked) =
            TwoUiDrawer::new("showcase_drawer", "TwoUI", &entries, &mut self.page_id).show(
                ui,
                &mut self.drawer_open,
                Some((TwoUiIcon::Settings, "Open Preferences")),
            );
        if settings_clicked {
            self.page_id = Page::Preferences.id();
        }

        egui::CentralPanel::default()
            .frame(Frame::NONE.fill(theme.palette.round_and_bg))
            .show(ui, |ui| {
                let mut open_about = false;
                let mut toggle_dark = false;
                let is_dark = self.dark_mode;

                let scaffold_resp = TwoUiScaffold::new("main_scaffold", page.title())
                    .subtitle(page.subtitle())
                    .nav_button(ScaffoldNavButton::DrawerMenu)
                    .window_controls(true)
                    .show(
                        ui,
                        |ui| {
                            if ui
                                .add(
                                    TwoUiIconButton::new(TwoUiIcon::Info)
                                        .size(40.0)
                                        .icon_size(20.0)
                                        .tooltip("About App"),
                                )
                                .clicked()
                            {
                                open_about = true;
                            }

                            if ui
                                .add(
                                    TwoUiIconButton::new(if is_dark {
                                        TwoUiIcon::Sun
                                    } else {
                                        TwoUiIcon::Moon
                                    })
                                    .size(40.0)
                                    .icon_size(20.0)
                                    .tooltip("Toggle Light / Dark mode"),
                                )
                                .clicked()
                            {
                                toggle_dark = true;
                            }
                        },
                        |ui| match page {
                            Page::Progress => self.render_progress_page(ui),
                            Page::SeekBars => self.render_seekbars_page(ui),
                            Page::Pickers => self.render_pickers_page(ui),
                            Page::QrCode => self.render_qr_page(ui),
                            Page::Navigation => self.render_navigation_page(ui),
                            Page::RecyclerViews => self.render_recyclerview_page(ui),
                            Page::Widgets => self.render_widgets_page(ui),
                            Page::Preferences => self.render_preferences_page(ui),
                            Page::AppInfo => self.render_app_info_page(ui),
                        },
                    );

                if open_about {
                    self.page_id = Page::AppInfo.id();
                }
                if toggle_dark {
                    self.follow_system = false;
                    self.dark_mode = !self.dark_mode;
                    self.sync_theme(ui.ctx());
                }
                if scaffold_resp.nav_clicked {
                    self.drawer_open = !self.drawer_open;
                }

                if page == Page::Widgets {
                    show_bottom_floating_bar(ui, 16.0, Id::unique("widgets_div_bar"), |ui| {
                        let items = [
                            DividerButtonItem::new("Share").icon(TwoUiIcon::Share),
                            DividerButtonItem::new("Favorite").icon(TwoUiIcon::Favorite),
                            DividerButtonItem::new("Delete").icon(TwoUiIcon::Delete),
                        ];
                        if let Some(idx) = TwoUiDividerButtonLayout::new("widgets_floating_div", &items)
                            .show(ui)
                        {
                            self.toast.show_message(
                                ui.ctx(),
                                format!("DividerButtonLayout -> {}", items[idx].label),
                                1.8,
                            );
                        }
                    });
                } else if page == Page::Navigation {
                    show_bottom_floating_bar(ui, 16.0, Id::unique("navi_mtab"), |ui| {
                        let btab_items = [
                            BottomNavItem::new("Nav item 1").icon(TwoUiIcon::Bell),
                            BottomNavItem::new("Nav item 2").icon(TwoUiIcon::Refresh),
                            BottomNavItem::new("Nav item 3").icon(TwoUiIcon::Star),
                            BottomNavItem::new("Nav item 4").icon(TwoUiIcon::Location),
                            BottomNavItem::new("Nav item 5").icon(TwoUiIcon::Info),
                            BottomNavItem::new("Nav item 6").icon(TwoUiIcon::Sun),
                            BottomNavItem::new("Nav item 7").icon(TwoUiIcon::Settings),
                            BottomNavItem::new("Nav item 8").icon(TwoUiIcon::QrCode),
                            BottomNavItem::new("Nav item 9").icon(TwoUiIcon::Download),
                            BottomNavItem::new("Nav item 10").icon(TwoUiIcon::Location),
                            BottomNavItem::new("Nav item 11").icon(TwoUiIcon::Info),
                            BottomNavItem::new("Nav item 12").icon(TwoUiIcon::Sun),
                            BottomNavItem::new("Nav item 13").icon(TwoUiIcon::Settings),
                        ];
                        if MainTabBar::new("navi_mtab", &btab_items, &mut self.bottom_tab_idx)
                            .show(ui)
                            .changed()
                        {
                            self.toast.show_message(
                                ui.ctx(),
                                format!("{} selected!", btab_items[self.bottom_tab_idx].label),
                                1.6,
                            );
                        }
                    });
                }
            });

        if page == Page::RecyclerViews && self.rv_tab_idx == 0 {
            let sel_count = self.icon_selected.iter().filter(|&&b| b).count();
            let (toggle_all, clear_sel, done) =
                TwoUiActionModeBar::new(sel_count, self.icon_selected.len())
                    .show(ui.ctx(), ui.ctx().content_rect());
            if toggle_all {
                let target = sel_count < self.icon_selected.len();
                self.icon_selected.fill(target);
            } else if clear_sel || done {
                self.icon_selected.fill(false);
                if clear_sel {
                    self.toast.show_message(ui.ctx(), "Selection cleared", 1.8);
                }
            }
        }

        self.render_dialogs(ui, &theme);
    }
}
