//! Variable font configuration and Roboto Flex typography helpers.

use std::sync::Arc;

use egui::epaint::text::{FontTweak, VariationCoords};
use egui::{Context, FontData, FontDefinitions, FontFamily, FontId, Id, Ui};

#[cfg(feature = "font")]
const ROBOTO_FLEX_TTF: &[u8] = include_bytes!("../assets/fonts/RobotoFlex.ttf");

const FAM_MEDIUM: &str = "twoui_medium";
const FAM_SEMIBOLD: &str = "twoui_semibold";
const FAM_DISPLAY: &str = "twoui_display";

const DATA_REGULAR: &str = "twoui_roboto_regular";
const DATA_MEDIUM: &str = "twoui_roboto_medium";
const DATA_SEMIBOLD: &str = "twoui_roboto_semibold";
const DATA_DISPLAY: &str = "twoui_roboto_display";

/// Semantic font weight roles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TwoUiFontWeight {
    /// Body text, descriptions, list summaries, and captions (`wght ~ 400`).
    #[default]
    Regular,
    /// Interactive labels, pills, chips, and tab items (`wght ~ 520`).
    Medium,
    /// Section headers, compact toolbar titles, card headers, and dialog titles (`wght ~ 630`).
    SemiBold,
    /// Extended collapsing AppBar hero headline (`34sp`, `opsz ~ 34`).
    Display,
}

impl TwoUiFontWeight {
    /// Returns the `egui::FontFamily` matching this semantic weight.
    pub fn to_family(self) -> FontFamily {
        match self {
            Self::Regular => FontFamily::Proportional,
            Self::Medium => FontFamily::Name(FAM_MEDIUM.into()),
            Self::SemiBold => FontFamily::Name(FAM_SEMIBOLD.into()),
            Self::Display => FontFamily::Name(FAM_DISPLAY.into()),
        }
    }
}

/// Variable-font axis configuration for **Roboto Flex**.
///
/// [`TwoUiTypography::tuned`] uses wider, open-counter proportions for crisp UI legibility.
#[derive(Debug, Clone, PartialEq)]
pub struct TwoUiTypography {
    /// Width axis (`wdth`, `25.0..=151.0`; stock `100.0`, tuned `104.5`).
    pub width: f32,
    /// Counter width axis (`XTRA`, `323.0..=603.0`; stock `468.0`, tuned `492.0`).
    pub counter_width: f32,
    /// Lowercase x-height axis (`YTLC`, `416.0..=570.0`; stock `514.0`, tuned `528.0`).
    pub lowercase_height: f32,
    /// Uppercase height axis (`YTUC`, `528.0..=760.0`; stock `712.0`).
    pub uppercase_height: f32,
    /// Thin stroke weight (`YOPQ`, `25.0..=135.0`; stock `79.0`, tuned `84.0`).
    pub thin_stroke: f32,
    /// Grade fine-tuning (`GRAD`, `-200.0..=150.0`, `0.0` default).
    pub grade: f32,
    /// Regular weight (`wght`, `100.0..=1000.0`, default `400.0`).
    pub regular_weight: f32,
    /// Medium weight (`wght`, `100.0..=1000.0`, default `520.0`).
    pub medium_weight: f32,
    /// SemiBold weight (`wght`, `100.0..=1000.0`, default `630.0`).
    pub semibold_weight: f32,
    /// Display headline weight (`wght`, `100.0..=1000.0`, default `480.0`).
    pub display_weight: f32,
    /// Visual scale tweak (`1.0` default).
    pub scale: f32,
}

impl Default for TwoUiTypography {
    fn default() -> Self {
        Self::tuned()
    }
}

impl TwoUiTypography {
    /// Default tuned Roboto Flex preset with wider counters and slightly taller x-height.
    pub fn tuned() -> Self {
        Self {
            width: 104.5,
            counter_width: 492.0,
            lowercase_height: 528.0,
            uppercase_height: 718.0,
            thin_stroke: 84.0,
            grade: 0.0,
            regular_weight: 400.0,
            medium_weight: 520.0,
            semibold_weight: 630.0,
            display_weight: 480.0,
            scale: 1.0,
        }
    }

    /// Default stock Roboto Flex proportions (`wdth = 100.0`).
    pub fn stock_roboto() -> Self {
        Self {
            width: 100.0,
            counter_width: 468.0,
            lowercase_height: 514.0,
            uppercase_height: 712.0,
            thin_stroke: 79.0,
            grade: 0.0,
            regular_weight: 400.0,
            medium_weight: 500.0,
            semibold_weight: 600.0,
            display_weight: 400.0,
            scale: 1.0,
        }
    }

    /// Slightly wider, softer rounded-feel preset for large desktop displays.
    pub fn rounded_wide() -> Self {
        Self {
            width: 108.0,
            counter_width: 515.0,
            lowercase_height: 534.0,
            uppercase_height: 722.0,
            thin_stroke: 86.0,
            grade: 0.0,
            regular_weight: 410.0,
            medium_weight: 530.0,
            semibold_weight: 640.0,
            display_weight: 490.0,
            scale: 1.0,
        }
    }

    fn coords_for(&self, weight: TwoUiFontWeight) -> VariationCoords {
        let (wght, opsz, xopq) = match weight {
            TwoUiFontWeight::Regular => (self.regular_weight, 14.0, 96.0),
            TwoUiFontWeight::Medium => (self.medium_weight, 15.0, 100.0),
            TwoUiFontWeight::SemiBold => (self.semibold_weight, 20.0, 105.0),
            TwoUiFontWeight::Display => (self.display_weight, 34.0, 98.0),
        };
        VariationCoords::new([
            (b"wght", wght.clamp(100.0, 1000.0)),
            (b"wdth", self.width.clamp(25.0, 151.0)),
            (b"XTRA", self.counter_width.clamp(323.0, 603.0)),
            (b"YTLC", self.lowercase_height.clamp(416.0, 570.0)),
            (b"YTUC", self.uppercase_height.clamp(528.0, 760.0)),
            (b"YOPQ", self.thin_stroke.clamp(25.0, 135.0)),
            (b"XOPQ", xopq),
            (b"opsz", opsz),
            (b"GRAD", self.grade.clamp(-200.0, 150.0)),
        ])
    }

    fn tweak_for(&self, weight: TwoUiFontWeight) -> FontTweak {
        FontTweak {
            scale: self.scale.clamp(0.75, 1.4),
            coords: self.coords_for(weight),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum FontState {
    Bundled(TwoUiTypography),
    CustomVariable(TwoUiTypography),
    CustomStatic,
    EguiDefault,
}

fn font_state_id() -> Id {
    Id::unique("egui_twoui_font_state")
}

/// Returns `true` if TwoUI font families (`twoui_medium` / `twoui_semibold` / `twoui_display`)
/// have been registered on `ctx`.
pub fn has_twoui_fonts(ctx: &Context) -> bool {
    ctx.data(|d| d.get_temp::<FontState>(font_state_id()))
        .is_some()
}

pub(crate) fn has_twoui_fonts_or_bundled() -> bool {
    cfg!(feature = "font")
}

/// Builds a [`FontDefinitions`] configured with `base` variable font bytes across all 4 weights,
/// preserving `egui`'s built-in fallback icons and emoji fonts.
pub fn build_variable_font_definitions(
    base: FontData,
    typography: &TwoUiTypography,
) -> FontDefinitions {
    let mut defs = FontDefinitions::default();
    let fallback_prop = defs
        .families
        .get(&FontFamily::Proportional)
        .cloned()
        .unwrap_or_default();

    for (key, weight) in [
        (DATA_REGULAR, TwoUiFontWeight::Regular),
        (DATA_MEDIUM, TwoUiFontWeight::Medium),
        (DATA_SEMIBOLD, TwoUiFontWeight::SemiBold),
        (DATA_DISPLAY, TwoUiFontWeight::Display),
    ] {
        let data = base.clone().tweak(typography.tweak_for(weight));
        defs.font_data.insert(key.to_owned(), Arc::new(data));
    }

    let mut prop = vec![DATA_REGULAR.to_owned()];
    prop.extend(fallback_prop.iter().cloned());
    defs.families.insert(FontFamily::Proportional, prop);

    for (fam_name, data_key) in [
        (FAM_MEDIUM, DATA_MEDIUM),
        (FAM_SEMIBOLD, DATA_SEMIBOLD),
        (FAM_DISPLAY, DATA_DISPLAY),
    ] {
        let mut chain = vec![data_key.to_owned()];
        chain.extend(fallback_prop.iter().cloned());
        defs.families
            .insert(FontFamily::Name(fam_name.into()), chain);
    }

    defs
}

/// Installs the bundled **Roboto Flex** variable font with the requested [`TwoUiTypography`]
/// preset (skips redundant work if already installed with identical parameters).
#[cfg(feature = "font")]
pub fn install_typography(ctx: &Context, typography: TwoUiTypography) {
    let desired = FontState::Bundled(typography.clone());
    if ctx.data(|d| d.get_temp::<FontState>(font_state_id())) == Some(desired.clone()) {
        return;
    }
    let defs = build_variable_font_definitions(FontData::from_static(ROBOTO_FLEX_TTF), &typography);
    ctx.set_fonts(defs);
    ctx.data_mut(|d| d.insert_temp(font_state_id(), desired));
}

/// Ensures bundled typography is initialized once without overwriting a custom preset.
pub(crate) fn ensure_default_typography(ctx: &Context) {
    #[cfg(feature = "font")]
    {
        if !has_twoui_fonts(ctx) {
            install_typography(ctx, TwoUiTypography::tuned());
        }
    }
    #[cfg(not(feature = "font"))]
    let _ = ctx;
}

/// Installs a user-supplied custom `.ttf`/`.otf` variable font with TwoUI weight axes.
pub fn install_custom_variable_font(
    ctx: &Context,
    bytes: &'static [u8],
    typography: TwoUiTypography,
) {
    let defs = build_variable_font_definitions(FontData::from_static(bytes), &typography);
    ctx.set_fonts(defs);
    ctx.data_mut(|d| d.insert_temp(font_state_id(), FontState::CustomVariable(typography)));
}

/// Installs a static `.ttf`/`.otf` custom font bytes slice as the primary proportional font while
/// keeping `egui` fallback glyphs and weight families valid.
pub fn install_custom_font(ctx: &Context, name: &str, bytes: &'static [u8]) {
    let mut defs = FontDefinitions::default();
    let fallback_prop = defs
        .families
        .get(&FontFamily::Proportional)
        .cloned()
        .unwrap_or_default();
    defs.font_data
        .insert(name.to_owned(), Arc::new(FontData::from_static(bytes)));

    let mut chain = vec![name.to_owned()];
    chain.extend(fallback_prop);
    defs.families.insert(FontFamily::Proportional, chain.clone());
    for fam_name in [FAM_MEDIUM, FAM_SEMIBOLD, FAM_DISPLAY] {
        defs.families
            .insert(FontFamily::Name(fam_name.into()), chain.clone());
    }
    ctx.set_fonts(defs);
    ctx.data_mut(|d| d.insert_temp(font_state_id(), FontState::CustomStatic));
}

/// Restores `egui`'s default built-in fonts while keeping TwoUI family aliases valid.
pub fn reset_default_fonts(ctx: &Context) {
    let desired = FontState::EguiDefault;
    if ctx.data(|d| d.get_temp::<FontState>(font_state_id())) == Some(desired.clone()) {
        return;
    }
    let mut defs = FontDefinitions::default();
    let default_prop = defs
        .families
        .get(&FontFamily::Proportional)
        .cloned()
        .unwrap_or_default();
    for fam_name in [FAM_MEDIUM, FAM_SEMIBOLD, FAM_DISPLAY] {
        defs.families
            .insert(FontFamily::Name(fam_name.into()), default_prop.clone());
    }
    ctx.set_fonts(defs);
    ctx.data_mut(|d| d.insert_temp(font_state_id(), desired));
}

/// Returns a [`FontId`] for `size` and semantic [`TwoUiFontWeight`], falling back to
/// `Proportional` when TwoUI font definitions have not been installed.
pub fn font_id_for(ctx: &Context, size: f32, weight: TwoUiFontWeight) -> FontId {
    let family = if has_twoui_fonts(ctx) {
        weight.to_family()
    } else {
        FontFamily::Proportional
    };
    FontId::new(size, family)
}

/// Convenience wrapper over [`font_id_for`] taking `&Ui`.
pub fn font_id(ui: &Ui, size: f32, weight: TwoUiFontWeight) -> FontId {
    font_id_for(ui.ctx(), size, weight)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "font")]
    #[test]
    fn bundled_roboto_flex_exposes_expected_variable_axes() {
        let data = FontData::from_static(ROBOTO_FLEX_TTF);
        let axes = data.variation_axes();
        let tags: Vec<String> = axes.iter().map(|a| a.tag.to_string()).collect();
        for expected in ["wght", "wdth", "opsz", "GRAD", "XTRA", "YTLC", "YOPQ"] {
            assert!(
                tags.iter().any(|t| t == expected),
                "missing Roboto Flex axis `{expected}` in {tags:?}"
            );
        }
    }

    #[cfg(feature = "font")]
    #[test]
    fn variable_font_definitions_share_blob_across_weights() {
        let typo = TwoUiTypography::tuned();
        let defs = build_variable_font_definitions(FontData::from_static(ROBOTO_FLEX_TTF), &typo);
        assert!(defs.font_data.contains_key(DATA_REGULAR));
        assert!(defs.font_data.contains_key(DATA_MEDIUM));
        assert!(defs.font_data.contains_key(DATA_SEMIBOLD));
        assert!(defs.font_data.contains_key(DATA_DISPLAY));
        assert!(defs
            .families
            .contains_key(&TwoUiFontWeight::SemiBold.to_family()));
    }
}
