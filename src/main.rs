use eframe::egui;
use egui::{Color32, FontId, Pos2, Rect, Stroke, Vec2};

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(1280.0, 780.0)),
        ..Default::default()
    };

    eframe::run_native(
        "FightGrid",
        native_options,
        Box::new(|cc| Box::new(FightGridApp::new(cc))),
    )
}

struct FightGridApp {
    left_seeds: Vec<&'static str>,
    right_seeds: Vec<&'static str>,
    palette: Vec<Color32>,
}

impl FightGridApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            left_seeds: vec![
                "Waking in the Snow",
                "Twin Hype Back",
                "Legend Has It",
                "Down",
                "Never Look Back",
                "Early",
                "Blockbuster Night, Pt. 2",
                "Just",
            ],
            right_seeds: vec![
                "Close Your Eyes",
                "Don't Get Captured",
                "See Logs",
                "All Due Respect",
                "Lie, Cheat, Steal",
                "Report to the Shareholders",
                "Crown",
                "Oh My Darling Don't Cry",
            ],
            palette: vec![
                Color32::from_rgb(212, 64, 91),
                Color32::from_rgb(211, 101, 71),
                Color32::from_rgb(204, 139, 64),
                Color32::from_rgb(111, 171, 83),
                Color32::from_rgb(60, 158, 161),
                Color32::from_rgb(66, 118, 186),
                Color32::from_rgb(101, 93, 194),
                Color32::from_rgb(161, 78, 186),
            ],
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
                    .inner_margin(egui::style::Margin::symmetric(18.0, 12.0))
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

        egui::SidePanel::left("sidebar")
            .resizable(false)
            .exact_width(210.0)
            .show(ctx, |ui| {
                ui.visuals_mut().widgets.inactive.bg_fill = Color32::from_rgb(30, 34, 40);
                ui.visuals_mut().widgets.hovered.bg_fill = Color32::from_rgb(45, 50, 60);
                ui.visuals_mut().widgets.active.bg_fill = Color32::from_rgb(65, 70, 82);
                ui.add_space(12.0);
                ui.heading(
                    egui::RichText::new("Navigation")
                        .color(Color32::from_rgb(190, 200, 215))
                        .size(18.0),
                );
                ui.separator();
                for item in ["Dashboard", "Brackets", "Fighters", "Events", "Settings"] {
                    ui.add_space(6.0);
                    ui.add_sized(
                        Vec2::new(f32::INFINITY, 32.0),
                        egui::SelectableLabel::new(item == "Brackets", item),
                    );
                }
                ui.add_space(12.0);
                ui.separator();
                ui.label(
                    egui::RichText::new("Pinned Event")
                        .color(Color32::from_rgb(160, 180, 210))
                        .size(15.0),
                );
                ui.label(egui::RichText::new("Eskrima Open 2025").color(Color32::from_rgb(
                    210, 110, 120,
                )));
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(Color32::from_rgb(12, 14, 18)))
            .show(ctx, |ui| {
                ui.add_space(10.0);
                draw_bracket(ui, self);
                ui.add_space(10.0);
                ui.horizontal_centered(|ui| {
                    for label in ["Add Match", "Shuffle", "Reset", "Advance"] {
                        let btn = egui::Button::new(label).fill(Color32::from_rgb(30, 80, 70));
                        ui.add(btn);
                        ui.add_space(4.0);
                    }
                });
            });
    }
}

fn draw_bracket(ui: &mut egui::Ui, app: &FightGridApp) {
    let available = ui.available_size();
    let width = available.x.clamp(960.0, 1280.0);
    let height = available.y.clamp(420.0, 620.0);
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, height), egui::Sense::hover());
    let painter = ui.painter_at(rect);

    painter.rect_filled(rect, 10.0, Color32::from_rgb(18, 20, 24));
    painter.rect_stroke(rect, 10.0, Stroke::new(1.0, Color32::from_rgb(36, 40, 48)));

    let slot = Vec2::new(160.0, 32.0);
    let gap = 76.0;
    let left_col1 = rect.left() + 32.0;
    let left_col2 = left_col1 + slot.x + gap;
    let left_col3 = left_col2 + slot.x + gap;

    let right_col1 = rect.right() - 32.0 - slot.x;
    let right_col2 = right_col1 - slot.x - gap;
    let right_col3 = right_col2 - slot.x - gap;

    let start_y = rect.top() + 32.0;
    let stroke = Stroke::new(1.4, Color32::from_rgb(92, 108, 128));
    let left_final =
        draw_side(&painter, &app.left_seeds, &app.palette, left_col1, left_col2, left_col3, start_y, slot, false, stroke);
    let right_final =
        draw_side(&painter, &app.right_seeds, &app.palette, right_col1, right_col2, right_col3, start_y, slot, true, stroke);

    draw_center_finals(&painter, left_final, right_final, stroke);
}

fn draw_side(
    painter: &egui::Painter,
    seeds: &[&str],
    palette: &[Color32],
    col1_x: f32,
    col2_x: f32,
    col3_x: f32,
    start_y: f32,
    slot: Vec2,
    going_left: bool,
    stroke: Stroke,
) -> Rect {
    let spacing = 16.0;
    let mut round1 = Vec::new();
    for (i, name) in seeds.iter().enumerate() {
        let y = start_y + i as f32 * (slot.y + spacing);
        let rect = Rect::from_min_size(Pos2::new(col1_x, y), slot);
        let color = palette[i % palette.len()];
        painter.rect_filled(rect, 6.0, color);
        painter.rect_stroke(rect, 6.0, stroke);
        painter.text(
            Pos2::new(rect.left() + 8.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            *name,
            FontId::proportional(13.0),
            Color32::WHITE,
        );
        round1.push(rect);
    }

    let mut round2 = Vec::new();
    for pair in 0..4 {
        let a = round1[pair * 2];
        let b = round1[pair * 2 + 1];
        let y = (a.center().y + b.center().y) / 2.0;
        let rect = Rect::from_min_size(Pos2::new(col2_x, y - slot.y / 2.0), slot);
        painter.rect_filled(rect, 6.0, Color32::from_rgb(28, 32, 38));
        painter.rect_stroke(rect, 6.0, stroke);
        painter.text(
            Pos2::new(rect.left() + 8.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            "Advancing",
            FontId::proportional(12.0),
            Color32::from_rgb(185, 205, 225),
        );
        connect(painter, a, rect, going_left, stroke);
        connect(painter, b, rect, going_left, stroke);
        round2.push(rect);
    }

    let mut round3 = Vec::new();
    for pair in 0..2 {
        let a = round2[pair * 2];
        let b = round2[pair * 2 + 1];
        let y = (a.center().y + b.center().y) / 2.0;
        let rect = Rect::from_min_size(Pos2::new(col3_x, y - slot.y / 2.0), slot);
        painter.rect_filled(rect, 6.0, Color32::from_rgb(30, 36, 44));
        painter.rect_stroke(rect, 6.0, stroke);
        painter.text(
            Pos2::new(rect.left() + 8.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            "Semi Finalist",
            FontId::proportional(12.0),
            Color32::from_rgb(200, 210, 230),
        );
        connect(painter, a, rect, going_left, stroke);
        connect(painter, b, rect, going_left, stroke);
        round3.push(rect);
    }

    let finals_rect = Rect::from_min_size(
        Pos2::new(col3_x + if going_left { -slot.x - 12.0 } else { slot.x + 12.0 }, round3[0].center().y - slot.y / 2.0),
        slot,
    );
    painter.rect_filled(finals_rect, 6.0, Color32::from_rgb(34, 40, 48));
    painter.rect_stroke(finals_rect, 6.0, stroke);
    painter.text(
        Pos2::new(finals_rect.left() + 8.0, finals_rect.center().y),
        egui::Align2::LEFT_CENTER,
        "Finalist",
        FontId::proportional(12.0),
        Color32::from_rgb(225, 230, 235),
    );
    connect(painter, round3[0], finals_rect, going_left, stroke);
    connect(painter, round3[1], finals_rect, going_left, stroke);

    finals_rect
}

fn connect(painter: &egui::Painter, from: Rect, to: Rect, going_left: bool, stroke: Stroke) {
    let start = if going_left {
        from.center_left()
    } else {
        from.center_right()
    };
    let end = if going_left { to.center_right() } else { to.center_left() };
    let mid_x = (start.x + end.x) / 2.0;
    painter.line_segment([start, Pos2::new(mid_x, start.y)], stroke);
    painter.line_segment([Pos2::new(mid_x, start.y), Pos2::new(mid_x, end.y)], stroke);
    painter.line_segment([Pos2::new(mid_x, end.y), end], stroke);
}

fn draw_center_finals(painter: &egui::Painter, left_final: Rect, right_final: Rect, stroke: Stroke) {
    let mid_y = (left_final.center().y + right_final.center().y) / 2.0;
    let final_slot = Vec2::new(130.0, 36.0);
    let champ_slot = Vec2::new(160.0, 40.0);

    let center_x = (left_final.center().x + right_final.center().x) / 2.0;
    let left_final_box = Rect::from_min_size(
        Pos2::new(center_x - final_slot.x - 24.0, mid_y - final_slot.y / 2.0),
        final_slot,
    );
    let right_final_box = Rect::from_min_size(
        Pos2::new(center_x + 24.0, mid_y - final_slot.y / 2.0),
        final_slot,
    );

    painter.rect_filled(left_final_box, 6.0, Color32::from_rgb(40, 46, 56));
    painter.rect_stroke(left_final_box, 6.0, stroke);
    painter.text(
        Pos2::new(left_final_box.left() + 8.0, left_final_box.center().y),
        egui::Align2::LEFT_CENTER,
        "West Final",
        FontId::proportional(12.0),
        Color32::WHITE,
    );
    painter.rect_filled(right_final_box, 6.0, Color32::from_rgb(40, 46, 56));
    painter.rect_stroke(right_final_box, 6.0, stroke);
    painter.text(
        Pos2::new(right_final_box.left() + 8.0, right_final_box.center().y),
        egui::Align2::LEFT_CENTER,
        "East Final",
        FontId::proportional(12.0),
        Color32::WHITE,
    );

    connect(painter, left_final, left_final_box, false, stroke);
    connect(painter, right_final, right_final_box, true, stroke);

    let champ_box = Rect::from_min_size(
        Pos2::new(center_x - champ_slot.x / 2.0, mid_y + 64.0 - champ_slot.y / 2.0),
        champ_slot,
    );
    painter.rect_filled(champ_box, 8.0, Color32::from_rgb(210, 120, 70));
    painter.rect_stroke(champ_box, 8.0, Stroke::new(2.0, Color32::from_rgb(255, 222, 200)));
    painter.text(
        champ_box.center(),
        egui::Align2::CENTER_CENTER,
        "Champion",
        FontId::proportional(15.0),
        Color32::from_rgb(30, 14, 10),
    );

    connect(painter, left_final_box, champ_box, false, stroke);
    connect(painter, right_final_box, champ_box, true, stroke);
}
