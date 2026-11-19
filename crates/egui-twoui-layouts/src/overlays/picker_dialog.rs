//! Modal color picker dialog helper combining [`super::TwoUiDialog`] and
//! [`egui_twoui_widgets::pickers::TwoUiColorPicker`].

use egui::{Color32, Context};
use egui_twoui_tokens::colors;
use egui_twoui_widgets::{
    button::{TwoUiButton, TwoUiButtonStyle},
    pickers::TwoUiColorPicker,
};

use super::{DialogAction, TwoUiDialog};

/// Presents [`TwoUiColorPicker`] inside a [`TwoUiDialog`]; returns `Some(Color32)` on Apply.
pub fn show_color_picker_dialog(
    ctx: &Context,
    id_salt: &str,
    open: &mut bool,
    color: &mut Color32,
    draft: &mut Color32,
) -> Option<Color32> {
    if !*open {
        return None;
    }
    let clicked = TwoUiDialog::new(id_salt, "Select color")
        .action(DialogAction::cancel("Cancel"))
        .action(DialogAction::primary("Done"))
        .show(ctx, open, |ui| {
            if ui
                .add(TwoUiButton::new("Reset to Default Blue").style(TwoUiButtonStyle::Transparent))
                .clicked()
            {
                *draft = colors::PRIMARY_BLUE;
            }
            ui.add_space(6.0);
            TwoUiColorPicker::new(id_salt, draft).show(ui);
        });
    if clicked == Some(1) {
        *color = *draft;
        Some(*draft)
    } else {
        None
    }
}
