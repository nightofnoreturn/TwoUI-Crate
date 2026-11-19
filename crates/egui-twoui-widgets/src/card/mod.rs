//! Grouped cards, preference rows, tips, suggestions, and related links.

mod item;
mod links;
mod section;
mod tips_suggestion;
mod toggle_items;

pub use item::CardItem;
pub use links::RelativeLinksCard;
pub use section::{paint_divider, CardSection, DividerInset, TwoUiPreferenceCaption, SectionHeader};
pub use tips_suggestion::{SuggestionCard, SuggestionCardResponse, TipsCard, TipsCardResponse};
pub use toggle_items::{RadioItem, SwitchItem, SwitchItemResponse};

use std::sync::Arc;

use egui::{Color32, FontFamily, FontId, Galley, Ui};
use egui_twoui_tokens::tokens::dimens;

pub(crate) struct RowTextGalleys {
    pub title: Arc<Galley>,
    pub summary: Option<Arc<Galley>>,
    pub block_height: f32,
}

pub(crate) fn layout_row_galleys(
    ui: &Ui,
    title: &str,
    title_col: Color32,
    summary: Option<&str>,
    summary_col: Color32,
    max_width: f32,
) -> RowTextGalleys {
    let title_galley = ui.painter().layout(
        title.to_owned(),
        FontId::new(dimens::TEXT_LIST_PRIMARY, FontFamily::Proportional),
        title_col,
        max_width,
    );
    let summary_galley = summary.map(|s| {
        ui.painter().layout(
            s.to_owned(),
            FontId::new(dimens::TEXT_LIST_SECONDARY, FontFamily::Proportional),
            summary_col,
            max_width,
        )
    });
    let block_height =
        title_galley.size().y + summary_galley.as_ref().map_or(0.0, |g| 3.0 + g.size().y);
    RowTextGalleys {
        title: title_galley,
        summary: summary_galley,
        block_height,
    }
}
