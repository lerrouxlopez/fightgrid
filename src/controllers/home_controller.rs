use crate::repositories::PlayerRepository;
use crate::services::{BracketService, PlayerService};

pub struct HomeController;

impl HomeController {
    pub fn new() -> Self {
        Self
    }

    pub fn seeds<R: PlayerRepository>(
        &self,
        player_service: &PlayerService<R>,
        bracket_service: &BracketService,
    ) -> Vec<String> {
        let players = player_service.seed_players();
        bracket_service.seed_names(&players)
    }
}
