extern crate clap;
use clap::{App, Arg};

extern crate timetrack;
use timetrack::{track::track, calc::calc};

fn main() {
    // todo add cli option to clear history
    let matches = App::new("TimeTrack")
        .arg(Arg::with_name("track")
            .short("t")
            .long("track")
            .help("Set to track, otherwise show data"))
        .get_matches();

    if matches.is_present("track") {
        track();
    } else {
        calc();
    }
}
