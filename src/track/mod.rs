use std::path::Path;
use ::{RAW_DATA_FILE, ROOT_PATH};
use std::fs::OpenOptions;
use std::time::SystemTime;
use std::io::Write;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use notify::{RecursiveMode, Watcher, RecommendedWatcher, DebouncedEvent};
use std::time::Duration;

pub fn track() {
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();

    watcher.watch(ROOT_PATH, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(event) => {
                if let Some(path) = get_path_from_event(&event) {
                    store_path(path);
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        };
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

fn get_project_name_from_path(path: &Path) -> String {
    path
        .strip_prefix(ROOT_PATH)
        .expect("Path doesn't contain root path")
        .components()
        .next()
        .expect("Path only contained root path")
        .as_os_str()
        .to_string_lossy()
        .to_string()
}

fn extract_project_name<T>(path: T) -> Option<String>
    where T: AsRef<Path>
{
    let path = path.as_ref();
    let raw_data_file_path = PathBuf::from(RAW_DATA_FILE);
    // TODO handle file system separators in platform independent way
    if path != raw_data_file_path {
        let project = get_project_name_from_path(path);
        return Some(project)
    }

    None
}

fn store_path<T>(path: T)
    where T: AsRef<Path>
{
    if let Some(data) = extract_project_name(path) {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(RAW_DATA_FILE).unwrap();
        let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

        writeln!(&mut file, "{}/{}", data, time);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_project_name_some() {
        let event_path = PathBuf::from(ROOT_PATH.to_string() + "/testProj/file1.rs");

        assert_eq!(Some("testProj".to_string()), extract_project_name(event_path));
    }

    #[test]
    fn extract_project_name_none() {
        let event_path = PathBuf::from(RAW_DATA_FILE);

        assert_eq!(None, extract_project_name(event_path));
    }
}
