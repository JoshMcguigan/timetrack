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
    use self::raw_log::RawLog;

    #[test]
    fn raw_log_to_span_no_logs() {
        let spans = get_spans_from(vec![]);
        assert_eq!(0, spans.len())
    }

    #[test]
    fn raw_log_to_span_single_project() {
        let project_name = "test_proj";
        let raw_log_1 = RawLog { name: String::from(project_name), timestamp: 0 };
        let raw_log_2 = RawLog { name: String::from(project_name), timestamp: 5 };
        let raw_log_3 = RawLog { name: String::from(project_name), timestamp: 20 };
        let raw_logs = vec![raw_log_1, raw_log_2, raw_log_3];

        let mut spans = get_spans_from(raw_logs);

        assert_eq!(1, spans.len());

        let span = spans.pop().unwrap();

        assert_eq!(project_name, span.name);
        assert_eq!(20, span.duration())
    }

    #[test]
    fn raw_log_to_span_two_project() {
        let project_name = "test_proj";
        let project_2_name = "test_proj2";
        let raw_log_1 = RawLog { name: String::from(project_name), timestamp: 0 };
        let raw_log_2 = RawLog { name: String::from(project_name), timestamp: 5 };
        let raw_log_3 = RawLog { name: String::from(project_2_name), timestamp: 20 };
        let raw_log_4 = RawLog { name: String::from(project_2_name), timestamp: 26 };
        let raw_logs = vec![raw_log_1, raw_log_2, raw_log_3, raw_log_4];

        let mut spans = get_spans_from(raw_logs);

        assert_eq!(2, spans.len());

        let span_1 = spans.remove(0);
        assert_eq!(project_name, span_1.name);
        assert_eq!(5, span_1.duration());

        let span_2 = spans.remove(0);
        assert_eq!(project_2_name, span_2.name);
        assert_eq!(6, span_2.duration());
    }

    #[test]
    fn raw_log_to_span_large_timegap() {
        let project_name = "test_proj";
        let raw_log_1 = RawLog { name: String::from(project_name), timestamp: 0 };
        let raw_log_2 = RawLog { name: String::from(project_name), timestamp: 5 };
        let raw_log_3 = RawLog { name: String::from(project_name), timestamp: 555520 };
        let raw_log_4 = RawLog { name: String::from(project_name), timestamp: 555526 };
        let raw_logs = vec![raw_log_1, raw_log_2, raw_log_3, raw_log_4];

        let mut spans = get_spans_from(raw_logs);

        assert_eq!(2, spans.len());

        let span_1 = spans.remove(0);
        assert_eq!(project_name, span_1.name);
        assert_eq!(5, span_1.duration());

        let span_2 = spans.remove(0);
        assert_eq!(project_name, span_2.name);
        assert_eq!(6, span_2.duration());
    }

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
