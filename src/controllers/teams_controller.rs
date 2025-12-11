pub struct TeamInfo {
    pub name: &'static str,
    pub roster: &'static str,
    pub status: &'static str,
}

pub struct TeamsController {
    teams: Vec<TeamInfo>,
}

impl TeamsController {
    pub fn new() -> Self {
        Self {
            teams: vec![
                TeamInfo {
                    name: "Northern Strikers",
                    roster: "8 fighters",
                    status: "Weigh-ins complete",
                },
                TeamInfo {
                    name: "Coastal Guardians",
                    roster: "6 fighters",
                    status: "Locking roster",
                },
                TeamInfo {
                    name: "Iron Wolves",
                    roster: "10 fighters",
                    status: "Scrimmage ready",
                },
                TeamInfo {
                    name: "Crimson Lotus",
                    roster: "7 fighters",
                    status: "Awaiting medicals",
                },
            ],
        }
    }

    pub fn teams(&self) -> &[TeamInfo] {
        &self.teams
    }
}
