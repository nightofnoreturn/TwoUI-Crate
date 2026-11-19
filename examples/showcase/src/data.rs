//! Static sample data, navigation pages, and preset definitions for [`ShowcaseApp`].

use egui::Color32;
use egui_twoui::prelude::TwoUiIcon;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Page {
    Progress,
    SeekBars,
    Pickers,
    QrCode,
    Navigation,
    RecyclerViews,
    Widgets,
    Preferences,
    AppInfo,
}

impl Page {
    pub(crate) fn from_id(id: usize) -> Self {
        match id {
            0 => Self::Progress,
            1 => Self::SeekBars,
            2 => Self::Pickers,
            3 => Self::QrCode,
            4 => Self::Navigation,
            5 => Self::RecyclerViews,
            6 => Self::Widgets,
            7 => Self::Preferences,
            _ => Self::AppInfo,
        }
    }

    pub(crate) fn id(self) -> usize {
        match self {
            Self::Progress => 0,
            Self::SeekBars => 1,
            Self::Pickers => 2,
            Self::QrCode => 3,
            Self::Navigation => 4,
            Self::RecyclerViews => 5,
            Self::Widgets => 6,
            Self::Preferences => 7,
            Self::AppInfo => 8,
        }
    }

    pub(crate) fn title(self) -> &'static str {
        match self {
            Self::Progress => "ProgressBars",
            Self::SeekBars => "SeekBars",
            Self::Pickers => "Pickers",
            Self::QrCode => "QRCodes",
            Self::Navigation => "Navigation",
            Self::RecyclerViews => "RecyclerViews",
            Self::Widgets => "Misc. Widgets",
            Self::Preferences => "Preferences",
            Self::AppInfo => "About TwoUI-Crate",
        }
    }

    pub(crate) fn subtitle(self) -> Option<&'static str> {
        match self {
            Self::Progress => Some("Determinate, indeterminate, and gradient indicators"),
            Self::SeekBars => Some("Standard, expand, stepped, overlap, and vertical sliders"),
            Self::Pickers => Some("Number, time, calendar, week, sleep, and color pickers"),
            Self::QrCode => Some("QR cards and live encoder"),
            Self::Navigation => Some("Tabs, floating capsules, bottom bars, and action dividers"),
            Self::RecyclerViews => Some("Icons, index rail, multi-select, contacts, and app lists"),
            Self::Widgets => Some("Buttons, chips, search bars, list items, and dialogs"),
            Self::Preferences => Some("Theme controls, typography, tips, and preference cards"),
            Self::AppInfo => None,
        }
    }
}

pub(crate) struct AppRowItem {
    pub(crate) name: &'static str,
    pub(crate) package: &'static str,
    pub(crate) icon: TwoUiIcon,
    pub(crate) selected: bool,
}

pub(crate) struct ContactItem {
    pub(crate) name: &'static str,
    pub(crate) subtitle: &'static str,
    pub(crate) color: Color32,
}

pub(crate) const ACCENT_PRESETS: [(&str, Color32); 5] = [
    ("Default Blue", Color32::from_rgb(0x38, 0x7A, 0xFF)),
    ("Emerald", Color32::from_rgb(0x11, 0xA8, 0x5F)),
    ("Violet", Color32::from_rgb(0x7C, 0x5C, 0xFF)),
    ("Orange", Color32::from_rgb(0xE6, 0x5B, 0x17)),
    ("Rose", Color32::from_rgb(0xE1, 0x4B, 0x6E)),
];

pub(crate) const CONTACTS: [ContactItem; 8] = [
    ContactItem { name: "Alex Morgan", subtitle: "UI Designer", color: Color32::from_rgb(0x38, 0x7A, 0xFF) },
    ContactItem { name: "Budi Santoso", subtitle: "Desktop Maintainer", color: Color32::from_rgb(0x23, 0xC4, 0x92) },
    ContactItem { name: "Clara Chen", subtitle: "Graphics Dev", color: Color32::from_rgb(0x7C, 0x5C, 0xFF) },
    ContactItem { name: "David Kim", subtitle: "Core Contributor", color: Color32::from_rgb(0xEC, 0x40, 0x7A) },
    ContactItem { name: "Elena Cruz", subtitle: "Systems Engineer", color: Color32::from_rgb(0xFF, 0x70, 0x43) },
    ContactItem { name: "Farhan Malik", subtitle: "Product Designer", color: Color32::from_rgb(0x00, 0xBF, 0xA5) },
    ContactItem { name: "Grace Patel", subtitle: "QA Lead", color: Color32::from_rgb(0x3B, 0xA3, 0xC3) },
    ContactItem { name: "Hadi Wijaya", subtitle: "Accessibility", color: Color32::from_rgb(0x38, 0x7A, 0xFF) },
];

pub(crate) fn sample_apps() -> Vec<AppRowItem> {
    vec![
        AppRowItem { name: "Settings", package: "app.settings", icon: TwoUiIcon::Settings, selected: true },
        AppRowItem { name: "Calendar", package: "app.calendar", icon: TwoUiIcon::Calendar, selected: false },
        AppRowItem { name: "System Monitor", package: "app.monitor", icon: TwoUiIcon::Refresh, selected: true },
        AppRowItem { name: "Bluetooth", package: "app.bluetooth", icon: TwoUiIcon::Bluetooth, selected: false },
        AppRowItem { name: "Notifications", package: "app.notifications", icon: TwoUiIcon::Bell, selected: true },
        AppRowItem { name: "Quick Share", package: "app.share", icon: TwoUiIcon::Share, selected: false },
        AppRowItem { name: "Security", package: "app.security", icon: TwoUiIcon::Shield, selected: true },
        AppRowItem { name: "Network", package: "app.network", icon: TwoUiIcon::Wifi, selected: false },
    ]
}
