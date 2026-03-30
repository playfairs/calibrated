pub mod autoclicker;
pub mod config;
pub mod ui;

use autoclicker::{clicker::Clicker, scheduler::Scheduler};
use config::settings::Settings;
use ui::content::Content;

pub struct App {
    clicker: Clicker,
    scheduler: Scheduler,
    settings: Settings,
    content: Content,
}

impl App {
    pub fn new() -> Self {
        Self {
            clicker: Clicker::new(),
            scheduler: Scheduler::new(),
            settings: Settings::new(),
            content: Content::new(),
        }
    }

    pub fn clicker_mut(&mut self) -> &mut Clicker {
        &mut self.clicker
    }

    pub fn scheduler_mut(&mut self) -> &mut Scheduler {
        &mut self.scheduler
    }

    pub fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    pub fn content_mut(&mut self) -> &mut Content {
        &mut self.content
    }

    pub fn clicker(&self) -> &Clicker {
        &self.clicker
    }

    pub fn scheduler(&self) -> &Scheduler {
        &self.scheduler
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn content(&self) -> &Content {
        &self.content
    }
}
