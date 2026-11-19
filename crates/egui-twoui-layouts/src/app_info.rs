//! Application "About" view.

use egui::{vec2, RichText, Ui};
use egui_twoui_tokens::theme_from_ui;
use egui_twoui_widgets::{
    button::{TwoUiButton, TwoUiButtonStyle},
    progress::{TwoUiSpinner, SpinnerSize},
};

/// Update status displayed in [`AppInfoView`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AppUpdateState {
    #[default]
    LatestInstalled,
    Checking,
    UpdateAvailable,
}

/// About / AppInfo screen layout.
pub struct AppInfoView<'a> {
    app_name: &'a str,
    version: &'a str,
    update_state: AppUpdateState,
    links: &'a [&'a str],
}

impl<'a> AppInfoView<'a> {
    pub fn new(app_name: &'a str, version: &'a str) -> Self {
        Self {
            app_name,
            version,
            update_state: AppUpdateState::LatestInstalled,
            links: &[],
        }
    }

    pub fn update_state(mut self, state: AppUpdateState) -> Self {
        self.update_state = state;
        self
    }

    pub fn links(mut self, links: &'a [&'a str]) -> Self {
        self.links = links;
        self
    }

    /// Shows the AppInfo layout. Returns `(update_clicked, clicked_link_index)`.
    pub fn show(self, ui: &mut Ui) -> (bool, Option<usize>) {
        let theme = theme_from_ui(ui);
        let p = &theme.palette;
        let mut update_clicked = false;
        let mut link_clicked = None;

        ui.vertical_centered(|ui| {
            ui.add_space(36.0);
            ui.label(
                RichText::new(self.app_name)
                    .size(34.0)
                    .strong()
                    .color(p.text_primary),
            );
            ui.add_space(16.0);
            ui.label(
                RichText::new(format!("Version {}", self.version))
                    .size(13.5)
                    .color(p.text_muted),
            );
            ui.add_space(4.0);

            match self.update_state {
                AppUpdateState::LatestInstalled => {
                    ui.label(
                        RichText::new("The latest version is installed")
                            .size(13.5)
                            .color(p.text_muted),
                    );
                }
                AppUpdateState::Checking => {
                    ui.add_space(8.0);
                    ui.add(TwoUiSpinner::indeterminate(SpinnerSize::Small));
                }
                AppUpdateState::UpdateAvailable => {
                    ui.label(
                        RichText::new("A new version is available")
                            .size(13.5)
                            .color(p.primary_high),
                    );
                    ui.add_space(12.0);
                    if ui
                        .add(
                            TwoUiButton::contained_primary("Update")
                                .min_size(vec2(220.0, 48.0)),
                        )
                        .clicked()
                    {
                        update_clicked = true;
                    }
                }
            }

            if !self.links.is_empty() {
                ui.add_space(40.0);
                for (i, link) in self.links.iter().enumerate() {
                    if ui
                        .add(
                            TwoUiButton::new(*link)
                                .style(TwoUiButtonStyle::Contained)
                                .min_size(vec2(236.0, 46.0)),
                        )
                        .clicked()
                    {
                        link_clicked = Some(i);
                    }
                    ui.add_space(8.0);
                }
            }
        });

        (update_clicked, link_clicked)
    }
}
