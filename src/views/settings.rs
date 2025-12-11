use crate::controllers::SettingsInfo;
use eframe::egui::{self, Color32, Margin};

pub fn render(
    ui: &mut egui::Ui,
    main_width: f32,
    row_height: f32,
    settings: &SettingsInfo,
) {
    ui.vertical(|ui| {
        ui.set_width(main_width);
        ui.set_min_height(row_height);
        section_frame(ui, "Settings", "Brand, automation, and safety options.", |ui| {
            ui.label(
                egui::RichText::new("Organization")
                    .color(Color32::from_rgb(210, 220, 235))
                    .strong(),
            );
            ui.colored_label(Color32::from_rgb(170, 190, 210), settings.organization);
            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);

            ui.label(
                egui::RichText::new("Automation")
                    .color(Color32::from_rgb(210, 220, 235))
                    .strong(),
            );
            for auto in settings.automation {
                ui.label(
                    egui::RichText::new(*auto)
                        .color(Color32::from_rgb(150, 220, 170)),
                );
            }
            ui.colored_label(
                Color32::from_rgb(170, 190, 210),
                settings.notifications,
            );
            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);

            ui.label(
                egui::RichText::new("Safety")
                    .color(Color32::from_rgb(210, 220, 235))
                    .strong(),
            );
            for safety in settings.safety {
                ui.label(
                    egui::RichText::new(*safety)
                        .color(Color32::from_rgb(255, 200, 130)),
                );
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
