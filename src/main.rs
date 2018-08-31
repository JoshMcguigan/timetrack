extern crate clap;
use clap::App;

extern crate timetrack;
use timetrack::get_config;
use timetrack::TimeTracker;
use clap::SubCommand;
use clap::Arg;
use log::LevelFilter;

extern crate log;
extern crate env_logger;

fn main() {
    let matches = App::new("TimeTrack")
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity (0-5, example: -vv for WARN)"))
        .subcommand(SubCommand::with_name("track"))
        .subcommand(SubCommand::with_name("clear"))
        .subcommand(SubCommand::with_name("config"))
        .get_matches();

    let log_level = match matches.occurrences_of("v") {
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        5 | _ => LevelFilter::Trace,
    };

    // TODO modify the formatting to only show local time (not date and time in UTC)
    env_logger::Builder::new()
        .filter_level(log_level)
        .init();

    let config = get_config();
    let time_tracker = TimeTracker::new(&config);

    if matches.subcommand_matches("clear").is_some() {
        // TODO don't unwrap inside the library calls, handle errors here and exit with appropriate error message and exit code
        time_tracker.clear();
    } else if matches.subcommand_matches("track").is_some() {
        time_tracker.track();
    } else if matches.subcommand_matches("config").is_some() {
        time_tracker.print_config();
    } else {
        time_tracker.calc();
    }
}
