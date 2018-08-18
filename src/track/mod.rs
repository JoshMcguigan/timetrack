use config::Configuration;
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::time::SystemTime;

pub struct Tracker<'a> {
    config: &'a Configuration
}

impl<'a> Tracker<'a> {
    pub fn new(config: &'a Configuration) -> Self {
        Tracker { config }
    }

    pub fn track(&self) {
        let (tx, rx) = channel();

        let mut watchers = vec![]; // need to keep ownership of watchers so they aren't dropped at end of for-loop

        for track_path in &self.config.track_paths {
            let mut watcher: RecommendedWatcher = Watcher::new(tx.clone(), Duration::from_secs(2)).unwrap();
            watcher.watch(track_path, RecursiveMode::Recursive).unwrap();

            watchers.push(watcher);
        }

        loop {
            match rx.recv() {
                Ok(event) => {
                    if let Some(path) = get_path_from_event(&event) {
                        self.store_path(path);
                    }
                },
                Err(e) => println!("watch error: {:?}", e),
            };
        }
    }

    fn get_project_name_from_path(&self, path: &Path) -> String {
        path
            .strip_prefix(PathBuf::from(self.config.track_paths.get(0).unwrap())) // TODO correct this for multiple path options
            .expect("Path doesn't contain root path")
            .components()
            .next()
            .expect("Path only contained root path")
            .as_os_str()
            .to_string_lossy()
            .to_string()
    }

    fn extract_project_name<T>(&self, path: T) -> Option<String>
        where T: AsRef<Path>
    {
        let path = path.as_ref();
        let raw_data_file_path = PathBuf::from(&self.config.raw_data_path);
        // TODO handle file system separators in platform independent way
        if path != raw_data_file_path {
            let project = self.get_project_name_from_path(path);
            return Some(project)
        }

        None
    }

    fn store_path<T>(&self, path: T)
        where T: AsRef<Path>
    {
        if let Some(data) = self.extract_project_name(path) {
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .append(true)
                .open(&self.config.raw_data_path).unwrap();
            let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

            writeln!(&mut file, "{}/{}", data, time);
        }
    }
}

fn get_path_from_event(event: &DebouncedEvent) -> Option<&Path> {
    match event {
        DebouncedEvent::Create(path) |
        DebouncedEvent::Write(path) |
        DebouncedEvent::Chmod(path) |
        DebouncedEvent::Remove(path) |
        DebouncedEvent::Rename(_, path) => { Some(path.as_ref()) }, // TODO use both paths from rename?
        DebouncedEvent::NoticeWrite(_) | // NoticeWrite and NoticeRemove both create duplicate entries for our use case
        DebouncedEvent::NoticeRemove(_) |
        DebouncedEvent::Rescan |
        DebouncedEvent::Error(_, _) => { None },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_project_name_some() {
        let config = get_mock_config();
        let event_path = PathBuf::from(config.track_paths.get(0).unwrap().clone() + "/testProj/file1.rs");

        let tracker = Tracker::new(&config);

        assert_eq!(Some("testProj".to_string()), tracker.extract_project_name(event_path));
    }

    #[test]
    fn extract_project_name_none() {
        let config = get_mock_config();
        let event_path = PathBuf::from(config.raw_data_path.clone());

        let tracker = Tracker::new(&config);

        assert_eq!(None, tracker.extract_project_name(event_path));
    }

    fn get_mock_config() -> Configuration {
        Configuration {
            track_paths: vec!["/Users/josh/Projects".to_string()],
            raw_data_path: "/Users/josh/.timetrack_raw".to_string(),
        }
    }
}
