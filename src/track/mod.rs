use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::time::SystemTime;
use TimeTracker;
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::time::Instant;
use std::collections::HashSet;

impl<'a> TimeTracker<'a> {
    pub fn track(&self) {
        let (tx, rx) = channel();

        let mut watchers = vec![]; // need to keep ownership of watchers so they aren't dropped at end of for-loop

        for track_path in &self.config.track_paths {
            let mut watcher: RecommendedWatcher = Watcher::new(tx.clone(), Duration::from_secs(0)).unwrap();
            watcher.watch(track_path, RecursiveMode::Recursive).unwrap();

            watchers.push(watcher);
        }

        let mut first_record_time;
        let write_delay = Duration::from_secs(2);
        let mut events = vec![];

        loop {
            // block waiting for the first event
            match rx.recv() {
                Ok(event) => {
                    first_record_time = Instant::now();
                    events.push(event);

                    // wait a small amount of additional time to see if other events come in
                    loop {
                        match rx.try_recv() {
                            Ok(event) => events.push(event),
                            Err(TryRecvError::Empty) => thread::sleep(Duration::from_millis(100)),
                            Err(e) => println!("watch error: {:?}", e),
                        };

                        if first_record_time.elapsed() >= write_delay {
                            break;
                        }
                    }
                },
                Err(e) => println!("watch error: {:?}", e),
            }

            first_record_time = Instant::now();
            let mut projects = HashSet::new();
            for event in events.drain(..) {
                if let Some(path) = get_path_from_event(&event) {
                    if let Some(project) = self.extract_project_name(path) {
                        trace!("File change detected on {:?}", path);
                        // dedup project list by inserting into hashmap
                        projects.insert(project);
                    }
                }
            }

            for project in projects {
                self.store_project(project);
            }

        }
    }

    fn extract_project_name<T>(&self, path: T) -> Option<String>
        where T: AsRef<Path>
    {
        let path = path.as_ref();
        let raw_data_file_path = PathBuf::from(&self.config.raw_data_path);
        // TODO handle file system separators in platform independent way
        if path != raw_data_file_path {
            for track_path in &self.config.track_paths {
                if let Ok(path) = path.strip_prefix(track_path) {
                    return Some(path
                        .components()
                        .next()
                        .expect("Path only contained root path")
                        .as_os_str()
                        .to_string_lossy()
                        .to_string())
                };
            }

            panic!("Failed processing path which was not in configured track paths");
        }

        None
    }

    fn store_project(&self, project_name: String) {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(&self.config.raw_data_path).unwrap();
        let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

        let log = format!("{}/{}", project_name, time);
        debug!("Log stored: {}", log);
        writeln!(&mut file, "{}", log);
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
