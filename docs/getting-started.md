# Getting Started

## Dependency & Feature Flags

Add `egui-twoui` to `Cargo.toml`:

```toml
[dependencies]
egui = "0.37.0"
egui-twoui = { git = "https://github.com/nightofnoreturn/TwoUI-Crate", package = "egui-twoui" }
```

To opt into specific modules only:

```toml
[dependencies]
egui-twoui = { git = "https://github.com/nightofnoreturn/TwoUI-Crate", package = "egui-twoui", default-features = false, features = ["icons", "widgets", "layouts", "font"] }
```

| Feature | Default | Enables |
| :--- | :---: | :--- |
| `icons` | Yes | `egui-twoui-icons` (`TwoUiIcon`) |
| `widgets` | Yes | `egui-twoui-widgets`, `TwoUiExt`, and `icons` |
| `layouts` | Yes | `egui-twoui-layouts`, `widgets`, and `icons` |
| `qrcode` | Yes | `TwoUiQrCode` widget |
| `font` | Yes | Bundled Roboto Flex variable font (`TwoUiTypography`, `install_typography`) |

## Theme & Typography Setup

Initialize themes and typography once when setting up `eframe` (or inside your first frame):

```rust
use egui::{Color32, ThemePreference};
use egui_twoui::prelude::*;

pub fn setup(ctx: &egui::Context) {
    install_typography(ctx, TwoUiTypography::tuned());
    install_themes_with_preference(ctx, None, TwoUiMode::Dark, ThemePreference::System);
}
```

Switch modes or customize accents dynamically:

```rust
let custom = TwoUiTheme::dark().with_accent(Color32::from_rgb(0x38, 0x7A, 0xFF));
apply_theme(ctx, &custom);
```

## Minimal App Example

```rust
use eframe::egui;
use egui_twoui::prelude::*;

struct MyApp {
    wifi: bool,
    volume: f32,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        TwoUiScaffold::new("home", "Settings")
            .subtitle("System preferences")
            .show(
                ui,
                |_ui| {},
                |ui| {
                    SectionHeader::new("Connections").show(ui);
                    CardSection::new().show(ui, |ui| {
                        SwitchItem::new(&mut self.wifi, "Wi-Fi")
                            .icon(TwoUiIcon::Wifi)
                            .show(ui);
                    });

                    SectionHeader::new("Audio").show(ui);
                    CardSection::new().show(ui, |ui| {
                        ui.add(TwoUiSeekBar::new("Media volume", &mut self.volume, 0.0..=100.0));
                    });
                },
            );
    }
}
```

## Adaptive Navigation & Floating Bars

Combine `TwoUiDrawer`, `TwoUiScaffold`, and `show_bottom_floating_bar` for responsive desktop and compact layouts:

- `TwoUiDrawer` renders a split rail/drawer on wide viewports and collapses cleanly on narrow windows.
- `TwoUiScaffold::window_controls(true)` adds compact frameless window controls (drag bar, minimize, maximize/restore, close), paired with `apply_borderless_window_resize`.
- `show_bottom_floating_bar` anchors `MainTabBar`, `BottomNavBar`, or `TwoUiDividerButtonLayout` over scrolling content.
