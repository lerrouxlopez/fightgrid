use eframe::egui;
use egui::{Color32, FontId, Margin, Pos2, Rect, Stroke, Vec2};

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 880.0])
            .with_fullscreen(true)
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
    active_nav: usize,
    players: Vec<&'static str>,
    palette: Vec<Color32>,
}

impl FightGridApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            nav_items: vec!["Home", "Leaderboard", "Players", "Reports", "Settings"],
            active_nav: 0,
            players: vec![
                "Aida Santos",
                "Ramon Cruz",
                "Leah Navarro",
                "Marco Dela Rosa",
                "Tina Valdez",
                "Gab Luna",
                "Rico Manalo",
                "Kara Abad",
                "Lito Vergara",
                "Nina Reyes",
                "Jun Sarmiento",
                "Mara Quinto",
                "Elena Flores",
                "Miguel Ibarra",
                "Paolo Castillo",
                "Sara Dominguez",
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
                for (idx, item) in self.nav_items.iter().enumerate() {
                    ui.add_space(6.0);
                    let selected = self.active_nav == idx;
                    let text = egui::RichText::new(*item).color(Color32::WHITE);
                    let mut frame = egui::Frame::none();
                    if selected {
                        frame = frame.fill(Color32::from_rgb(70, 80, 96));
                    }
                    frame.inner_margin(Margin::same(4.0)).show(ui, |ui| {
                        let button = egui::SelectableLabel::new(selected, text);
                        if ui.add_sized(Vec2::new(f32::INFINITY, 28.0), button).clicked() {
                            self.active_nav = idx;
                        }
                    });
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
                let mid = self.players.len() / 2;
                let left_seeds = &self.players[..mid];
                let right_seeds = &self.players[mid..];

                ui.add_space(6.0);
                match self.active_nav {
                    0 => {
                        draw_bracket(ui, left_seeds, right_seeds, &self.palette);
                        ui.add_space(10.0);
                        ui.horizontal_centered(|ui| {
                            for label in ["Seed", "Shuffle", "Reset", "Advance"] {
                                let btn =
                                    egui::Button::new(label).fill(Color32::from_rgb(48, 118, 96));
                                ui.add(btn);
                                ui.add_space(4.0);
                            }
                        });
                    }
                    _ => {
                        ui.vertical_centered(|ui| {
                            ui.add_space(60.0);
                            ui.label(
                                egui::RichText::new("Coming soon...")
                                    .size(24.0)
                                    .color(Color32::from_rgb(160, 175, 192)),
                            );
                        });
                    }
                }
            });
    }
}

fn draw_bracket(
    ui: &mut egui::Ui,
    left_seeds: &[&str],
    right_seeds: &[&str],
    palette: &[Color32],
) {
    let available = ui.available_size();
    let width = available.x.max(1024.0);
    let height = available.y.max(520.0);
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, height), egui::Sense::hover());
    let painter = ui.painter_at(rect);

    painter.rect_filled(rect, 10.0, Color32::from_rgb(18, 20, 24));
    painter.rect_stroke(rect, 10.0, Stroke::new(1.0, Color32::from_rgb(36, 40, 48)));

    let slot = Vec2::new(170.0, 36.0);
    let rounds_left = (left_seeds.len() as f32).log2().ceil() as usize;
    let rounds_right = (right_seeds.len() as f32).log2().ceil() as usize;
    let max_rounds = rounds_left.max(rounds_right).max(1);

    let half_space = rect.width() / 2.0 - 200.0;
    let col_spacing = (half_space / max_rounds as f32).clamp(120.0, 220.0);

    let left_cols: Vec<f32> = (0..rounds_left)
        .map(|i| rect.left() + 32.0 + i as f32 * col_spacing)
        .collect();
    let right_cols: Vec<f32> = (0..rounds_right)
        .map(|i| rect.right() - 32.0 - slot.x - i as f32 * col_spacing)
        .collect();

    let stroke = Stroke::new(1.4, Color32::from_rgb(110, 126, 150));
    let start_y = rect.center().y;

    let left_final = draw_side(
        &painter,
        left_seeds,
        palette,
        &left_cols,
        start_y,
        slot,
        false,
        stroke,
    );
    let right_final = draw_side(
        &painter,
        right_seeds,
        palette,
        &right_cols,
        start_y,
        slot,
        true,
        stroke,
    );

    draw_center_finals(&painter, left_final, right_final, stroke);
}

fn draw_side(
    painter: &egui::Painter,
    seeds: &[&str],
    palette: &[Color32],
    col_x: &[f32],
    center_y: f32,
    slot: Vec2,
    going_left: bool,
    stroke: Stroke,
) -> Rect {
    let spacing = 16.0;
    let total_height = seeds.len() as f32 * (slot.y + spacing) - spacing;
    let mut rounds: Vec<Vec<Rect>> = Vec::new();

    // Round 1
    let mut round = Vec::new();
    for (i, name) in seeds.iter().enumerate() {
        let y = center_y - total_height / 2.0 + i as f32 * (slot.y + spacing);
        let rect = Rect::from_min_size(Pos2::new(col_x[0], y), slot);
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
        round.push(rect);
    }
    rounds.push(round);

    // Subsequent rounds
    for r in 1..col_x.len() {
        let prev = rounds.last().unwrap();
        let mut next = Vec::new();
        for pair in 0..(prev.len() / 2) {
            let a = prev[pair * 2];
            let b = prev[pair * 2 + 1];
            let y = (a.center().y + b.center().y) / 2.0;
            let rect = Rect::from_min_size(Pos2::new(col_x[r], y - slot.y / 2.0), slot);
            painter.rect_filled(rect, 6.0, Color32::from_rgb(28 + (r as u8 * 4), 32, 42));
            painter.rect_stroke(rect, 6.0, stroke);
            painter.text(
                Pos2::new(rect.left() + 8.0, rect.center().y),
                egui::Align2::LEFT_CENTER,
                if r + 1 == col_x.len() { "Finalist" } else { "Advancing" },
                FontId::proportional(12.0),
                Color32::from_rgb(200, 210, 230),
            );
            connect(painter, a, rect, going_left, stroke);
            connect(painter, b, rect, going_left, stroke);
            next.push(rect);
        }
        rounds.push(next);
    }

    rounds.last().and_then(|v| v.first()).cloned().unwrap_or_else(|| {
        Rect::from_min_size(
            Pos2::new(col_x.last().copied().unwrap_or(0.0), center_y - slot.y / 2.0),
            slot,
        )
    })
}

fn connect(painter: &egui::Painter, from: Rect, to: Rect, going_left: bool, stroke: Stroke) {
    let start = if going_left {
        Pos2::new(from.left(), from.center().y)
    } else {
        Pos2::new(from.right(), from.center().y)
    };
    let end = if going_left {
        Pos2::new(to.right(), to.center().y)
    } else {
        Pos2::new(to.left(), to.center().y)
    };
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
