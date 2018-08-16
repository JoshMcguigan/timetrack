extern crate clap;
extern crate notify;

use clap::{App, Arg};
use notify::{raw_watcher, RawEvent, RecursiveMode, Watcher};
use std::fs::OpenOptions;
use std::io::Read;
use std::sync::mpsc::channel;

mod track;
mod calc;

static ROOT_PATH : &'static str = "/Users/josh/Projects";
static RAW_DATA_FILE : &'static str = "/Users/josh/.timetrack_raw";

fn main() {
    // todo add cli option to clear history
    let matches = App::new("TimeTrack")
        .arg(Arg::with_name("track")
            .short("t")
            .long("track")
            .help("Set to track, otherwise show data"))
        .get_matches();

    if matches.is_present("track") {
        let (tx, rx) = channel();

        // TODO convert to debounced watcher, or debounce in some other way
        let mut watcher = raw_watcher(tx).unwrap();

        watcher.watch(ROOT_PATH, RecursiveMode::Recursive).unwrap();

        loop {
            match rx.recv() {
                Ok(RawEvent{path: Some(path), ..}) => {
                    track::handle_event(path);
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
