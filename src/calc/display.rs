use prettytable::Table;
use std::collections::HashMap;

static NO_DATA_WARNING: &'static str = "No time track data found";

pub fn display(data: HashMap<String, u64>) {
    let output_rows = format(data);

    if output_rows.is_empty() {
        println!("{}", NO_DATA_WARNING);
    } else {
        print_table(output_rows);
    }
}

fn format(data: HashMap<String, u64>) -> Vec<(String, String)> {
    let mut output_rows = vec![];

    for (project, time_in_seconds) in data {
        if time_in_seconds > 0 {
            output_rows.push((project, to_hms(time_in_seconds)));
        }
    }

    output_rows.sort_by(|(a, _), (b, _)| a.cmp(b)); // alphabetize the output by project name

    output_rows
}

fn print_table(output_rows: Vec<(String, String)>) {
    let mut table = Table::new();

    // header row is bold
    table.add_row(row![b -> "Project Name", b -> "Time"]);
    for (project, time) in output_rows {
        table.add_row(row![project, time]);
    }

    table.printstd();
}

/// Converts a duration in seconds to a human readable string
fn to_hms(seconds: u64) -> String {
    let hours = seconds / (60 * 60);
    let minutes = (seconds - (hours * 60 * 60)) / 60;
    let seconds = seconds - (hours * 60 * 60) - (minutes * 60);

    match (hours, minutes, seconds) {
        (0, 0, 0) => String::from("None"),
        (0, 0, 1) => String::from("1 second"),
        (0, 0, seconds) => format!("{} seconds", seconds),
        (0, 1, _) => String::from("1 minute"),
        (0, minutes, _) => format!("{} minutes", minutes),
        (1, 1, _) => String::from("1 hour 1 minute"),
        (1, minutes, _) => format!("1 hour {} minutes", minutes),
        (hours, 1, _) => format!("{} hours 1 minute", hours),
        (hours, minutes, _) => format!("{} hours {} minutes", hours, minutes),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_hms_string_zero() {
        assert_eq!("None", to_hms(0));
    }

    #[test]
    fn to_hms_second() {
        assert_eq!("1 second", to_hms(1));
    }

    #[test]
    fn to_hms_seconds() {
        assert_eq!("30 seconds", to_hms(30));
    }

    #[test]
    fn to_hms_minute() {
        assert_eq!("1 minute", to_hms(60));
    }

    #[test]
    fn to_hms_minutes() {
        assert_eq!("5 minutes", to_hms(330));
    }

    #[test]
    fn to_hms_hour_minute() {
        assert_eq!("1 hour 1 minute", to_hms((1 * 60 * 60) + (1 * 60) + 30));
    }

    #[test]
    fn to_hms_hour() {
        assert_eq!("1 hour 10 minutes", to_hms((1 * 60 * 60) + (10 * 60) + 30));
    }

    #[test]
    fn to_hms_hours_minute() {
        assert_eq!("5 hours 1 minute", to_hms((5 * 60 * 60) + (1 * 60) + 30));
    }

    #[test]
    fn to_hms_hours() {
        assert_eq!("5 hours 10 minutes", to_hms((5 * 60 * 60) + (10 * 60) + 30));
    }
}
