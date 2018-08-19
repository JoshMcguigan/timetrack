extern crate notify;

use config::Configuration;

// TODO don't hard code this
pub static ROOT_PATH : &'static str = "/Users/josh/Projects";
pub static RAW_DATA_FILE : &'static str = "/Users/josh/.timetrack_raw";

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
