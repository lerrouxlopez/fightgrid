use crate::models::Event;
use crate::services::EventService;
use crate::repositories::EventRepository;

pub struct EventsController<R: EventRepository> {
    service: EventService<R>,
    new_name: String,
    new_description: String,
    show_add_modal: bool,
    edit_id: Option<i64>,
    edit_name: String,
    edit_description: String,
    show_edit_modal: bool,
}

impl<R: EventRepository> EventsController<R> {
    pub fn new(service: EventService<R>) -> Self {
        Self {
            service,
            new_name: String::new(),
            new_description: String::new(),
            show_add_modal: false,
            edit_id: None,
            edit_name: String::new(),
            edit_description: String::new(),
            show_edit_modal: false,
        }
    }

    pub fn events(&self) -> Vec<Event> {
        self.service.list_events()
    }

    pub fn new_fields(&mut self) -> (&mut String, &mut String) {
        (&mut self.new_name, &mut self.new_description)
    }

    pub fn open_add_modal(&mut self) {
        self.show_add_modal = true;
    }

    pub fn close_add_modal(&mut self) {
        self.show_add_modal = false;
        self.new_name.clear();
        self.new_description.clear();
    }

    pub fn add_modal_open(&self) -> bool {
        self.show_add_modal
    }

    pub fn submit_new(&mut self) {
        let name = self.new_name.trim();
        let desc = self.new_description.trim();
        if name.is_empty() {
            return;
        }
        let created = self.service.create_event(name, desc);
        if created.is_some() {
            self.close_add_modal();
        }
    }

    pub fn save_edit(&mut self, id: i64) {
        let name = self.edit_name.trim();
        let desc = self.edit_description.trim();
        if name.is_empty() {
            return;
        }
        if self.service.update_event(id, name, desc) {
            self.close_edit_modal();
        }
    }

    pub fn delete_event(&mut self, id: i64) {
        let _ = self.service.delete_event(id);
    }

    pub fn open_edit_modal(&mut self, event: &Event) {
        if let Some(id) = event.id {
            self.edit_id = Some(id);
            self.edit_name = event.name.clone();
            self.edit_description = event.description.clone();
            self.show_edit_modal = true;
        }
    }

    pub fn edit_modal_open(&self) -> bool {
        self.show_edit_modal
    }

    pub fn close_edit_modal(&mut self) {
        self.edit_id = None;
        self.edit_name.clear();
        self.edit_description.clear();
        self.show_edit_modal = false;
    }

    pub fn edit_fields(&mut self) -> (Option<i64>, &mut String, &mut String) {
        (self.edit_id, &mut self.edit_name, &mut self.edit_description)
    }
}
