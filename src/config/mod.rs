use crate::watcher;
use crate::TimeTracker;
use directories::BaseDirs;
use directories::ProjectDirs;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use toml;

pub struct Configuration {
    user_config_path: PathBuf, // this file should not be read outside this module
    pub track_paths: Vec<PathBuf>,
    pub raw_data_path: PathBuf,
    pub processed_data_path: PathBuf,
}

impl Display for Configuration {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
// Caution: The indent level below matters
"TimeTrack Configuration
    User configuration: {:?}
    Tracking paths: {:?}
    Raw data: {:?}
    Processed data: {:?}",
            self.user_config_path,
            self.track_paths,
            self.raw_data_path,
            self.processed_data_path
        )
    }
}

impl Configuration {
    /// Used for creating mock configuration files to test other modules
    pub fn new_mock_config(
        track_paths: Vec<PathBuf>,
        raw_data_path: PathBuf,
        processed_data_path: PathBuf,
    ) -> Self {
        Configuration {
            user_config_path: PathBuf::new(), // this is a private field so for mocking purposes doesn't matter
            track_paths,
            raw_data_path,
            processed_data_path,
        }
    }
}

#[derive(Deserialize, Serialize)]
struct UserConfig {
    track_paths: Vec<PathBuf>,
}

pub fn get_config() -> Configuration {
    let project_dir = ProjectDirs::from("rust", "cargo", "timetrack")
        .expect("Failed to read project directories");

    let raw_data_path = get_data_file_path(&project_dir, ".timetrack_raw");
    let processed_data_path = get_data_file_path(&project_dir, ".timetrack_processed");
    let user_config_path = project_dir.config_dir().join("timetrack_config");
    let user_config = read_user_config(&user_config_path);

    Configuration {
        user_config_path,
        // TODO how to handle two track paths where one is a subdirectory of another
        track_paths: user_config.track_paths,
        raw_data_path,
        processed_data_path,
    }
}

impl<'a> TimeTracker<'a> {
    pub fn print_config(&self) {
        println!("{}", self.config);
        println!("Starting self test..");
        let (tx, _rx) = channel();
        for track_path in &self.config.track_paths {
            match watcher::get_watcher(track_path, tx.clone()) {
                Ok(_) => {
                    println!(
                        "Successfully added watcher for path {}",
                        track_path.to_string_lossy()
                    );
                }
                Err(err) => {
                    println!(
                        "Error {} adding watcher for path {:?}",
                        err,
                        track_path.to_string_lossy()
                    );
                }
            };
        }
        println!("Completed self test.");
    }
}

fn get_data_file_path(project_dirs: &ProjectDirs, filename: &str) -> PathBuf {
    let data_directory = project_dirs.data_local_dir();
    let data_file_path = data_directory.join(filename);

    fs::create_dir_all(&data_directory).expect("Failed to create data directory");
    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&data_file_path)
        .expect("Failed to create data file");

    data_file_path
}

fn read_user_config(user_config_path: &PathBuf) -> UserConfig {
    if !user_config_path.exists() {
        init_config_file(&user_config_path);
    }

    let mut f = OpenOptions::new()
        .read(true)
        .open(&user_config_path)
        .expect("Failed to open config file");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    toml::from_str(&contents).expect("Failed to parse config file as TOML")
}

fn init_config_file(config_file_path: impl AsRef<Path>) {
    let config_dir = config_file_path.as_ref().parent().unwrap();

    fs::create_dir_all(&config_dir).expect("Failed to create config directory");
    let mut f = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&config_file_path)
        .expect("Failed to create config file");

    let home_dir = BaseDirs::new()
        .expect("Unable to find home directory")
        .home_dir()
        .to_owned();
    let default_config = UserConfig {
        track_paths: vec![home_dir],
    };

    write!(
        &mut f,
        "{}",
        toml::to_string(&default_config).expect("Failed to convert default user config to TOML")
    )
    .expect("Failed to initialize configuration file");
}
