use eframe::egui;
use egui::{Color32, FontId, Margin, Vec2};

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

struct FightGridApp;

impl FightGridApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self
    }
}

impl eframe::App for FightGridApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("banner")
            .exact_height(78.0)
            .show(ctx, |ui| {
                ui.add_space(6.0);
                egui::Frame::none()
                    .fill(Color32::from_rgb(26, 28, 32))
                    .inner_margin(Margin::symmetric(18.0, 12.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let logo_size = 46.0;
                            let (rect, _) =
                                ui.allocate_exact_size(Vec2::splat(logo_size), egui::Sense::hover());
                            let painter = ui.painter_at(rect);
                            painter.circle_filled(
                                rect.center(),
                                logo_size * 0.5,
                                Color32::from_rgb(210, 65, 92),
                            );
                            painter.text(
                                rect.center(),
                                egui::Align2::CENTER_CENTER,
                                "FG",
                                FontId::proportional(18.0),
                                Color32::WHITE,
                            );
                            ui.add_space(10.0);
                            ui.vertical(|ui| {
                                ui.label(
                                    egui::RichText::new("FightGrid")
                                        .size(24.0)
                                        .color(Color32::WHITE)
                                        .strong(),
                                );
                                ui.label(
                                    egui::RichText::new(
                                        "Every Strike. Every Round. Every Bracket.",
                                    )
                                    .color(Color32::from_rgb(180, 200, 225)),
                                );
                            });
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(
                                    egui::RichText::new("Tournament Dashboard")
                                        .color(Color32::from_rgb(150, 160, 175)),
                                );
                            });
                        });
                    });
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(Color32::from_rgb(12, 14, 18)))
            .show(ctx, |_ui| {});
    }
}
