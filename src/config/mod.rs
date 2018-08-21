use directories::ProjectDirs;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::fs;

pub struct Configuration {
    pub track_paths: Vec<PathBuf>,
    pub raw_data_path: PathBuf
}

pub fn get_config() -> Configuration {
    let project_dir = ProjectDirs::from(
        "rust",
        "cargo",
        "timetrack"
    ).expect("Failed to read project directories");

    let raw_data_directory = project_dir.data_local_dir();
    let raw_data_file_path = raw_data_directory.join(".timetrack_raw");

    fs::create_dir_all(&raw_data_directory)
        .expect("Failed to create raw data directory");
    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&raw_data_file_path)
        .expect("Failed to create raw data file");

    Configuration {
        // TODO how to handle two track paths where one is a subdirectory of another
        track_paths: vec![PathBuf::from("/Users/josh/Projects")],
        raw_data_path: raw_data_file_path,
    }
}

