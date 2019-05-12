extern crate directories;
extern crate notify;
extern crate toml;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate prettytable;

mod calc;
mod clear;
mod config;
mod schedule;
mod track;
mod watcher;

use crate::config::Configuration;

pub use crate::config::get_config;

pub struct TimeTracker<'a> {
    config: &'a Configuration,
}

impl<'a> TimeTracker<'a> {
    pub fn new(config: &'a Configuration) -> Self {
        TimeTracker { config }
    }
}
