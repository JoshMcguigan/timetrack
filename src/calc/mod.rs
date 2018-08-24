use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Read;
use TimeTracker;

mod raw_log;
use self::raw_log::{raw_logs_from};

mod span;
use self::span::{Span, get_spans_from};

mod display;
use self::display::display;

impl<'a> TimeTracker<'a> {

    pub fn calc(&self) {
        // TODO calc should clear raw data file and save only the spans to a processed file

        // rename raw data file to mark it as being processed

        // process data into spans

        // append spans to processed data file

        // write last timestamp for each project to the beginning of the raw data file (creating if necessary)

        // delete the being processed data file

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.config.raw_data_path).unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        display(&parse_raw_data(contents));
    }

}

pub fn parse_raw_data(raw_data: String) -> HashMap<String, u64> {
    calculate_project_total_time(get_spans_from(raw_logs_from(raw_data)))
}

fn calculate_project_total_time(spans: Vec<Span>) -> HashMap<String, u64> {
    let mut project_totals = HashMap::new();

    for span in spans {
        let span_duration = span.duration();
        let span_name = span.name;

        if project_totals.contains_key(&span_name) {
            let old_total = project_totals.remove(&span_name).unwrap();
            project_totals.insert(span_name, old_total + span_duration);
        } else {
            project_totals.insert(span_name, span_duration);
        };
    }

    project_totals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_project_total_time_single_project() {
        let mut spans = vec![];
        let proj_1_name = "proj1";
        spans.push(Span { name: String::from("proj1"), start:1, end: 5});
        spans.push(Span { name: String::from("proj1"), start: 11, end: 26});

        let project_totals = calculate_project_total_time(spans);

        assert!(project_totals.contains_key(proj_1_name));
        assert_eq!(19u64, *project_totals.get(proj_1_name).unwrap());
    }

    #[test]
    fn calculate_project_total_time_two_projects() {
        let mut spans = vec![];
        let proj_1_name = "proj1";
        let proj_2_name = "proj2";
        spans.push(Span { name: String::from("proj1"), start: 1, end: 5});
        spans.push(Span { name: String::from("proj2"), start: 7, end: 12});
        spans.push(Span { name: String::from("proj1"), start: 11, end: 26});

        let project_totals = calculate_project_total_time(spans);

        assert!(project_totals.contains_key(proj_1_name));
        assert_eq!(19u64, *project_totals.get(proj_1_name).unwrap());
        assert!(project_totals.contains_key(proj_2_name));
        assert_eq!(5u64, *project_totals.get(proj_2_name).unwrap());
    }
}
