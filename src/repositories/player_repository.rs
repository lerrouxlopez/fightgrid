use crate::models::Player;

pub trait PlayerRepository: Send + Sync {
    fn list_players(&self) -> Vec<Player>;
}

pub struct InMemoryPlayerRepository {
    players: Vec<Player>,
}

impl InMemoryPlayerRepository {
    pub fn new(players: Vec<Player>) -> Self {
        Self { players }
    }
}

impl PlayerRepository for InMemoryPlayerRepository {
    fn list_players(&self) -> Vec<Player> {
        self.players.clone()
    }
}
