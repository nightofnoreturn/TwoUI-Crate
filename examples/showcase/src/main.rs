//! TwoUI Showcase application.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod data;
mod dialogs;
mod pages;

use app::ShowcaseApp;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("TwoUI Showcase")
            .with_decorations(false)
            .with_inner_size([1120.0, 790.0])
            .with_min_inner_size([420.0, 540.0]),
        ..Default::default()
    };

    eframe::run_native(
        "TwoUI Showcase",
        options,
        Box::new(|cc| Ok(Box::new(ShowcaseApp::new(cc)))),
    )
}
