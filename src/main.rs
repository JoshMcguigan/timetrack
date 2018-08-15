extern crate notify;
extern crate clap;
use clap::{Arg, App};

use std::fs::OpenOptions;
use std::io::Write;
use notify::{Watcher, RecursiveMode, RawEvent, raw_watcher};
use std::sync::mpsc::channel;
use std::borrow::Cow;
use std::time::SystemTime;
use std::io::Read;

mod calc;

static ROOT_PATH : &'static str = "/Users/josh/Projects";
static RAW_DATA_FILE : &'static str = "/Users/josh/.timetrack_raw";

fn handle_event(path: Cow<str>){
    if !path.contains("/.") {
        let project = path
            .trim_left_matches(ROOT_PATH)
            .trim_left_matches("/")
            .split("/").next().unwrap();
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(RAW_DATA_FILE).unwrap();
        let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        write!(&mut file, "{}/{}\n", project, time);
    }
}

fn main() {
    let matches = App::new("TimeTrack")
        .arg(Arg::with_name("track")
            .short("t")
            .long("track")
            .help("Set to track, otherwise show data"))
        .get_matches();

    if matches.is_present("track") {

        // Create a channel to receive the events.
        let (tx, rx) = channel();

        // Create a watcher object, delivering raw events.
        // The notification back-end is selected based on the platform.
        let mut watcher = raw_watcher(tx).unwrap();

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher.watch(ROOT_PATH, RecursiveMode::Recursive).unwrap();

        loop {
            match rx.recv() {
                Ok(RawEvent{path: Some(path), ..}) => {
                    handle_event(path.to_string_lossy());
                },
                Ok(event) => println!("broken event: {:?}", event),
                Err(e) => println!("watch error: {:?}", e),
            }
        }

    } else {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(RAW_DATA_FILE).unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        println!("{:?}", calc::parse_raw_data(contents));
    }

}
