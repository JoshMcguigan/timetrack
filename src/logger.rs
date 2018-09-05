use env_logger;
use log::LevelFilter;

pub fn logger_init(log_level: u64) {
    let log_level = match log_level {
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        5 | _ => LevelFilter::Trace,
    };

    env_logger::Builder::new()
        .filter_level(log_level)
        .default_format_timestamp(false)
        .default_format_module_path(false)
        .init();
}
