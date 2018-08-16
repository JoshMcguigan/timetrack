use std::fs::OpenOptions;
use RAW_DATA_FILE;

pub fn clear() {
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(RAW_DATA_FILE).unwrap();
}
