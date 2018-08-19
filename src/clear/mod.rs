use std::fs::OpenOptions;
use TimeTracker;

impl<'a> TimeTracker<'a> {
    pub fn clear(&self) {
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.config.raw_data_path).unwrap();
    }
}
