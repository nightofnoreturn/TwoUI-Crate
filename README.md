# TwoUI-Crate

[![CI](https://github.com/nightofnoreturn/TwoUI-Crate/actions/workflows/build.yml/badge.svg)](https://github.com/nightofnoreturn/TwoUI-Crate/actions/workflows/build.yml)
[![API Docs](https://img.shields.io/badge/docs-rustdoc-blue)](https://nightofnoreturn.github.io/TwoUI-Crate/)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-informational)](Cargo.toml)

Modular UI component library for [egui](https://github.com/emilk/egui).

## Workspace Crates

| Crate | Description |
| :--- | :--- |
| `egui-twoui` | Umbrella crate re-exporting tokens, icons, widgets, and layouts behind feature flags |
| `egui-twoui-tokens` | Color palettes, light/dark themes, variable typography, and motion curves |
| `egui-twoui-icons` | Vector icon catalog rendered via `resvg` |
| `egui-twoui-widgets` | Buttons, cards, inputs, seekbars, progress indicators, pickers, and tabs |
| `egui-twoui-layouts` | Collapsing AppBar scaffold, adaptive drawer, dialogs, sheets, and toasts |
| `examples/showcase` | Interactive desktop showcase (`twoui-showcase`) |

## Quick Start

Add `egui-twoui` in `Cargo.toml`:

```toml
[dependencies]
egui = "0.37.0"
egui-twoui = { git = "https://github.com/nightofnoreturn/TwoUI-Crate", package = "egui-twoui" }
```

```rust
use egui_twoui::prelude::*;

// Install themes and typography once at startup:
install_themes(ctx, None, TwoUiMode::Dark);
install_typography(ctx, TwoUiTypography::tuned());

// Render inside your app update loop:
let mut enabled = true;
TwoUiScaffold::new("main", "TwoUI").show(
    ui,
    |_ui| {},
    |ui| {
        CardSection::new().show(ui, |ui| {
            SwitchItem::new(&mut enabled, "Enable option").show(ui);
        });
    },
);
```

## Run Showcase

```bash
cargo run -p twoui-showcase
```

## Documentation

- [Getting Started](docs/getting-started.md)
- [Components](docs/components.md)
- [Credits & Licenses](docs/credits.md)
- [API Reference](https://nightofnoreturn.github.io/TwoUI-Crate/)

## License

Dual-licensed under MIT or Apache-2.0.
