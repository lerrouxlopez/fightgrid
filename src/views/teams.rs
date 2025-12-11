use crate::controllers::TeamInfo;
use eframe::egui::{self, Color32, Margin};

pub fn render(ui: &mut egui::Ui, main_width: f32, row_height: f32, teams: &[TeamInfo]) {
    ui.vertical(|ui| {
        ui.set_width(main_width);
        ui.set_min_height(row_height);
        section_frame(ui, "Teams", "Manage team pools and roster approvals.", |ui| {
            egui::Grid::new("teams_grid")
                .num_columns(3)
                .min_col_width(120.0)
                .striped(true)
                .show(ui, |grid| {
                    grid.label(
                        egui::RichText::new("Team")
                            .color(Color32::from_rgb(200, 210, 230))
                            .strong(),
                    );
                    grid.label(
                        egui::RichText::new("Roster")
                            .color(Color32::from_rgb(200, 210, 230))
                            .strong(),
                    );
                    grid.label(
                        egui::RichText::new("Status")
                            .color(Color32::from_rgb(200, 210, 230))
                            .strong(),
                    );
                    grid.end_row();

                    for team in teams {
                        grid.label(
                            egui::RichText::new(team.name)
                                .color(Color32::WHITE)
                                .strong(),
                        );
                        grid.label(
                            egui::RichText::new(team.roster)
                                .color(Color32::from_rgb(170, 200, 210)),
                        );
                        grid.label(
                            egui::RichText::new(team.status)
                                .color(Color32::from_rgb(150, 220, 170)),
                        );
                        grid.end_row();
                    }
                });
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
