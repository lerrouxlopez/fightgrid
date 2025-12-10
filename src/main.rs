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

struct FightGridApp {
    nav_items: Vec<&'static str>,
}

impl FightGridApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            nav_items: vec!["Home", "Leaderboard", "Players", "Reports", "Settings"],
        }
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
            .frame(
                egui::Frame::none()
                    .fill(Color32::from_rgb(12, 14, 18))
                    .inner_margin(Margin {
                        left: 5.0,
                        right: 5.0,
                        top: 0.0,
                        bottom: 5.0,
                    }),
            )
            .show(ctx, |ui| {
                let available = ui.available_size();
                let gap = 12.0;
                let nav_width = (available.x * 0.4).max(220.0); // 2/5 columns
                let main_width = (available.x - nav_width - gap).max(320.0); // remaining 3/5 columns
                let row_height = available.y;

                ui.horizontal(|ui| {
                    ui.set_height(row_height);

                    // Column 1: side navigation (1/3 width)
                    ui.vertical(|ui| {
                        ui.set_width(nav_width);
                        egui::Frame::group(ui.style())
                            .fill(Color32::from_rgb(24, 26, 32))
                            .stroke(egui::Stroke::new(1.0, Color32::from_rgb(60, 70, 90)))
                            .inner_margin(Margin::symmetric(12.0, 10.0))
                            .show(ui, |ui| {
                                ui.heading(
                                    egui::RichText::new("Navigation")
                                        .size(18.0)
                                        .color(Color32::from_rgb(220, 225, 235)),
                                );
                                ui.separator();
                                ui.add_space(6.0);
                                for item in &self.nav_items {
                                    ui.add_space(4.0);
                                    let button = egui::Button::new(
                                        egui::RichText::new(*item)
                                            .size(14.0)
                                            .color(Color32::WHITE),
                                    )
                                    .fill(Color32::from_rgb(40, 44, 52))
                                    .stroke(egui::Stroke::new(
                                        1.0,
                                        Color32::from_rgb(80, 90, 110),
                                    ))
                                    .min_size(Vec2::new(nav_width - 24.0, 32.0));
                                    ui.add(button);
                                }
                            });
                    });

                    ui.add_space(12.0);

                    // Columns 2 + 3: main screen area
                    ui.vertical(|ui| {
                        ui.set_width(main_width);
                        egui::Frame::group(ui.style())
                            .fill(Color32::from_rgb(18, 18, 26))
                            .stroke(egui::Stroke::new(1.0, Color32::from_rgb(70, 70, 90)))
                            .inner_margin(Margin::symmetric(14.0, 12.0))
                            .show(ui, |ui| {
                                ui.heading(
                                    egui::RichText::new("Main Screen")
                                        .size(18.0)
                                        .color(Color32::from_rgb(235, 235, 245)),
                                );
                                ui.separator();
                                ui.add_space(8.0);
                                ui.label(
                                    egui::RichText::new(
                                        "Grid layout: banner spans 3 columns; navigation uses column 1; this area spans columns 2 and 3.",
                                    )
                                    .color(Color32::from_rgb(190, 200, 215)),
                                );
                                ui.add_space(12.0);
                                ui.label(
                                    egui::RichText::new("Placeholder content for main screen.")
                                        .color(Color32::from_rgb(150, 160, 180)),
                                );
                            });
                    });
                });
            });
    }
}
