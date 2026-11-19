use egui::{pos2, vec2, Color32, CornerRadius, FontFamily, FontId, Frame, Margin, Sense, Stroke, Ui};
use egui_twoui_tokens::{font_id, theme_from_ui, tokens::dimens, TwoUiFontWeight};

/// Section subheader label above a [`CardSection`].
pub struct SectionHeader {
    text: String,
    top_space: f32,
}

impl SectionHeader {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            top_space: 12.0,
        }
    }

    pub fn top_space(mut self, top_space: f32) -> Self {
        self.top_space = top_space;
        self
    }

    pub fn show(self, ui: &mut Ui) {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        ui.add_space(self.top_space);
        let width = ui.available_width().max(80.0);
        if self.text.is_empty() {
            ui.allocate_exact_size(vec2(width, 14.0), Sense::hover());
            return;
        }
        let header_font = font_id(ui, dimens::TEXT_SUBHEADER, TwoUiFontWeight::Medium);
        let galley = ui.painter().layout_no_wrap(self.text, header_font, p.text_muted);
        let h = galley.size().y + 10.0;
        let (rect, _) = ui.allocate_exact_size(vec2(width, h), Sense::hover());
        ui.painter().galley(
            pos2(rect.left() + 18.0, rect.bottom() - galley.size().y - 6.0),
            galley,
            p.text_muted,
        );
    }
}

/// Divider inset mode inside a [`CardSection`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DividerInset {
    /// Standard horizontal inset matching card item padding (`20px`).
    #[default]
    Standard,
    /// Deeper left inset aligning text past leading icon or radio circle (`60px`).
    WithLeading,
    /// Spans full card width.
    FullWidth,
}

/// Paints a hairline divider inside a card or list.
pub fn paint_divider(ui: &mut Ui, inset: DividerInset) {
    let theme = theme_from_ui(ui);
    let p = &theme.palette;
    let width = ui.available_width().max(20.0);
    let (rect, _) = ui.allocate_exact_size(vec2(width, 1.0), Sense::hover());
    if ui.is_rect_visible(rect) {
        let (left_pad, right_pad) = match inset {
            DividerInset::Standard => (
                dimens::LIST_HORIZONTAL_PADDING,
                dimens::LIST_HORIZONTAL_PADDING,
            ),
            DividerInset::WithLeading => (60.0, dimens::LIST_HORIZONTAL_PADDING),
            DividerInset::FullWidth => (0.0, 0.0),
        };
        let y = rect.center().y;
        ui.painter().line_segment(
            [
                pos2(rect.left() + left_pad, y),
                pos2((rect.right() - right_pad).max(rect.left() + left_pad), y),
            ],
            Stroke::new(1.0, p.divider),
        );
    }
}

/// Rounded grouped container (`26dp` corner radius).
pub struct CardSection {
    inner_margin: Margin,
    corner_radius: Option<u8>,
    fill: Option<Color32>,
}

impl Default for CardSection {
    fn default() -> Self {
        Self::new()
    }
}

impl CardSection {
    pub fn new() -> Self {
        Self {
            inner_margin: Margin::symmetric(0, 6),
            corner_radius: None,
            fill: None,
        }
    }

    /// Sets inner padding inside the rounded card.
    pub fn inner_margin(mut self, margin: Margin) -> Self {
        self.inner_margin = margin;
        self
    }

    pub fn corner_radius(mut self, radius: u8) -> Self {
        self.corner_radius = Some(radius);
        self
    }

    pub fn fill(mut self, fill: Color32) -> Self {
        self.fill = Some(fill);
        self
    }

    pub fn show<R>(
        self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> egui::InnerResponse<R> {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let fill = self.fill.unwrap_or(p.surface);
        let radius = self.corner_radius.unwrap_or(theme.card_corner_radius);

        Frame::NONE
            .fill(fill)
            .corner_radius(CornerRadius::same(radius))
            .inner_margin(self.inner_margin)
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.spacing_mut().item_spacing.y = 0.0;
                add_contents(ui)
            })
    }
}

/// List/preference footer caption item (`12sp`, `8dp` top / `16dp` bottom padding, optional trailing link).
pub struct TwoUiPreferenceCaption<'a> {
    summary: &'a str,
    link_text: Option<&'a str>,
}

impl<'a> TwoUiPreferenceCaption<'a> {
    pub fn new(summary: &'a str) -> Self {
        Self {
            summary,
            link_text: None,
        }
    }

    pub fn link(mut self, link_text: &'a str) -> Self {
        self.link_text = Some(link_text);
        self
    }

    /// Shows the caption block and returns `true` if the optional link was clicked.
    pub fn show(self, ui: &mut Ui) -> bool {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let mut link_clicked = false;

        Frame::NONE
            .inner_margin(Margin {
                left: dimens::LIST_HORIZONTAL_PADDING as i8,
                right: dimens::LIST_HORIZONTAL_PADDING as i8,
                top: dimens::LIST_CAPTION_PADDING_TOP as i8,
                bottom: dimens::LIST_CAPTION_PADDING_BOTTOM as i8,
            })
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 4.0;
                ui.label(
                    egui::RichText::new(self.summary)
                        .size(dimens::LIST_CAPTION_TEXT_SIZE)
                        .color(p.text_muted),
                );
                if let Some(link) = self.link_text {
                    let font_id = FontId::new(
                        dimens::LIST_CAPTION_TEXT_SIZE + 1.0,
                        FontFamily::Proportional,
                    );
                    let galley = ui
                        .painter()
                        .layout_no_wrap(link.to_owned(), font_id, p.primary_high);
                    let size = galley.size();
                    let (rect, resp) = ui.allocate_exact_size(size + vec2(2.0, 6.0), Sense::click());
                    if ui.is_rect_visible(rect) {
                        ui.painter()
                            .galley(pos2(rect.left(), rect.top() + 2.0), galley, p.primary_high);
                        let y = rect.bottom() - 2.0;
                        ui.painter().line_segment(
                            [pos2(rect.left(), y), pos2(rect.left() + size.x, y)],
                            Stroke::new(1.0, p.primary_high),
                        );
                    }
                    if resp.clicked() {
                        link_clicked = true;
                    }
                }
            });

        link_clicked
    }
}
