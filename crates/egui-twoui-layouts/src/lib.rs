//! High-level application containers, drawers, AppInfo views, and modal overlays.

pub mod app_info;
pub mod drawer;
pub mod overlays;
pub mod scaffold;

pub use app_info::{AppInfoView, AppUpdateState};
pub use drawer::{DrawerEntry, TwoUiDrawer};
pub use overlays::{
    show_color_picker_dialog, DialogAction, TwoUiBottomSheet, TwoUiDialog,
    TwoUiSuggestiveDialog, TwoUiTipPopup, TwoUiToastState,
};
pub use scaffold::{
    apply_borderless_window_resize, show_bottom_floating_bar, TwoUiScaffold, ScaffoldNavButton,
    ScaffoldResponse,
};

/// Shared `egui::Context` temp key syncing whether the primary [`TwoUiScaffold`]
/// toolbar nav toggle is currently projecting a floating pill (`0.0..=1.0`).
#[doc(hidden)]
pub fn toolbar_projection_state_id() -> egui::Id {
    egui::Id::unique("twoui_shared_toolbar_projection_t")
}

/// Shared `egui::Context` temp key storing left horizontal clearance (`f32`) needed to avoid
/// overlapping [`TwoUiDrawer`]'s compact floating vertical navigation pill on narrow viewports.
#[doc(hidden)]
pub fn compact_rail_inset_id() -> egui::Id {
    egui::Id::unique("twoui_shared_compact_rail_inset_left")
}
