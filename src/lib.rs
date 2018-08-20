extern crate notify;
extern crate directories;

mod config;
mod calc;
mod track;
mod clear;
mod display;

use config::Configuration;

pub use config::get_config;

pub struct TimeTracker<'a> {
    config: &'a Configuration
}

impl<'a> TimeTracker<'a> {
    pub fn new(config: &'a Configuration) -> Self {
        TimeTracker {
            config
        }
    }
}
