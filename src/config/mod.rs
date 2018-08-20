use directories::ProjectDirs;
use std::path::PathBuf;

pub struct Configuration {
    pub track_paths: Vec<PathBuf>,
    pub raw_data_path: PathBuf
}

pub fn get_config() -> Configuration {
//    let project_dir = ProjectDirs::from(
//        "rust",
//        "cargo",
//        "timetrack"
//    ).expect("Failed to read project directories");
//
//    let raw_data_path = project_dir.data_local_dir();

    Configuration {
        // TODO how to handle two track paths where one is a subdirectory of another
        track_paths: vec![PathBuf::from("/Users/josh/Projects")],
        raw_data_path: PathBuf::from("/Users/josh/.timetrack_raw"),
    }
}

