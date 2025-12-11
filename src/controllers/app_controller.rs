use crate::repositories::PlayerRepository;
use crate::services::{BracketService, PlayerService};
use crate::controllers::{
    EventsController, HomeController, LiveController, ResultsController, SettingsController,
    TeamsController,
};
use eframe::egui::Color32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavSection {
    Home,
    Events,
    Teams,
    Results,
    Live,
    Settings,
}

impl NavSection {
    pub fn label(&self) -> &'static str {
        match self {
            NavSection::Home => "Home",
            NavSection::Events => "Events",
            NavSection::Teams => "Teams",
            NavSection::Results => "Results",
            NavSection::Live => "Live",
            NavSection::Settings => "Settings",
        }
    }
}

pub struct AppController<R: PlayerRepository> {
    nav_items: Vec<NavSection>,
    active_nav: NavSection,
    palette: Vec<Color32>,
    player_service: PlayerService<R>,
    bracket_service: BracketService,
    home_controller: HomeController,
    events_controller: EventsController,
    teams_controller: TeamsController,
    results_controller: ResultsController,
    live_controller: LiveController,
    settings_controller: SettingsController,
}

impl<R: PlayerRepository> AppController<R> {
    pub fn new(player_service: PlayerService<R>) -> Self {
        Self {
            nav_items: vec![
                NavSection::Home,
                NavSection::Events,
                NavSection::Teams,
                NavSection::Results,
                NavSection::Live,
                NavSection::Settings,
            ],
            active_nav: NavSection::Home,
            palette: vec![
                Color32::from_rgb(232, 81, 122),
                Color32::from_rgb(236, 142, 92),
                Color32::from_rgb(239, 185, 77),
                Color32::from_rgb(115, 201, 148),
                Color32::from_rgb(94, 183, 224),
                Color32::from_rgb(142, 136, 242),
                Color32::from_rgb(215, 121, 231),
                Color32::from_rgb(255, 111, 162),
            ],
            player_service,
            bracket_service: BracketService::new(),
            home_controller: HomeController::new(),
            events_controller: EventsController::new(),
            teams_controller: TeamsController::new(),
            results_controller: ResultsController::new(),
            live_controller: LiveController::new(),
            settings_controller: SettingsController::new(),
        }
    }

    pub fn nav_items(&self) -> &[NavSection] {
        &self.nav_items
    }

    pub fn active_nav(&self) -> NavSection {
        self.active_nav
    }

    pub fn set_active_nav(&mut self, nav: NavSection) {
        self.active_nav = nav;
    }

    pub fn palette(&self) -> &[Color32] {
        &self.palette
    }

    pub fn home_seeds(&self) -> Vec<String> {
        self.home_controller
            .seeds(&self.player_service, &self.bracket_service)
    }

    pub fn events(&self) -> &[crate::controllers::EventInfo] {
        self.events_controller.events()
    }

    pub fn teams(&self) -> &[crate::controllers::TeamInfo] {
        self.teams_controller.teams()
    }

    pub fn results(&self) -> &[crate::controllers::ResultInfo] {
        self.results_controller.results()
    }

    pub fn live_blocks(&self) -> &[crate::controllers::LiveBlock] {
        self.live_controller.blocks()
    }

    pub fn live_callouts(&self) -> &'static str {
        self.live_controller.callouts
    }

    pub fn settings_info(&self) -> &crate::controllers::SettingsInfo {
        self.settings_controller.info()
    }
}
