pub struct Configuration {
    pub track_paths: Vec<String>,
    pub raw_data_path: String
}

pub fn get_config() -> Configuration {
    Configuration {
        track_paths: vec!["/Users/josh/Projects".to_string()],
        raw_data_path: "/Users/josh/.timetrack_raw".to_string(),
    }
}

