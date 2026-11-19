//! SubTabBar, MainTabBar, BottomNavBar, and DividerButtonLayout showcase page.

use egui::{Margin, Ui};
use egui_twoui::prelude::*;

use crate::app::ShowcaseApp;

impl ShowcaseApp {
    pub(crate) fn render_navigation_page(&mut self, ui: &mut Ui) {
        SectionHeader::new("SubTabBar").show(ui);
        CardSection::new()
            .inner_margin(Margin::symmetric(12, 10))
            .show(ui, |ui| {
                SubTabBar::new(
                    "nav_subtabs",
                    &[
                        "Subtab 1", "Subtab 2", "Subtab 3", "Subtab 4", "Subtab 5", "Subtab 6",
                        "Subtab 7", "Subtab 8",
                    ],
                    &mut self.sub_tab_idx,
                )
                .show(ui);
            });

        SectionHeader::new("MainTabBar").show(ui);
        let main_items = [
            BottomNavItem::new("Alerts").icon(TwoUiIcon::Bell).badge("2"),
            BottomNavItem::new("Sync").icon(TwoUiIcon::Refresh),
            BottomNavItem::new("Featured").icon(TwoUiIcon::Star),
        ];
        if MainTabBar::new("nav_maintabs", &main_items, &mut self.main_tab_idx)
            .show(ui)
            .changed()
        {
            self.toast.show_message(
                ui.ctx(),
                format!("MainTab: {}", main_items[self.main_tab_idx].label),
                1.6,
            );
        }

        SectionHeader::new("BottomNavBar (Icon + Label + Overflow)").show(ui);
        let nav_items = [
            BottomNavItem::new("Favorites").icon(TwoUiIcon::Favorite),
            BottomNavItem::new("Share").icon(TwoUiIcon::Share),
            BottomNavItem::new("Archive").icon(TwoUiIcon::Delete),
        ];
        let nav_overflow = [
            BottomNavOverflowItem::new("Saved drafts").badge("N"),
            BottomNavOverflowItem::new("Preferences").divider_before(true),
        ];
        let (fl_resp, fl_ov) =
            BottomNavBar::new("bnav_fl", &nav_items, &mut self.bottom_nav_icon_idx)
                .overflow(&nav_overflow)
                .show_with_overflow(ui);
        if let Some(ov_idx) = fl_ov {
            self.toast.show_message(
                ui.ctx(),
                format!("Overflow: {}", nav_overflow[ov_idx].label),
                1.6,
            );
        } else if fl_resp.changed() {
            self.toast.show_message(
                ui.ctx(),
                format!("{} selected", nav_items[self.bottom_nav_icon_idx].label),
                1.6,
            );
        }

        SectionHeader::new("BottomNavBar (Icon Only)").show(ui);
        let icon_only_items = [
            BottomNavItem::new("Home").icon(TwoUiIcon::Apps),
            BottomNavItem::new("Calendar").icon(TwoUiIcon::Calendar),
            BottomNavItem::new("Settings").icon(TwoUiIcon::Settings),
        ];
        BottomNavBar::new(
            "bnav_icon_only",
            &icon_only_items,
            &mut self.bottom_nav_icon_only_idx,
        )
        .icon_only(true)
        .show(ui);

        SectionHeader::new("DividerButtonLayout").show(ui);
        let div_items = [
            DividerButtonItem::new("Cancel"),
            DividerButtonItem::new("Apply"),
            DividerButtonItem::new("Share").icon(TwoUiIcon::Share),
        ];
        ui.vertical_centered(|ui| {
            if let Some(idx) = TwoUiDividerButtonLayout::new("nav_div_btn", &div_items)
                .selected(Some(self.div_btn_selected))
                .show(ui)
            {
                self.div_btn_selected = idx;
                self.toast.show_message(
                    ui.ctx(),
                    format!("DividerButton -> {}", div_items[idx].label),
                    1.6,
                );
            }
        });
        ui.add_space(92.0);
    }
}
