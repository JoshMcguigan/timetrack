#[macro_use]
extern crate clap;
use clap::{App, Arg};

extern crate timetrack;
use timetrack::get_config;
use timetrack::TimeTracker;
use clap::SubCommand;

fn main() {
    let matches = App::new("TimeTrack")
        .subcommand(SubCommand::with_name("track"))
        .subcommand(SubCommand::with_name("clear"))
        .get_matches();

    let config = get_config();
    let time_tracker = TimeTracker::new(&config);

    if matches.subcommand_matches("clear").is_some() {
        // TODO don't unwrap inside the library calls, handle errors here and exit with appropriate error message and exit code
        time_tracker.clear();
    } else if matches.subcommand_matches("track").is_some() {
        time_tracker.track();
    } else {
        time_tracker.calc();
    }
}
