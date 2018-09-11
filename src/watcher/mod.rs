use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use std::time::SystemTime;
use TimeTracker;
use std::sync::mpsc::Sender;
use notify::FsEventWatcher;
use notify;

pub fn get_watcher(track_path: &PathBuf, tx: Sender<DebouncedEvent>) -> Result<FsEventWatcher, notify::Error> {
    let mut watcher: RecommendedWatcher =
        Watcher::new(tx, Duration::from_secs(0))?;
    watcher.watch(track_path, RecursiveMode::Recursive)?;

    Ok(watcher)
}
