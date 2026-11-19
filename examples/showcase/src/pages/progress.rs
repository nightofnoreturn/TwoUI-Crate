//! Progress indicators & spinners showcase page.

use egui::{Margin, RichText, Ui};
use egui_twoui::prelude::*;

use crate::app::ShowcaseApp;

impl ShowcaseApp {
    pub(crate) fn render_progress_page(&mut self, ui: &mut Ui) {
        if self.auto_animate_progress {
            let t = ui.input(|i| i.time) as f32;
            ui.ctx().request_repaint();
            self.determinate_progress = (t * 0.125) % 1.0;
        }

        SectionHeader::new("Indeterminate").show(ui);
        CardSection::new()
            .inner_margin(Margin::symmetric(20, 18))
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 16.0;
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 22.0;
                    ui.add(TwoUiSpinner::indeterminate(SpinnerSize::SmallTitle));
                    ui.add(TwoUiSpinner::indeterminate(SpinnerSize::Small));
                    ui.add(TwoUiSpinner::indeterminate(SpinnerSize::Medium));
                    ui.add(TwoUiSpinner::indeterminate(SpinnerSize::Large));
                    ui.add(TwoUiSpinner::indeterminate(SpinnerSize::XLarge));
                });

                ui.add(TwoUiProgressBar::indeterminate());
            });

        SectionHeader::new("Determinate & Gradient").show(ui);
        CardSection::new()
            .inner_margin(Margin::symmetric(20, 18))
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 14.0;
                let grad = self.gradient_progress_mode;
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 22.0;
                    ui.add(
                        TwoUiSpinner::determinate(SpinnerSize::SmallTitle, self.determinate_progress)
                            .gradient(grad),
                    );
                    ui.add(
                        TwoUiSpinner::determinate(SpinnerSize::Small, self.determinate_progress)
                            .gradient(grad),
                    );
                    ui.add(
                        TwoUiSpinner::determinate(SpinnerSize::Medium, self.determinate_progress)
                            .gradient(grad),
                    );
                    ui.add(
                        TwoUiSpinner::determinate(SpinnerSize::Large, self.determinate_progress)
                            .gradient(grad),
                    );
                    ui.add(
                        TwoUiSpinner::determinate(SpinnerSize::XLarge, self.determinate_progress)
                            .gradient(grad),
                    );
                });

                ui.add(TwoUiProgressBar::determinate(self.determinate_progress).gradient(grad));

                paint_divider(ui, DividerInset::FullWidth);
                ui.horizontal_wrapped(|ui| {
                    ui.twoui_checkbox(&mut self.auto_animate_progress, "Auto loop (0..100%)");
                    ui.add_space(12.0);
                    ui.twoui_checkbox(&mut self.gradient_progress_mode, "Gradient Sweep");
                });
                let muted = theme_from_ui(ui).palette.text_muted;
                ui.label(
                    RichText::new(format!(
                        "Manual progress ({:.0}%)",
                        self.determinate_progress * 100.0
                    ))
                    .size(13.0)
                    .color(muted),
                );
                if ui
                    .add(
                        TwoUiSeekBar::new(&mut self.determinate_progress, 0.0..=1.0)
                            .mode(SeekBarMode::Expand),
                    )
                    .changed()
                {
                    self.auto_animate_progress = false;
                }
            });
    }
}
