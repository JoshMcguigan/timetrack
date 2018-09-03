extern crate notify;
extern crate directories;
extern crate toml;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;

mod config;
mod calc;
mod track;
mod clear;
mod schedule;

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
