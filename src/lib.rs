extern crate notify;

use config::Configuration;

pub mod config;
pub mod calc;
pub mod track;
pub mod clear;

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
