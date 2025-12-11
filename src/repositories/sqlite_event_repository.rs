use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;

use crate::models::Event;
use crate::repositories::EventRepository;
use rusqlite::{params, Connection};

pub struct SqliteEventRepository {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteEventRepository {
    pub fn new<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                description TEXT NOT NULL
            );",
        )?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }
}

impl EventRepository for SqliteEventRepository {
    fn list_events(&self) -> anyhow::Result<Vec<Event>> {
        let conn = self.conn.lock().expect("event db mutex poisoned");
        let mut stmt = conn.prepare("SELECT id, name, description FROM events ORDER BY id ASC")?;
        let rows = stmt.query_map([], |row| {
            Ok(Event {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                description: row.get(2)?,
            })
        })?;
        let mut events = Vec::new();
        for row in rows {
            events.push(row?);
        }
        Ok(events)
    }

    fn create_event(&self, name: &str, description: &str) -> anyhow::Result<Event> {
        let conn = self.conn.lock().expect("event db mutex poisoned");
        conn.execute(
            "INSERT INTO events (name, description) VALUES (?1, ?2)",
            params![name, description],
        )?;
        let id = conn.last_insert_rowid();
        Ok(Event {
            id: Some(id),
            name: name.to_string(),
            description: description.to_string(),
        })
    }

    fn update_event(&self, id: i64, name: &str, description: &str) -> anyhow::Result<bool> {
        let conn = self.conn.lock().expect("event db mutex poisoned");
        let updated = conn.execute(
            "UPDATE events SET name = ?1, description = ?2 WHERE id = ?3",
            params![name, description, id],
        )?;
        Ok(updated > 0)
    }

    fn delete_event(&self, id: i64) -> anyhow::Result<bool> {
        let conn = self.conn.lock().expect("event db mutex poisoned");
        let deleted = conn.execute("DELETE FROM events WHERE id = ?1", params![id])?;
        Ok(deleted > 0)
    }
}
