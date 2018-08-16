use std::path::Path;
use ::{RAW_DATA_FILE, ROOT_PATH};
use std::fs::OpenOptions;
use std::time::SystemTime;
use std::io::Write;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use notify::{raw_watcher, RawEvent, RecursiveMode, Watcher};

pub fn track() {
    let (tx, rx) = channel();

    // TODO convert to debounced watcher, or debounce in some other way
    let mut watcher = raw_watcher(tx).unwrap();

    watcher.watch(ROOT_PATH, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(RawEvent{path: Some(path), ..}) => {
                handle_event(path);
            },
            Ok(event) => println!("broken event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
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

fn handle_event<T>(path: T)
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
//    use super::*;

    #[test]
    fn it_works() {

    }
}
