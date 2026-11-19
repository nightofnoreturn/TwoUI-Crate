//! Bundled SVG vector icons rasterized cleanly via `resvg`.

mod svg_paths;

use egui::{
    pos2, vec2, Color32, ColorImage, Context, Id, Pos2, Rect, TextureHandle, TextureOptions, Ui,
    Vec2,
};
use resvg::tiny_skia::{Pixmap, Transform};
use resvg::usvg::{Options, Tree};

pub use svg_paths::TwoUiIcon;

const ICON_RASTER_SIZE: u32 = 96;

impl TwoUiIcon {
    /// All built-in icon variants.
    pub const ALL: [Self; svg_paths::ICON_COUNT] = svg_paths::ALL_ICONS;

    /// Returns the variant name of this icon.
    pub const fn name(self) -> &'static str {
        svg_paths::icon_name(self)
    }

    /// Returns `true` if this icon carries baked multi-color styling rather than a tintable monochrome glyph.
    pub const fn is_multicolor(self) -> bool {
        svg_paths::is_multicolor(self)
    }

    /// Returns the raw SVG document for this icon in normalized white foreground.
    pub const fn svg_markup(self) -> &'static str {
        svg_paths::svg_source(self)
    }

    /// Paints this icon centered inside `rect` tinted with `color`.
    pub fn paint(&self, ui: &Ui, rect: Rect, color: Color32) {
        if !ui.is_rect_visible(rect) {
            return;
        }
        let side = rect.width().min(rect.height()).max(0.0);
        if side <= 0.0 || color.a() == 0 {
            return;
        }
        let Some(texture) = self.cached_texture(ui.ctx()) else {
            return;
        };
        let square = Rect::from_center_size(rect.center(), Vec2::splat(side));
        let uv = Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));
        let tint = if self.is_multicolor() {
            Color32::from_white_alpha(color.a())
        } else {
            color
        };
        ui.painter().image(texture.id(), square, uv, tint);
    }

    /// Allocates a `size x size` square and paints the icon centered inside it.
    pub fn show(&self, ui: &mut Ui, size: f32, color: Color32) -> Pos2 {
        let (rect, _) = ui.allocate_exact_size(vec2(size, size), egui::Sense::hover());
        self.paint(ui, rect, color);
        rect.center()
    }

    fn cached_texture(self, ctx: &Context) -> Option<TextureHandle> {
        let key = Id::unique(("twoui_icon_tex", self));
        if let Some(handle) = ctx.data(|d| d.get_temp::<TextureHandle>(key)) {
            return Some(handle);
        }
        let image = rasterize_svg(self.svg_markup(), ICON_RASTER_SIZE)?;
        let name = format!("twoui_icon_{}", self.name());
        let handle = ctx.load_texture(name, image, TextureOptions::LINEAR);
        ctx.data_mut(|d| {
            d.insert_temp(key, handle.clone());
        });
        Some(handle)
    }
}

/// Rasterizes a square-viewport SVG string into a straight-alpha [`ColorImage`].
pub(crate) fn rasterize_svg(svg: &str, size: u32) -> Option<ColorImage> {
    let opt = Options::default();
    let tree = Tree::from_str(svg, &opt).ok()?;
    let src = tree.size();
    if src.width() <= 0.0 || src.height() <= 0.0 || size == 0 {
        return None;
    }
    let mut pixmap = Pixmap::new(size, size)?;
    let scale_x = size as f32 / src.width();
    let scale_y = size as f32 / src.height();
    resvg::render(
        &tree,
        Transform::from_scale(scale_x, scale_y),
        &mut pixmap.as_mut(),
    );
    let rgba = unpremultiply_rgba(pixmap.data());
    Some(ColorImage::from_rgba_unmultiplied(
        [size as usize, size as usize],
        &rgba,
    ))
}

fn unpremultiply_rgba(premultiplied: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(premultiplied.len());
    for px in premultiplied.chunks_exact(4) {
        let a = px[3];
        match a {
            0 => out.extend_from_slice(&[0, 0, 0, 0]),
            255 => out.extend_from_slice(&[px[0], px[1], px[2], 255]),
            _ => {
                let a16 = u16::from(a);
                let half = a16 / 2;
                let r = ((u16::from(px[0]) * 255 + half) / a16).min(255) as u8;
                let g = ((u16::from(px[1]) * 255 + half) / a16).min(255) as u8;
                let b = ((u16::from(px[2]) * 255 + half) / a16).min(255) as u8;
                out.extend_from_slice(&[r, g, b, a]);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_icon_svg_rasterizes_with_visible_pixels() {
        for icon in TwoUiIcon::ALL {
            let image = rasterize_svg(icon.svg_markup(), 32)
                .unwrap_or_else(|| panic!("failed to rasterize {}", icon.name()));
            assert_eq!(image.size, [32, 32], "unexpected dimensions for {}", icon.name());
            let lit_pixels = image.pixels.iter().filter(|c| c.a() > 0).count();
            assert!(
                lit_pixels > 5,
                "{} rasterized with too few visible pixels ({lit_pixels})",
                icon.name()
            );
            assert!(
                lit_pixels < 32 * 32,
                "{} unexpectedly filled the entire canvas",
                icon.name()
            );
        }
    }

    #[test]
    fn search_icon_preserves_hollow_lens_center() {
        let image = rasterize_svg(TwoUiIcon::Search.svg_markup(), 48).expect("search raster");
        let center_pixel = image.pixels[21 * 48 + 21];
        assert_eq!(center_pixel.a(), 0, "search lens interior must stay transparent");
    }
}
