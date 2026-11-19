//! Collapsible floating inset navigation drawer (`22dp` corner radius, `10dp` inset, full panel <-> compact icon pill).

mod compact_pill;
mod full_panel;

use egui::{pos2, Color32, Id, Rect, Shape, Ui};
use egui_twoui_icons::TwoUiIcon;
use egui_twoui_tokens::{animate_toggle, dimens, ease_release, theme_from_ui};

/// An item or visual divider inside [`TwoUiDrawer`].
#[derive(Debug, Clone)]
pub enum DrawerEntry<'a> {
    Item {
        id: usize,
        label: &'a str,
        icon: TwoUiIcon,
        badge_count: Option<&'a str>,
    },
    DottedDivider,
    SectionLabel(&'a str),
}

impl<'a> DrawerEntry<'a> {
    pub fn item(id: usize, label: &'a str, icon: TwoUiIcon) -> Self {
        Self::Item {
            id,
            label,
            icon,
            badge_count: None,
        }
    }

    pub fn with_badge(id: usize, label: &'a str, icon: TwoUiIcon, badge: &'a str) -> Self {
        Self::Item {
            id,
            label,
            icon,
            badge_count: Some(badge),
        }
    }
}

/// Collapsible floating inset drawer (`22dp` rounded corners, `10dp` inset)
/// that shrinks into a compact floating vertical icon pill beneath the toolbar menu toggle
/// so content widens cleanly across the viewport.
pub struct TwoUiDrawer<'a> {
    id_salt: &'a str,
    header_title: &'a str,
    entries: &'a [DrawerEntry<'a>],
    selected_id: &'a mut usize,
    width: f32,
    rail_on_wide: bool,
}

impl<'a> TwoUiDrawer<'a> {
    pub fn new(
        id_salt: &'a str,
        header_title: &'a str,
        entries: &'a [DrawerEntry<'a>],
        selected_id: &'a mut usize,
    ) -> Self {
        Self {
            id_salt,
            header_title,
            entries,
            selected_id,
            width: dimens::DRAWER_WIDTH_OPEN,
            rail_on_wide: true,
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width.clamp(220.0, 340.0);
        self
    }

    /// Whether collapsing shows a compact floating vertical icon pill beneath the toolbar's
    /// navigation toggle (`true` by default), or hides navigation completely (`false`).
    pub fn rail_on_wide(mut self, enabled: bool) -> Self {
        self.rail_on_wide = enabled;
        self
    }

    /// Shows the collapsible floating drawer card on the left side of `ui` (or, when collapsed,
    /// renders the compact floating vertical navigation pill directly beneath the navigation toggle).
    /// Returns `(selection_changed, header_action_clicked)`.
    pub fn show(
        self,
        ui: &mut Ui,
        is_open: &mut bool,
        header_action: Option<(TwoUiIcon, &'static str)>,
    ) -> (bool, bool) {
        let theme = theme_from_ui(ui);
        let mut selection_changed = false;
        let mut action_clicked = false;

        let inset = dimens::DRAWER_START_INSET;
        let open_panel_w = self.width + inset + 4.0;
        let anim_t = animate_toggle(
            ui,
            Id::unique((self.id_salt, "twoui_drawer_t")),
            *is_open,
            0.30,
            ease_release,
        );
        let panel_w = open_panel_w * anim_t;

        // Crossfade timing: full drawer content fades cleanly before width hits 0 so content widens
        // smoothly while the compact vertical pill unfolds right beneath `Toggle Navigation`.
        let interpolated_alpha = ((anim_t - 0.22) / 0.78).clamp(0.0, 1.0);
        let card_vis = (anim_t / 0.22).clamp(0.0, 1.0);
        let compact_raw = ((0.58 - anim_t) / 0.58).clamp(0.0, 1.0);
        let compact_t = compact_raw * compact_raw * (3.0 - 2.0 * compact_raw);

        let rail_inset_id = crate::compact_rail_inset_id();
        let target_inset = if self.rail_on_wide { 70.0 * compact_t } else { 0.0 };
        let prev_inset: f32 = ui.ctx().data(|d| d.get_temp(rail_inset_id)).unwrap_or(0.0);
        if (prev_inset - target_inset).abs() > 0.05 {
            ui.ctx().request_repaint();
        }
        ui.ctx().data_mut(|d| d.insert_temp(rail_inset_id, target_inset));

        if panel_w >= 4.0 {
            let (sel, act) = full_panel::paint_full_drawer_panel(
                ui,
                &theme,
                self.id_salt,
                self.header_title,
                self.entries,
                self.selected_id,
                panel_w,
                inset,
                card_vis,
                interpolated_alpha,
                header_action,
            );
            selection_changed |= sel;
            action_clicked |= act;
        }

        if self.rail_on_wide && compact_t > 0.01 {
            let after_avail = ui.available_rect_before_wrap();
            let pill_origin = pos2(
                after_avail.left() + 14.0,
                after_avail.top() + dimens::TOOLBAR_TOP_INSET + dimens::TOOLBAR_ROW_HEIGHT + 8.0,
            );
            let (pill_sel, pill_act) = compact_pill::paint_compact_pill(
                ui,
                &theme,
                self.id_salt,
                self.entries,
                self.selected_id,
                pill_origin,
                after_avail.bottom(),
                compact_t,
                header_action,
            );
            selection_changed |= pill_sel;
            action_clicked |= pill_act;
        }

        (selection_changed, action_clicked)
    }
}

pub(crate) fn paint_drawer_dots(
    ui: &Ui,
    div_rect: Rect,
    is_dark: bool,
    horiz_inset: f32,
    alpha: f32,
) {
    let y = div_rect.center().y;
    let dot_col = (if is_dark {
        Color32::from_rgb(0x58, 0x58, 0x58)
    } else {
        Color32::from_rgb(0xC6, 0xC6, 0xC6)
    })
    .gamma_multiply(alpha);
    let pts = [
        pos2(div_rect.left() + horiz_inset, y),
        pos2(div_rect.right() - horiz_inset, y),
    ];
    ui.painter()
        .extend(Shape::dotted_line(&pts, dot_col, 6.0, 1.0));
}
