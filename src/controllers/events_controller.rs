pub struct EventInfo {
    pub title: &'static str,
    pub subtitle: &'static str,
    pub status: &'static str,
}

pub struct EventsController {
    events: Vec<EventInfo>,
}

impl EventsController {
    pub fn new() -> Self {
        Self {
            events: vec![
                EventInfo {
                    title: "Contenders Night",
                    subtitle: "Jan 18 • 8 bouts • Manila Arena",
                    status: "Matchups locked",
                },
                EventInfo {
                    title: "Rivals Cup",
                    subtitle: "Feb 02 • 16 fighters • Cebu Dome",
                    status: "Bracket building",
                },
                EventInfo {
                    title: "Open Qualifiers",
                    subtitle: "Feb 20 • 32 bracket • Davao",
                    status: "Registration open",
                },
            ],
        }
    }

    pub fn events(&self) -> &[EventInfo] {
        &self.events
    }
}
