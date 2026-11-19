//! Global modal dialogs, bottom sheets, tip popup, snackbar, and toast rendering.

use egui::Ui;
use egui_twoui::prelude::*;

use crate::app::ShowcaseApp;

impl ShowcaseApp {
    pub(crate) fn render_dialogs(&mut self, ui: &Ui, theme: &TwoUiTheme) {
        let ctx = ui.ctx();

        if let Some(chosen) = show_color_picker_dialog(
            ctx,
            "accent_color_dlg",
            &mut self.color_dialog_open,
            &mut self.custom_accent,
            &mut self.color_dialog_draft,
        ) {
            self.custom_accent = chosen;
            self.sync_theme(ctx);
        }

        TwoUiDialog::new("date_picker_dlg", "Select date")
            .action(DialogAction::cancel("Cancel"))
            .action(DialogAction::primary("Done"))
            .show(ctx, &mut self.date_dialog_open, |ui| {
                TwoUiDatePicker::new("dlg_cal", &mut self.date_year, &mut self.date_month, &mut self.date_day)
                    .mode(DatePickerMode::Calendar)
                    .show(ui);
            });

        TwoUiDialog::new("time_picker_dlg", "Select time")
            .action(DialogAction::cancel("Cancel"))
            .action(DialogAction::primary("Done"))
            .show(ctx, &mut self.time_dialog_open, |ui| {
                TwoUiTimePicker::new("dlg_tp", &mut self.time_hour, &mut self.time_min)
                    .is_24_hour(self.time_24h)
                    .show(ui);
            });

        TwoUiDialog::new("sleep_picker_dlg", "Start & End Time")
            .action(DialogAction::cancel("Cancel"))
            .action(DialogAction::primary("Done"))
            .show(ctx, &mut self.sleep_dialog_open, |ui| {
                TwoUiSleepTimePicker::new("dlg_sleep", &mut self.bedtime_min, &mut self.wakeup_min)
                    .is_24_hour(self.time_24h)
                    .show(ui);
            });

        if let Some(action_idx) = TwoUiDialog::new("demo_dialog", "Reset Settings?")
            .body("Modal dialog with rounded corners, pill buttons, and optional checkbox.")
            .single_choice_option(&mut self.dialog_dont_show_again, "Don't ask again")
            .action(DialogAction::cancel("Cancel"))
            .action(DialogAction::destructive("Reset"))
            .action(DialogAction::primary("Confirm"))
            .show(ctx, &mut self.dialog_open, |_ui| {})
        {
            let label = ["Cancel", "Reset", "Confirm"][action_idx];
            self.toast.show_message(ctx, format!("Dialog: {label}"), 1.8);
        }

        if let Some(action_idx) = TwoUiSuggestiveDialog::new(
            "demo_suggestive_dlg",
            "Floating Navigation",
            "Enable floating capsules and translucent toolbar pills across pages.",
        )
        .icon(TwoUiIcon::Star)
        .action(DialogAction::cancel("Not now"))
        .action(DialogAction::primary("Turn on"))
        .show(ctx, &mut self.suggestive_dialog_open)
        {
            let label = ["Not now", "Turn on"][action_idx];
            self.toast
                .show_message(ctx, format!("SuggestiveDialog: {label}"), 1.8);
        }

        TwoUiBottomSheet::new("demo_bottom_sheet", "Bottom Sheet").show(
            ctx,
            &mut self.bottom_sheet_open,
            |ui| {
                ui.label("Anchored bottom sheet with top rounded corners and drag handle.");
                ui.add_space(12.0);
                CardSection::new()
                    .fill(theme.palette.surface_container)
                    .show(ui, |ui| {
                        ui.add(CardItem::new("Share QR").icon(TwoUiIcon::Share));
                        ui.add(CardItem::new("Download").icon(TwoUiIcon::Download).top_divider(true));
                    });
            },
        );

        TwoUiBottomSheet::new("licenses_sheet", "Open Source Licenses").show(
            ctx,
            &mut self.licenses_sheet_open,
            |ui| {
                CardSection::new()
                    .fill(theme.palette.surface_container)
                    .show(ui, |ui| {
                        ui.add(CardItem::new("egui & eframe 0.37.0").summary("MIT / Apache-2.0"));
                        ui.add(CardItem::new("resvg 0.50.0").summary("MPL-2.0").top_divider(true));
                        ui.add(CardItem::new("qrcode 0.14.1").summary("MIT / Apache-2.0").top_divider(true));
                        ui.add(CardItem::new("Roboto Flex").summary("SIL OFL 1.1").top_divider(true));
                        ui.add(CardItem::new("Vector Icons").summary("MIT").top_divider(true));
                    });
            },
        );

        if TwoUiTipPopup::new("demo_tip_popup", "TipPopup floats with an action button!")
            .action("OK")
            .show(ctx, self.tip_anchor, &mut self.tip_popup_open)
        {
            self.toast.show_message(ctx, "TipPopup acknowledged!", 1.8);
        }

        if self.snackbar.render(ctx, ctx.content_rect()) {
            self.toast.show_message(ctx, "Action undone!", 1.8);
        }

        self.toast.ui(ui);
    }
}
