use crate::TimeTracker;
use std::fs::OpenOptions;

impl<'a> TimeTracker<'a> {
    pub fn clear(&self) {
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.config.raw_data_path)
            .unwrap();
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.config.processed_data_path)
            .unwrap();
    }
}
