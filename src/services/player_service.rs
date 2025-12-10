use crate::models::Player;
use crate::repositories::PlayerRepository;

pub struct PlayerService<R: PlayerRepository> {
    repository: R,
}

impl<R: PlayerRepository> PlayerService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn seed_players(&self) -> Vec<Player> {
        self.repository.list_players()
    }
}
