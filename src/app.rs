use crate::controllers::AppController;
use crate::repositories::InMemoryPlayerRepository;
use crate::services::PlayerService;
use crate::views;
use eframe::egui;

pub struct FightGridApp {
    controller: AppController<InMemoryPlayerRepository>,
}

impl FightGridApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let repository = InMemoryPlayerRepository::new(default_players());
        let player_service = PlayerService::new(repository);
        let controller = AppController::new(player_service);

        Self { controller }
    }
}

impl eframe::App for FightGridApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let nav_items = self.controller.nav_items();
        let seeds = self.controller.seed_names();
        let palette = self.controller.palette();

        views::dashboard::render(ctx, nav_items, &seeds, palette);
    }
}

fn default_players() -> Vec<crate::models::Player> {
    vec![
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
    ]
    .into_iter()
    .map(|name| crate::models::Player {
        name: name.to_string(),
    })
    .collect()
}
