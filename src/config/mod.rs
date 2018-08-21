use directories::ProjectDirs;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::fs;
use TimeTracker;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use toml;

pub struct Configuration {
    pub track_paths: Vec<PathBuf>,
    pub raw_data_path: PathBuf
}

#[derive(Deserialize, Serialize)]
struct UserConfig {
    track_paths: Vec<PathBuf>
}

pub fn get_config() -> Configuration {
    let project_dir = ProjectDirs::from(
        "rust",
        "cargo",
        "timetrack"
    ).expect("Failed to read project directories");

    let raw_data_path = get_raw_data_file_path(&project_dir);
    let user_config = get_user_config(&project_dir);

    Configuration {
        // TODO how to handle two track paths where one is a subdirectory of another
        track_paths: user_config.track_paths,
        raw_data_path,
    }
}

impl<'a> TimeTracker<'a> {
    pub fn print_config(&self) {
        // TODO print relevant configuration details
        println!("Configuration");
    }
}

fn get_raw_data_file_path(project_dirs: &ProjectDirs) -> PathBuf {
    let raw_data_directory = project_dirs.data_local_dir();
    let raw_data_file_path = raw_data_directory.join(".timetrack_raw");

    fs::create_dir_all(&raw_data_directory)
        .expect("Failed to create raw data directory");
    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&raw_data_file_path)
        .expect("Failed to create raw data file");

    raw_data_file_path
}

fn get_user_config(project_dirs: &ProjectDirs) -> UserConfig {
    let config_dir = project_dirs.config_dir();
    let config_file_path = config_dir.join("timetrack_config");

    if !config_file_path.exists() {
        init_config_file(&config_dir, &config_file_path);
    }

    let mut f = OpenOptions::new()
        .read(true)
        .open(&config_file_path)
        .expect("Failed to open config file");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    toml::from_str(&contents).expect("Failed to parse config file as TOML")
}

fn init_config_file(config_dir: impl AsRef<Path>, config_file_path: impl AsRef<Path>) {
    fs::create_dir_all(&config_dir)
        .expect("Failed to create config directory");
    let mut f = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&config_file_path)
        .expect("Failed to create config file");

    // TODO set this default to users HOME directory
    let default_config = UserConfig {
        track_paths: vec![PathBuf::from("/Users/josh/Projects")]
    };

    write!(&mut f, "{}", toml::to_string(&default_config).expect("Failed to convert default user config to TOML"));
}
