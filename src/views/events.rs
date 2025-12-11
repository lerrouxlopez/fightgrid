use crate::controllers::EventsController;
use crate::repositories::SqliteEventRepository;
use eframe::egui::{self, Color32, Margin, Vec2};

pub fn render(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    main_width: f32,
    row_height: f32,
    controller: &mut EventsController<SqliteEventRepository>,
) {
    ui.vertical(|ui| {
        ui.set_width(main_width);
        ui.set_min_height(row_height);
        ui.set_max_width(main_width);
        section_frame(
            ui,
            "Events",
            "Name and description. Stored in fightgrid.db (SQLite).",
            |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("All events")
                            .color(Color32::from_rgb(210, 220, 230))
                            .strong(),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .add(
                                egui::Button::new("Add Event")
                                    .fill(Color32::from_rgb(75, 140, 85))
                                    .min_size(Vec2::new(110.0, 28.0)),
                            )
                            .clicked()
                        {
                            controller.open_add_modal();
                        }
                    });
                });
                ui.add_space(8.0);

                egui::ScrollArea::horizontal()
                    .id_source("events_table_scroll")
                    .show(ui, |ui| {
                        let spacing_x = 12.0;
                        let outer_padding = 16.0;
                        let cols = 3.0;
                        let col_width = ((main_width - outer_padding) - spacing_x * (cols - 1.0))
                            / cols;
                        ui.set_width(main_width);
                        ui.set_max_width(main_width);

                        // Headers
                        egui::Frame::none()
                            .fill(Color32::from_rgb(24, 28, 36))
                            .inner_margin(Margin::symmetric(8.0, 6.0))
                            .show(ui, |ui| {
                                egui::Grid::new("events_header")
                                    .num_columns(3)
                                    .spacing([spacing_x, 6.0])
                                    .min_col_width(col_width)
                                    .show(ui, |grid| {
                                        grid.label(header("Name"));
                                        grid.label(header("Description"));
                                        grid.label(header("Actions"));
                                    });
                            });

                        ui.separator();

                        let events = controller.events();
                        let row_fill = Color32::from_rgb(22, 24, 30);

                        for (idx, event) in events.iter().enumerate() {
                            if let Some(id) = event.id {
                                egui::Frame::none()
                                    .fill(if idx % 2 == 0 { row_fill } else { Color32::from_rgb(18, 20, 26) })
                                    .inner_margin(Margin::symmetric(8.0, 6.0))
                                    .show(ui, |ui| {
                                        egui::Grid::new(format!("event_row_{id}"))
                                            .num_columns(3)
                                            .spacing([spacing_x, 6.0])
                                            .min_col_width(col_width)
                                            .show(ui, |grid| {
                                                grid.label(
                                                    egui::RichText::new(&event.name)
                                                        .color(Color32::from_rgb(230, 235, 245)),
                                                );
                                                grid.label(
                                                    egui::RichText::new(&event.description)
                                                        .color(Color32::from_rgb(200, 205, 220)),
                                                );
                                                grid.horizontal(|ui| {
                                                    if ui
                                                        .add(
                                                            egui::Button::new("Edit")
                                                                .fill(Color32::from_rgb(55, 150, 140))
                                                                .min_size(Vec2::new(64.0, 26.0)),
                                                        )
                                                        .clicked()
                                                    {
                                                        controller.open_edit_modal(event);
                                                    }
                                                    if ui
                                                        .add(
                                                            egui::Button::new("Delete")
                                                                .fill(Color32::from_rgb(190, 70, 70))
                                                                .min_size(Vec2::new(72.0, 26.0)),
                                                        )
                                                        .clicked()
                                                    {
                                                        controller.delete_event(id);
                                                    }
                                                });
                                            });
                                    });
                            }
                        }
                    });

                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new("Showing all events")
                        .color(Color32::from_rgb(160, 170, 190)),
                );

                render_add_modal(ctx, controller);
                render_edit_modal(ctx, controller);
            },
        );
    });
}

fn header(text: &str) -> egui::RichText {
    egui::RichText::new(text)
        .color(Color32::from_rgb(200, 210, 230))
        .strong()
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

fn render_add_modal(ctx: &egui::Context, controller: &mut EventsController<SqliteEventRepository>) {
    if controller.add_modal_open() {
        let mut open = true;
        egui::Window::new("Add Event")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                let (new_name, new_desc) = controller.new_fields();
                ui.label("Name");
                ui.text_edit_singleline(new_name);
                ui.label("Description");
                ui.text_edit_singleline(new_desc);
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    if ui
                        .add(
                            egui::Button::new("Add")
                                .fill(Color32::from_rgb(75, 140, 85))
                                .min_size(Vec2::new(80.0, 26.0)),
                        )
                        .clicked()
                    {
                        controller.submit_new();
                    }
                    if ui.button("Cancel").clicked() {
                        controller.close_add_modal();
                    }
                });
            });
        if !open {
            controller.close_add_modal();
        }
    }
}

fn render_edit_modal(ctx: &egui::Context, controller: &mut EventsController<SqliteEventRepository>) {
    if controller.edit_modal_open() {
        let mut open = true;
        egui::Window::new("Edit Event")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                let (id_opt, name, desc) = controller.edit_fields();
                ui.label("Name");
                ui.text_edit_singleline(name);
                ui.label("Description");
                ui.text_edit_singleline(desc);
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    if ui
                        .add(
                            egui::Button::new("Save")
                                .fill(Color32::from_rgb(55, 150, 140))
                                .min_size(Vec2::new(80.0, 26.0)),
                        )
                        .clicked()
                    {
                        if let Some(id) = id_opt {
                            controller.save_edit(id);
                        }
                    }
                    if ui.button("Cancel").clicked() {
                        controller.close_edit_modal();
                    }
                });
            });
        if !open {
            controller.close_edit_modal();
        }
    }
}
