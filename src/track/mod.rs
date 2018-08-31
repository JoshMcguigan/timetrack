use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::time::SystemTime;
use TimeTracker;

impl<'a> TimeTracker<'a> {
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
                        trace!("File change detected on {:?}", path);
                        self.store_path(path);
                    }
                },
                Err(e) => println!("watch error: {:?}", e),
            };
        }
    }

    fn get_project_name_from_path(&self, path: &Path) -> String {
        for track_path in &self.config.track_paths {
            if let Ok(path) = path.strip_prefix(track_path) {
                return path
                    .components()
                    .next()
                    .expect("Path only contained root path")
                    .as_os_str()
                    .to_string_lossy()
                    .to_string()
            };
        }

        panic!("Failed processing path which was not in configured track paths");
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
        if let Some(project_name) = self.extract_project_name(path) {
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .append(true)
                .open(&self.config.raw_data_path).unwrap();
            let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

            debug!("File change stored for {}", project_name);
            writeln!(&mut file, "{}/{}", project_name, time);
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
    use config::Configuration;


    #[test]
    fn extract_project_name_some() {
        let config = get_mock_config();
        let event_path = PathBuf::from(config.track_paths.get(0).unwrap().clone().join("testProj/file1.rs"));

        let tracker = TimeTracker::new(&config);

        assert_eq!(Some("testProj".to_string()), tracker.extract_project_name(event_path));
    }

    #[test]
    fn extract_project_name_multiple_paths() {
        let config = get_mock_config();
        let event_path = PathBuf::from(config.track_paths.get(1).unwrap().clone().join("testOtherProj/file1.rs"));

        let tracker = TimeTracker::new(&config);

        assert_eq!(Some("testOtherProj".to_string()), tracker.extract_project_name(event_path));
    }

    #[test]
    fn extract_project_name_none() {
        let config = get_mock_config();
        let event_path = PathBuf::from(config.raw_data_path.clone());

        let tracker = TimeTracker::new(&config);

        assert_eq!(None, tracker.extract_project_name(event_path));
    }

    fn get_mock_config() -> Configuration {
        Configuration {
            track_paths: vec![
                PathBuf::from("/Users/josh/Projects"),
                PathBuf::from("/Users/josh/OtherProjects"),
            ],
            raw_data_path: PathBuf::from("/Users/josh/.timetrack_raw"),
            processed_data_path: PathBuf::from("/Users/josh/.timetrack_processed"),
        }
    }
}
