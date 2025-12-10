use crate::repositories::PlayerRepository;
use crate::services::{BracketService, PlayerService};
use eframe::egui::Color32;

pub struct AppController<R: PlayerRepository> {
    nav_items: Vec<&'static str>,
    palette: Vec<Color32>,
    player_service: PlayerService<R>,
    bracket_service: BracketService,
}

impl<R: PlayerRepository> AppController<R> {
    pub fn new(player_service: PlayerService<R>) -> Self {
        Self {
            nav_items: vec!["Home", "Leaderboard", "Players", "Reports", "Settings"],
            palette: vec![
                Color32::from_rgb(232, 81, 122),
                Color32::from_rgb(236, 142, 92),
                Color32::from_rgb(239, 185, 77),
                Color32::from_rgb(115, 201, 148),
                Color32::from_rgb(94, 183, 224),
                Color32::from_rgb(142, 136, 242),
                Color32::from_rgb(215, 121, 231),
                Color32::from_rgb(255, 111, 162),
            ],
            player_service,
            bracket_service: BracketService::new(),
        }
    }

    pub fn nav_items(&self) -> &[&'static str] {
        &self.nav_items
    }

    pub fn palette(&self) -> &[Color32] {
        &self.palette
    }

    pub fn seed_names(&self) -> Vec<String> {
        let players = self.player_service.seed_players();
        self.bracket_service.seed_names(&players)
    }
}
