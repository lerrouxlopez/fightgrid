use eframe::egui;
use egui::{Color32, FontId, Margin, Vec2};

pub fn render(ctx: &egui::Context, nav_items: &[&str], seeds: &[String], palette: &[Color32]) {
    render_banner(ctx);

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
            let gap = 5.0;
            let nav_width = (available.x * 0.2).max(180.0);
            let main_width = (available.x - nav_width - gap - 5.0).max(320.0);
            let row_height = available.y;

            ui.horizontal(|ui| {
                ui.set_height(row_height);

                render_navigation(ui, nav_items, nav_width, row_height);

                ui.add_space(gap);

                render_bracket_container(ui, main_width, row_height, seeds, palette);
            });
        });
}

fn render_banner(ctx: &egui::Context) {
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
                                egui::RichText::new("Every Strike. Every Round. Every Bracket.")
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
}

fn render_navigation(ui: &mut egui::Ui, nav_items: &[&str], nav_width: f32, row_height: f32) {
    ui.vertical(|ui| {
        ui.set_width(nav_width);
        ui.set_min_height(row_height);
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
                for item in nav_items {
                    ui.add_space(4.0);
                    let button = egui::Button::new(
                        egui::RichText::new(*item)
                            .size(14.0)
                            .color(Color32::WHITE),
                    )
                    .fill(Color32::from_rgb(40, 44, 52))
                    .stroke(egui::Stroke::new(1.0, Color32::from_rgb(80, 90, 110)))
                    .min_size(Vec2::new(nav_width - 24.0, 32.0));
                    ui.add(button);
                }
            });
    });
}

fn render_bracket_container(
    ui: &mut egui::Ui,
    main_width: f32,
    row_height: f32,
    seeds: &[String],
    palette: &[Color32],
) {
    ui.vertical(|ui| {
        ui.set_width(main_width);
        ui.set_min_height(row_height);
        egui::Frame::group(ui.style())
            .fill(Color32::from_rgb(18, 18, 26))
            .stroke(egui::Stroke::new(1.0, Color32::from_rgb(70, 70, 90)))
            .inner_margin(Margin::symmetric(14.0, 12.0))
            .show(ui, |ui| {
                ui.heading(
                    egui::RichText::new("Tournament Bracket")
                        .size(18.0)
                        .color(Color32::from_rgb(235, 235, 245)),
                );
                ui.separator();
                ui.add_space(10.0);
                draw_bracket(ui, seeds, palette);
            });
    });
}

fn draw_bracket(ui: &mut egui::Ui, seeds: &[String], palette: &[Color32]) {
    if seeds.len() < 2 {
        ui.label(
            egui::RichText::new("Not enough players to build a bracket.")
                .color(Color32::from_rgb(200, 120, 120)),
        );
        return;
    }

    let available = ui.available_size();
    let slot = Vec2::new(150.0, 32.0);
    let rounds = (seeds.len() as f32).log2().ceil() as usize;
    let margin_x = 24.0;
    let width = available.x.max(640.0);
    let height = available.y.max(420.0);
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, height), egui::Sense::hover());
    let painter = ui.painter_at(rect);

    let col_spacing = if rounds > 1 {
        (rect.width() - 2.0 * margin_x - slot.x) / (rounds as f32 - 1.0)
    } else {
        rect.width() - 2.0 * margin_x - slot.x
    };

    let matches_round0 = seeds.len() / 2;
    let base_gap = 16.0;
    let total_height =
        matches_round0 as f32 * slot.y + (matches_round0.saturating_sub(1) as f32) * base_gap;
    let start_y = rect.center().y - total_height / 2.0;

    let mut rounds_rects: Vec<Vec<egui::Rect>> = Vec::new();
    let mut round0 = Vec::new();
    for i in 0..matches_round0 {
        let y = start_y + i as f32 * (slot.y + base_gap);
        let r = egui::Rect::from_min_size(egui::Pos2::new(rect.left() + margin_x, y), slot);
        let c = palette.get(i % palette.len()).copied().unwrap_or(Color32::GRAY);
        painter.rect_filled(r, 6.0, c);
        painter.rect_stroke(r, 6.0, egui::Stroke::new(1.2, Color32::from_rgb(55, 65, 90)));
        let name = seeds.get(i * 2).map(|s| s.as_str()).unwrap_or("");
        let name2 = seeds.get(i * 2 + 1).map(|s| s.as_str()).unwrap_or("");
        painter.text(
            egui::Pos2::new(r.left() + 8.0, r.top() + 9.0),
            egui::Align2::LEFT_CENTER,
            name,
            egui::FontId::proportional(13.0),
            Color32::WHITE,
        );
        painter.text(
            egui::Pos2::new(r.left() + 8.0, r.bottom() - 9.0),
            egui::Align2::LEFT_CENTER,
            name2,
            egui::FontId::proportional(12.0),
            Color32::from_rgb(220, 220, 230),
        );
        round0.push(r);
    }
    rounds_rects.push(round0);

    for round_idx in 1..rounds {
        let prev = rounds_rects[round_idx - 1].clone();
        let matches = prev.len() / 2;
        let mut current = Vec::new();
        let x = rect.left() + margin_x + col_spacing * round_idx as f32;
        for m in 0..matches {
            let a = prev[m * 2];
            let b = prev[m * 2 + 1];
            let center_y = (a.center().y + b.center().y) / 2.0;
            let r = egui::Rect::from_center_size(egui::Pos2::new(x, center_y), slot);
            painter.rect_filled(r, 6.0, Color32::from_rgb(30, 32, 46));
            painter.rect_stroke(r, 6.0, egui::Stroke::new(1.2, Color32::from_rgb(95, 105, 130)));
            painter.text(
                r.center(),
                egui::Align2::CENTER_CENTER,
                if round_idx + 1 == rounds { "Final" } else { "Advancing" },
                egui::FontId::proportional(12.0),
                Color32::from_rgb(220, 225, 235),
            );
            connect(&painter, a, r);
            connect(&painter, b, r);
            current.push(r);
        }
        rounds_rects.push(current);
    }

    if let Some(last_round) = rounds_rects.last() {
        if let Some(final_rect) = last_round.first() {
            let champ_x = (final_rect.center().x + rect.right() - margin_x - slot.x * 0.5) / 2.0;
            let champ_rect = egui::Rect::from_center_size(
                egui::Pos2::new(champ_x, final_rect.center().y),
                Vec2::new(slot.x + 10.0, slot.y + 6.0),
            );
            painter.rect_filled(champ_rect, 8.0, Color32::from_rgb(250, 130, 80));
            painter.rect_stroke(
                champ_rect,
                8.0,
                egui::Stroke::new(1.6, Color32::from_rgb(255, 215, 180)),
            );
            painter.text(
                champ_rect.center(),
                egui::Align2::CENTER_CENTER,
                "Champion",
                egui::FontId::proportional(14.0),
                Color32::from_rgb(32, 18, 12),
            );
            connect(&painter, *final_rect, champ_rect);
        }
    }

    painter.rect_stroke(
        rect.expand(4.0),
        8.0,
        egui::Stroke::new(1.0, Color32::from_rgb(70, 70, 100)),
    );
}

fn connect(painter: &egui::Painter, from: egui::Rect, to: egui::Rect) {
    let start = egui::Pos2::new(from.right(), from.center().y);
    let end = egui::Pos2::new(to.left(), to.center().y);
    let mid_x = (start.x + end.x) / 2.0;
    let stroke = egui::Stroke::new(1.2, Color32::from_rgb(90, 100, 130));
    painter.line_segment([start, egui::Pos2::new(mid_x, start.y)], stroke);
    painter.line_segment([egui::Pos2::new(mid_x, start.y), egui::Pos2::new(mid_x, end.y)], stroke);
    painter.line_segment([egui::Pos2::new(mid_x, end.y), end], stroke);
}
