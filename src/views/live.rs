use crate::controllers::LiveBlock;
use eframe::egui::{self, Color32, Margin};

pub fn render(
    ui: &mut egui::Ui,
    main_width: f32,
    row_height: f32,
    blocks: &[LiveBlock],
    callouts: &str,
) {
    ui.vertical(|ui| {
        ui.set_width(main_width);
        ui.set_min_height(row_height);
        section_frame(
            ui,
            "Live",
            "Production checklist and stream monitors.",
            |ui| {
                ui.horizontal(|ui| {
                    for block in blocks {
                        ui.vertical(|ui| {
                            egui::Frame::none()
                                .fill(Color32::from_rgb(25, 30, 38))
                                .stroke(egui::Stroke::new(1.0, Color32::from_rgb(70, 80, 110)))
                                .inner_margin(Margin::symmetric(10.0, 8.0))
                                .show(ui, |ui| {
                                    ui.label(
                                        egui::RichText::new(block.title)
                                            .color(Color32::from_rgb(220, 230, 245))
                                            .strong(),
                                    );
                                    ui.colored_label(status_color(block.title), block.status);
                                });
                        });
                        ui.add_space(10.0);
                    }
                });
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new("Live callouts")
                        .color(Color32::from_rgb(170, 190, 210)),
                );
                ui.colored_label(Color32::from_rgb(255, 160, 140), callouts);
            },
        );
    });
}

fn status_color(title: &str) -> Color32 {
    match title {
        "Cage Feed" => Color32::from_rgb(140, 235, 180),
        "Commentary" => Color32::from_rgb(255, 200, 130),
        "Clock/Scoring" => Color32::from_rgb(150, 210, 255),
        _ => Color32::LIGHT_GRAY,
    }
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
