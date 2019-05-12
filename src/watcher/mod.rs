use notify;
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::time::Duration;

pub fn get_watcher(
    track_path: &PathBuf,
    tx: Sender<DebouncedEvent>,
) -> Result<RecommendedWatcher, notify::Error> {
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(0))?;
    watcher.watch(track_path, RecursiveMode::Recursive)?;

    Ok(watcher)
}
