use crate::models::Event;
use crate::repositories::EventRepository;

pub struct EventService<R: EventRepository> {
    repository: R,
}

impl<R: EventRepository> EventService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn list_events(&self) -> Vec<Event> {
        self.repository
            .list_events()
            .unwrap_or_else(|e| {
                eprintln!("Failed to list events: {e}");
                Vec::new()
            })
    }

    pub fn create_event(&self, name: &str, description: &str) -> Option<Event> {
        match self.repository.create_event(name, description) {
            Ok(evt) => Some(evt),
            Err(e) => {
                eprintln!("Failed to create event: {e}");
                None
            }
        }
    }

    pub fn update_event(&self, id: i64, name: &str, description: &str) -> bool {
        match self.repository.update_event(id, name, description) {
            Ok(updated) => updated,
            Err(e) => {
                eprintln!("Failed to update event: {e}");
                false
            }
        }
    }

    pub fn delete_event(&self, id: i64) -> bool {
        match self.repository.delete_event(id) {
            Ok(deleted) => deleted,
            Err(e) => {
                eprintln!("Failed to delete event: {e}");
                false
            }
        }
    }
}
