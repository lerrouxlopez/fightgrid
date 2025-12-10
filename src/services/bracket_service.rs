use crate::models::Player;

pub struct BracketService;

impl BracketService {
    pub fn new() -> Self {
        Self
    }

    pub fn seed_names(&self, players: &[Player]) -> Vec<String> {
        players.iter().map(|p| p.name.clone()).collect()
    }
}
