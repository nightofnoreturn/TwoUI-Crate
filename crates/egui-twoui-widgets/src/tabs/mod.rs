//! Navigation and tab components (`SubTabBar`, `MainTabBar`, `BottomNavBar`).

mod bottom_nav;
mod main_tab;
mod sub_tab;

pub use bottom_nav::{BottomNavBar, BottomNavItem, BottomNavOverflowItem};
pub use main_tab::MainTabBar;
pub use sub_tab::SubTabBar;

use egui::{pos2, Color32, Rect, Ui, Vec2};
use egui_twoui_icons::TwoUiIcon;
use egui_twoui_tokens::{font_id, TwoUiFontWeight, TwoUiPalette};

use crate::controls::paint_badge;

pub(crate) fn floating_capsule_colors(p: &TwoUiPalette, dark: bool) -> (Color32, Color32, Color32) {
    let sel_pill_fill = if dark {
        Color32::from_rgba_unmultiplied(0xFF, 0xFF, 0xFF, 0x1F)
    } else {
        Color32::from_rgba_unmultiplied(0x00, 0x00, 0x00, 0x10)
    };
    let active_fg = if dark {
        p.primary_high.lerp_to_gamma(Color32::WHITE, 0.36)
    } else {
        p.primary_high
    };
    (sel_pill_fill, active_fg, p.text_secondary)
}

pub(crate) fn paint_nav_slot_contents(
    ui: &Ui,
    paint_r: Rect,
    icon: Option<TwoUiIcon>,
    label: Option<&str>,
    badge: Option<&str>,
    fg: Color32,
    selected: bool,
) {
    let weight = if selected {
        TwoUiFontWeight::SemiBold
    } else {
        TwoUiFontWeight::Medium
    };
    match (icon, label) {
        (Some(ic), Some(lbl)) => {
            let icon_center = pos2(paint_r.center().x, paint_r.center().y - 9.0);
            ic.paint(
                ui,
                Rect::from_center_size(icon_center, Vec2::splat(22.0)),
                fg,
            );
            let g = ui
                .painter()
                .layout_no_wrap(lbl.to_owned(), font_id(ui, 12.0, weight), fg);
            ui.painter().galley(
                pos2(
                    paint_r.center().x - g.size().x * 0.5,
                    paint_r.center().y + 12.0 - g.size().y * 0.5,
                ),
                g,
                fg,
            );
            if let Some(b) = badge {
                paint_badge(ui, pos2(icon_center.x + 13.0, icon_center.y - 8.0), b);
            }
        }
        (Some(ic), None) => {
            ic.paint(
                ui,
                Rect::from_center_size(paint_r.center(), Vec2::splat(21.0)),
                fg,
            );
            if let Some(b) = badge {
                paint_badge(
                    ui,
                    pos2(paint_r.center().x + 12.0, paint_r.center().y - 8.0),
                    b,
                );
            }
        }
        (None, Some(lbl)) => {
            let g = ui
                .painter()
                .layout_no_wrap(lbl.to_owned(), font_id(ui, 14.5, weight), fg);
            ui.painter().galley(
                pos2(
                    paint_r.center().x - g.size().x * 0.5,
                    paint_r.center().y - g.size().y * 0.5,
                ),
                g,
                fg,
            );
            if let Some(b) = badge {
                paint_badge(ui, pos2(paint_r.right() - 12.0, paint_r.top() + 10.0), b);
            }
        }
        (None, None) => {}
    }
}
