//! Modal dialogs, bottom sheets, tooltips, and toast overlays (`TwoUiDialog`,
//! `TwoUiSuggestiveDialog`, `TwoUiBottomSheet`, `TwoUiTipPopup`, `TwoUiToastState`, and
//! `show_color_picker_dialog`).

mod bottom_sheet;
mod dialog;
mod picker_dialog;
mod suggestive_dialog;
mod tip_toast;

pub use bottom_sheet::TwoUiBottomSheet;
pub use dialog::TwoUiDialog;
pub use picker_dialog::show_color_picker_dialog;
pub use suggestive_dialog::TwoUiSuggestiveDialog;
pub use tip_toast::{TwoUiTipPopup, TwoUiToastState};

/// Action button specification for [`TwoUiDialog`] and [`TwoUiSuggestiveDialog`].
#[derive(Debug, Clone)]
pub struct DialogAction<'a> {
    pub label: &'a str,
    pub destructive: bool,
    pub primary: bool,
}

impl<'a> DialogAction<'a> {
    pub fn cancel(label: &'a str) -> Self {
        Self {
            label,
            destructive: false,
            primary: false,
        }
    }

    pub fn primary(label: &'a str) -> Self {
        Self {
            label,
            destructive: false,
            primary: true,
        }
    }

    pub fn destructive(label: &'a str) -> Self {
        Self {
            label,
            destructive: true,
            primary: false,
        }
    }
}
