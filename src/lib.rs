mod calc;
mod clear;
mod config;
mod error;
mod schedule;
mod track;
mod watcher;

use crate::config::Configuration;

pub use crate::config::get_config;

pub use error::TimeTrackerError;

pub struct TimeTracker<'a> {
    config: &'a Configuration,
}

impl<'a> TimeTracker<'a> {
    pub fn new(config: &'a Configuration) -> Self {
        TimeTracker { config }
    }
}
