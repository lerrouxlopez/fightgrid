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
        let nav_items = self.controller.nav_items().to_vec();
        let seeds = self.controller.home_seeds();
        let palette = self.controller.palette().to_vec();
        let active_nav = self.controller.active_nav();

        let teams = self.controller.teams().to_vec();
        let results = self.controller.results().to_vec();
        let live_blocks = self.controller.live_blocks().to_vec();
        let live_callouts = self.controller.live_callouts().to_string();
        let settings = *self.controller.settings_info();

        if let Some(selected) = views::home::render(
            ctx,
            &nav_items,
            active_nav,
            &palette,
            &seeds,
            self.controller.events_controller_mut(),
            &teams,
            &results,
            &live_blocks,
            &live_callouts,
            &settings,
        ) {
            self.controller.set_active_nav(selected);
        }
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
        "Rodrigo Duterte",
    ]
    .into_iter()
    .map(|name| crate::models::Player {
        name: name.to_string(),
    })
    .collect()
}
