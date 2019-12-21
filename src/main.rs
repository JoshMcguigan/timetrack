use clap::{crate_version, App};

use clap::Arg;
use clap::SubCommand;
use timetrack::get_config;
use timetrack::TimeTracker;

mod logger;
use crate::logger::logger_init;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("TimeTrack")
        .version(crate_version!())
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity (0-5, example: -vv for WARN)"),
        )
        .subcommand(
            SubCommand::with_name("track")
                .about("Starts the file system watcher for time tracking"),
        )
        .subcommand(
            SubCommand::with_name("clear")
                .about("Clear all TimeTrack history (Warning: this cannot be undone)"),
        )
        .subcommand(SubCommand::with_name("config").about("Display the TimeTrack configuration"))
        .subcommand(
            SubCommand::with_name("schedule")
                .about("Schedule TimeTrack to start tracking on login for the current user"),
        )
        .subcommand(
            SubCommand::with_name("unschedule")
                .about("Disable automatic tracking on login for the current user"),
        )
        .get_matches();

    logger_init(matches.occurrences_of("v"));

    let config = get_config();
    let time_tracker = TimeTracker::new(&config);

    if matches.subcommand_matches("clear").is_some() {
        // TODO don't unwrap inside the library calls, handle errors here and exit with appropriate error message and exit code
        time_tracker.clear();
    } else if matches.subcommand_matches("track").is_some() {
        time_tracker.track();
    } else if matches.subcommand_matches("config").is_some() {
        time_tracker.print_config();
    } else if matches.subcommand_matches("schedule").is_some() {
        time_tracker.schedule();
    } else if matches.subcommand_matches("unschedule").is_some() {
        time_tracker.unschedule();
    } else {
        time_tracker.calc()?;
    }
    Ok(())
}
