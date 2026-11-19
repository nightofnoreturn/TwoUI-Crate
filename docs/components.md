# Components

API catalog across `TwoUI-Crate` workspace crates (available directly via `egui_twoui::prelude::*`).

## Tokens & Theming (`egui-twoui-tokens`)

- **Palette & Modes:** `TwoUiMode`, `TwoUiPalette`, `detect_system_mode`
- **Themes:** `TwoUiTheme`, `apply_theme`, `install_themes`, `install_themes_with_preference`, `theme_from_ctx`, `theme_from_ui`
- **Typography:** `TwoUiTypography`, `TwoUiFontWeight`, `install_typography`, `install_custom_font`, `install_custom_variable_font`, `reset_default_fonts`, `has_twoui_fonts`, `font_id`, `font_id_for`
- **Motion & Surfaces:** `ElevationTier`, `elevation_shadow`, `concentric_radius`, `paint_floating_capsule`, `paint_frosted_surface`, `show_frosted_frame`, `enable_backdrop_blur`, `set_backdrop_blur_enabled`, `is_backdrop_blur_enabled`, `cubic_bezier`, `ease_fast_out_slow_in`, `ease_sine_out_70`, `ease_sine_out_80`, `ease_sine_in_out_80`, `ease_release`, `animate_toggle`, `animate_toggle_ctx`

## Icons (`egui-twoui-icons`)

- `TwoUiIcon` — resolution-independent SVG icon enum rasterized on demand via `resvg`

## Widgets (`egui-twoui-widgets`)

- **Buttons & Capsules:** `TwoUiButton`, `TwoUiButtonStyle`, `TwoUiIconButton`, `TwoUiChip`, `TwoUiSwitchBar`, `TwoUiDividerButtonLayout`, `DividerButtonItem`
- **Cards & Lists:** `SectionHeader`, `CardSection`, `CardItem`, `SwitchItem`, `RadioItem`, `SuggestionCard`, `TipsCard`, `RelativeLinksCard`, `TwoUiPreferenceCaption`, `paint_divider`, `DividerInset`
- **Selection Controls:** `TwoUiSwitch`, `TwoUiCheckBox`, `TwoUiRadioButton`, `paint_badge`
- **Inputs & Search:** `TwoUiSearchBar`, `TwoUiDropdown`
- **SeekBars:** `TwoUiSeekBar`, `TwoUiSeekBarPlus`, `TwoUiSeekBarPro`, `SeekBarMode`
- **Progress:** `TwoUiProgressBar`, `TwoUiSpinner`, `SpinnerSize`
- **Pickers & QR:** `TwoUiDatePicker`, `DatePickerMode`, `TwoUiTimePicker`, `TwoUiSleepTimePicker`, `TwoUiNumberPicker`, `TwoUiColorPicker`, `TwoUiQrCode` (requires `qrcode` feature)
- **Tabs & Extras:** `SubTabBar`, `MainTabBar`, `BottomNavBar`, `BottomNavItem`, `BottomNavOverflowItem`, `TwoUiActionModeBar`, `TwoUiIndexScroll`, `TwoUiSnackBarState`

## Layouts & Overlays (`egui-twoui-layouts`)

- **Scaffold & Navigation:** `TwoUiScaffold`, `ScaffoldNavButton`, `ScaffoldResponse`, `TwoUiDrawer`, `DrawerEntry`, `show_bottom_floating_bar`, `apply_borderless_window_resize`
- **Dialogs & Sheets:** `TwoUiDialog`, `TwoUiSuggestiveDialog`, `TwoUiBottomSheet`, `TwoUiTipPopup`, `TwoUiToastState`, `DialogAction`, `show_color_picker_dialog`
- **App Info:** `AppInfoView`, `AppUpdateState`
