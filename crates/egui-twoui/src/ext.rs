//! Extension trait [`TwoUiExt`] on [`egui::Ui`].

use std::ops::RangeInclusive;

use egui::{InnerResponse, Response, Ui};
use egui_twoui_widgets::{
    button::{TwoUiButton, TwoUiButtonStyle},
    card::{
        paint_divider, CardItem, CardSection, DividerInset, SectionHeader, SwitchItem,
        SwitchItemResponse,
    },
    controls::{TwoUiCheckBox, TwoUiRadioButton, TwoUiSwitch},
    seekbar::{TwoUiSeekBar, SeekBarMode},
};

/// Extension methods adding TwoUI widgets directly onto [`egui::Ui`].
pub trait TwoUiExt {
    fn twoui_section_header(&mut self, title: impl Into<String>);
    fn twoui_card<R>(&mut self, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R>;
    fn twoui_divider(&mut self);
    fn twoui_divider_inset(&mut self, inset: DividerInset);
    fn twoui_button(&mut self, text: impl Into<String>) -> Response;
    fn twoui_primary_button(&mut self, text: impl Into<String>) -> Response;
    fn twoui_contained_button(&mut self, text: impl Into<String>, primary: bool) -> Response;
    fn twoui_switch(&mut self, checked: &mut bool, label: impl Into<String>) -> Response;
    fn twoui_checkbox(&mut self, checked: &mut bool, label: impl Into<String>) -> Response;
    fn twoui_radio(&mut self, selected: bool, label: impl Into<String>) -> Response;
    fn twoui_card_item(&mut self, title: impl Into<String>) -> Response;
    fn twoui_switch_item(&mut self, checked: &mut bool, title: impl Into<String>) -> SwitchItemResponse;
    fn twoui_seekbar(&mut self, value: &mut f32, range: RangeInclusive<f32>, mode: SeekBarMode) -> Response;
}

impl TwoUiExt for Ui {
    fn twoui_section_header(&mut self, title: impl Into<String>) {
        SectionHeader::new(title).show(self);
    }

    fn twoui_card<R>(&mut self, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        CardSection::new().show(self, add_contents)
    }

    fn twoui_divider(&mut self) {
        paint_divider(self, DividerInset::Standard);
    }

    fn twoui_divider_inset(&mut self, inset: DividerInset) {
        paint_divider(self, inset);
    }

    fn twoui_button(&mut self, text: impl Into<String>) -> Response {
        self.add(TwoUiButton::new(text))
    }

    fn twoui_primary_button(&mut self, text: impl Into<String>) -> Response {
        self.add(TwoUiButton::primary(text))
    }

    fn twoui_contained_button(&mut self, text: impl Into<String>, primary: bool) -> Response {
        let style = if primary {
            TwoUiButtonStyle::ContainedPrimary
        } else {
            TwoUiButtonStyle::Contained
        };
        self.add(TwoUiButton::new(text).style(style))
    }

    fn twoui_switch(&mut self, checked: &mut bool, label: impl Into<String>) -> Response {
        self.add(TwoUiSwitch::new(checked).label(label))
    }

    fn twoui_checkbox(&mut self, checked: &mut bool, label: impl Into<String>) -> Response {
        self.add(TwoUiCheckBox::new(checked).label(label))
    }

    fn twoui_radio(&mut self, selected: bool, label: impl Into<String>) -> Response {
        self.add(TwoUiRadioButton::new(selected).label(label))
    }

    fn twoui_card_item(&mut self, title: impl Into<String>) -> Response {
        self.add(CardItem::new(title))
    }

    fn twoui_switch_item(
        &mut self,
        checked: &mut bool,
        title: impl Into<String>,
    ) -> SwitchItemResponse {
        SwitchItem::new(checked, title).show(self)
    }

    fn twoui_seekbar(
        &mut self,
        value: &mut f32,
        range: RangeInclusive<f32>,
        mode: SeekBarMode,
    ) -> Response {
        self.add(TwoUiSeekBar::new(value, range).mode(mode))
    }
}
