//! Operating-system Light / Dark mode detection across Windows, Linux, and macOS.

use egui::{Context, Id};

use crate::palette::TwoUiMode;

#[derive(Clone, Copy)]
struct CachedOsMode {
    time: f64,
    mode: Option<TwoUiMode>,
}

/// Detects the operating system's Light / Dark mode across Windows, Linux, and macOS.
///
/// Combines OS-level lookups (Windows registry `AppsUseLightTheme`, Linux `gsettings` / `GTK_THEME`
/// / KDE / GTK config files, throttled to ~1.5s unless `force_refresh` is `true`) with
/// `ctx.system_theme()`.
pub fn detect_system_mode(ctx: &Context, force_refresh: bool) -> Option<TwoUiMode> {
    let os_theme_from_ctx = if ctx.options(|o| o.theme_preference) == egui::ThemePreference::System
    {
        ctx.system_theme().map(|t| match t {
            egui::Theme::Dark => TwoUiMode::Dark,
            egui::Theme::Light => TwoUiMode::Light,
        })
    } else {
        None
    };

    let cache_id = Id::unique("egui_twoui_os_theme_cache");
    let now = ctx.input(|i| i.time);
    let cached = ctx.data(|d| d.get_temp::<CachedOsMode>(cache_id));

    let probed = match cached {
        Some(c) if !force_refresh && (now - c.time).abs() < 1.5 => c.mode,
        _ => {
            let fresh = probe_os_theme_uncached().or(os_theme_from_ctx);
            ctx.data_mut(|d| {
                d.insert_temp(
                    cache_id,
                    CachedOsMode {
                        time: now,
                        mode: fresh,
                    },
                )
            });
            fresh
        }
    };

    probed.or(os_theme_from_ctx)
}

fn probe_os_theme_uncached() -> Option<TwoUiMode> {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        if let Ok(out) = std::process::Command::new("reg")
            .args([
                "query",
                r"HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
                "/v",
                "AppsUseLightTheme",
            ])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
        {
            if out.status.success() {
                let text = String::from_utf8_lossy(&out.stdout);
                for line in text.lines() {
                    if line.contains("AppsUseLightTheme") {
                        if let Some(val) = line.split_whitespace().last() {
                            if val == "0x0" || val == "0" {
                                return Some(TwoUiMode::Dark);
                            }
                            if val == "0x1" || val == "1" {
                                return Some(TwoUiMode::Light);
                            }
                        }
                    }
                }
            }
        }
        None
    }

    #[cfg(target_os = "linux")]
    {
        if let Ok(out) = std::process::Command::new("gsettings")
            .args(["get", "org.gnome.desktop.interface", "color-scheme"])
            .output()
        {
            if out.status.success() {
                let s = String::from_utf8_lossy(&out.stdout).to_ascii_lowercase();
                if s.contains("prefer-dark") {
                    return Some(TwoUiMode::Dark);
                }
                if s.contains("prefer-light") {
                    return Some(TwoUiMode::Light);
                }
            }
        }

        if let Ok(out) = std::process::Command::new("gsettings")
            .args(["get", "org.gnome.desktop.interface", "gtk-theme"])
            .output()
        {
            if out.status.success() {
                let s = String::from_utf8_lossy(&out.stdout).trim().to_ascii_lowercase();
                if !s.is_empty() && s != "''" {
                    return Some(if s.contains("dark") {
                        TwoUiMode::Dark
                    } else {
                        TwoUiMode::Light
                    });
                }
            }
        }

        if let Ok(gtk_env) = std::env::var("GTK_THEME") {
            let low = gtk_env.to_ascii_lowercase();
            if low.contains(":dark") || low.contains("-dark") {
                return Some(TwoUiMode::Dark);
            }
        }

        if let Some(config_dir) = std::env::var_os("XDG_CONFIG_HOME")
            .map(std::path::PathBuf::from)
            .or_else(|| {
                std::env::var_os("HOME").map(|h| std::path::PathBuf::from(h).join(".config"))
            })
        {
            for gtk_ver in ["gtk-4.0", "gtk-3.0"] {
                let path = config_dir.join(gtk_ver).join("settings.ini");
                if let Ok(contents) = std::fs::read_to_string(path) {
                    let mut prefer_dark_hint = None;
                    for line in contents.lines() {
                        let low = line.trim().to_ascii_lowercase();
                        if low.starts_with("gtk-application-prefer-dark-theme") {
                            if low.ends_with('1') || low.ends_with("true") {
                                return Some(TwoUiMode::Dark);
                            }
                            if low.ends_with('0') || low.ends_with("false") {
                                prefer_dark_hint = Some(TwoUiMode::Light);
                            }
                        } else if low.starts_with("gtk-theme-name") && low.contains("-dark") {
                            return Some(TwoUiMode::Dark);
                        }
                    }
                    if let Some(mode) = prefer_dark_hint {
                        return Some(mode);
                    }
                }
            }
            let kde_path = config_dir.join("kdeglobals");
            if let Ok(contents) = std::fs::read_to_string(kde_path) {
                for line in contents.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with('#') || trimmed.starts_with(';') {
                        continue;
                    }
                    let low = trimmed.to_ascii_lowercase();
                    if low.starts_with("colorscheme=") || low.starts_with("name=") {
                        return Some(if low.contains("dark") {
                            TwoUiMode::Dark
                        } else {
                            TwoUiMode::Light
                        });
                    }
                }
            }
        }
        None
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        None
    }
}
