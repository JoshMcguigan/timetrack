extern crate clap;
use clap::{App, Arg};

extern crate timetrack;
use timetrack::config::get_config;
use timetrack::TimeTracker;

fn main() {
    // todo add cli option to clear history
    let matches = App::new("TimeTrack")
        .arg(Arg::with_name("track")
            .short("t")
            .long("track")
            .help("Set to track, otherwise show data"))
        .arg(Arg::with_name("clear")
            .short("c")
            .long("clear")
            .help("Clear all data"))
        .get_matches();

    let config = get_config();
    let time_tracker = TimeTracker::new(&config);

    if matches.is_present("clear") {
        // TODO don't unwrap inside the library calls, handle errors here and exit with appropriate error message and exit code
        time_tracker.clear();
    } else if matches.is_present("track") {
        time_tracker.track();
    } else {
        time_tracker.calc();
    }
}
