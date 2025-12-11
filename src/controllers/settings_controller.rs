#[derive(Clone, Copy, Debug)]
pub struct SettingsInfo {
    pub organization: &'static str,
    pub automation: &'static [&'static str],
    pub notifications: &'static str,
    pub safety: &'static [&'static str],
}

pub struct SettingsController {
    info: SettingsInfo,
}

impl SettingsController {
    pub fn new() -> Self {
        Self {
            info: SettingsInfo {
                organization: "FightGrid Promotions • Manila, PH",
                automation: &["Auto-save brackets: Enabled"],
                notifications: "Notifications: Matchmaker + Team leads",
                safety: &[
                    "Medical suspensions synced",
                    "Cutman + doctor confirmed",
                ],
            },
        }
    }

    pub fn info(&self) -> &SettingsInfo {
        &self.info
    }
}
