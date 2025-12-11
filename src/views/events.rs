use crate::controllers::EventInfo;
use eframe::egui::{self, Color32, Margin};

pub fn render(ui: &mut egui::Ui, main_width: f32, row_height: f32, events: &[EventInfo]) {
    ui.vertical(|ui| {
        ui.set_width(main_width);
        ui.set_min_height(row_height);
        section_frame(ui, "Events", "Fight night calendar and registration windows.", |ui| {
            for event in events {
                egui::Frame::none()
                    .fill(Color32::from_rgb(26, 28, 36))
                    .stroke(egui::Stroke::new(1.0, Color32::from_rgb(60, 70, 90)))
                    .inner_margin(Margin::symmetric(10.0, 8.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label(
                                    egui::RichText::new(event.title)
                                        .color(Color32::from_rgb(230, 235, 245))
                                        .strong(),
                                );
                                ui.label(
                                    egui::RichText::new(event.subtitle)
                                        .color(Color32::from_rgb(170, 180, 200)),
                                );
                            });
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.label(
                                        egui::RichText::new(event.status)
                                            .color(Color32::from_rgb(140, 200, 255)),
                                    );
                                },
                            );
                        });
                    });
                ui.add_space(6.0);
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
