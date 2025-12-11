use crate::controllers::ResultInfo;
use eframe::egui::{self, Color32, Margin};

pub fn render(ui: &mut egui::Ui, main_width: f32, row_height: f32, results: &[ResultInfo]) {
    ui.vertical(|ui| {
        ui.set_width(main_width);
        ui.set_min_height(row_height);
        section_frame(ui, "Results", "Latest completed bouts and scorecards.", |ui| {
            for result in results {
                egui::Frame::none()
                    .fill(Color32::from_rgb(24, 26, 34))
                    .stroke(egui::Stroke::new(1.0, Color32::from_rgb(55, 65, 90)))
                    .inner_margin(Margin::symmetric(10.0, 8.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(result.bout)
                                    .color(Color32::WHITE)
                                    .strong(),
                            );
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.colored_label(
                                        Color32::from_rgb(255, 200, 130),
                                        result.outcome,
                                    );
                                },
                            );
                        });
                    });
                ui.add_space(4.0);
            }
        });
    });
}

fn section_frame<F: FnOnce(&mut egui::Ui)>(
    ui: &mut egui::Ui,
    title: &str,
    subtitle: &str,
    content: F,
) {
    egui::Frame::group(ui.style())
        .fill(Color32::from_rgb(18, 18, 26))
        .stroke(egui::Stroke::new(1.0, Color32::from_rgb(70, 70, 90)))
        .inner_margin(Margin::symmetric(14.0, 12.0))
        .show(ui, |ui| {
            ui.heading(
                egui::RichText::new(title)
                    .size(18.0)
                    .color(Color32::from_rgb(235, 235, 245)),
            );
            if !subtitle.is_empty() {
                ui.label(
                    egui::RichText::new(subtitle)
                        .color(Color32::from_rgb(170, 180, 200)),
                );
            }
            ui.separator();
            ui.add_space(10.0);
            content(ui);
        });
}
