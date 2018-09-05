extern crate directories;
extern crate notify;
extern crate toml;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

mod calc;
mod clear;
mod config;
mod schedule;
mod track;

use config::Configuration;

pub use config::get_config;

pub struct TimeTracker<'a> {
    config: &'a Configuration,
}

impl<'a> TimeTracker<'a> {
    pub fn new(config: &'a Configuration) -> Self {
        TimeTracker { config }
    }
}
