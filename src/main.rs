mod app;
mod controllers;
mod models;
mod repositories;
mod services;
mod views;

use crate::app::FightGridApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1600.0, 900.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "FightGrid",
        native_options,
        Box::new(|cc| Box::new(FightGridApp::new(cc))),
    )
}
