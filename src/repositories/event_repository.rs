use crate::models::Event;

pub trait EventRepository: Send + Sync {
    fn list_events(&self) -> anyhow::Result<Vec<Event>>;
    fn create_event(&self, name: &str, description: &str) -> anyhow::Result<Event>;
    fn update_event(&self, id: i64, name: &str, description: &str) -> anyhow::Result<bool>;
    fn delete_event(&self, id: i64) -> anyhow::Result<bool>;
}
