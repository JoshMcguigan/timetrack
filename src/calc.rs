use chrono::DateTime;
use chrono::Utc;
use chrono::TimeZone;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
struct RawLog {
    name: String,
    timestamp: i64,
}

struct Span {
    name: String,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

impl Span {
    fn duration(&self) -> i64 {
        (self.end-self.start).num_seconds()
    }
}

pub fn parse_raw_data(raw_data: String) -> HashMap<String, i64> {
    calculate_project_total_time(get_spans_from(raw_logs_from(raw_data)))
}

fn raw_logs_from(raw_data: String) -> Vec<RawLog> {
    let mut raw_logs = vec![];

    for line in raw_data.lines() {
        raw_logs.push(RawLog::from(line));
    }

    raw_logs
}

impl<'a> From<&'a str> for RawLog {
    fn from(raw_data: &'a str) -> Self {
        // TODO convert this to try_from
        let mut parts = raw_data.split("/");
        RawLog { name: parts.next().unwrap().to_string(), timestamp: parts.next().unwrap().parse::<i64>().unwrap() }
    }
}

fn get_spans_from(mut raw_logs: Vec<RawLog>) -> Vec<Span> {
    // TODO long gaps in raw logs should not be the same span (maybe 5 minute max to start?)

    if raw_logs.len() == 0 { return vec![] }

    let mut spans = vec![];

    let first_log = raw_logs.remove(0);

    let date_time = Utc.timestamp(first_log.timestamp, 0);
    let mut span = Span {name: String::from(first_log.name), start: date_time, end: date_time};
    for log in raw_logs {
        if log.name == span.name {
            span.end = Utc.timestamp(log.timestamp, 0);
        } else {
            spans.push(span);
            let date_time = Utc.timestamp(log.timestamp, 0);
            span = Span {name: String::from(log.name), start: date_time, end: date_time};
        }
    }
    spans.push(span);
    spans
}

fn calculate_project_total_time(spans: Vec<Span>) -> HashMap<String, i64> {
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
    fn calculate_project_total_time_single_project() {
        let mut spans = vec![];
        let proj_1_name = "proj1";
        spans.push(Span { name: String::from("proj1"), start: Utc.timestamp(1, 0), end: Utc.timestamp(5, 0)});
        spans.push(Span { name: String::from("proj1"), start: Utc.timestamp(11, 0), end: Utc.timestamp(26, 0)});

        let project_totals = calculate_project_total_time(spans);

        assert!(project_totals.contains_key(proj_1_name));
        assert_eq!(19i64, *project_totals.get(proj_1_name).unwrap());
    }

    #[test]
    fn calculate_project_total_time_two_projects() {
        let mut spans = vec![];
        let proj_1_name = "proj1";
        let proj_2_name = "proj2";
        spans.push(Span { name: String::from("proj1"), start: Utc.timestamp(1, 0), end: Utc.timestamp(5, 0)});
        spans.push(Span { name: String::from("proj2"), start: Utc.timestamp(7, 0), end: Utc.timestamp(12, 0)});
        spans.push(Span { name: String::from("proj1"), start: Utc.timestamp(11, 0), end: Utc.timestamp(26, 0)});

        let project_totals = calculate_project_total_time(spans);

        assert!(project_totals.contains_key(proj_1_name));
        assert_eq!(19i64, *project_totals.get(proj_1_name).unwrap());
        assert!(project_totals.contains_key(proj_2_name));
        assert_eq!(5i64, *project_totals.get(proj_2_name).unwrap());
    }

    #[test]
    fn raw_logs_from_string() {
        let raw_data = "testproj1/123\ntestproj2/456\n";

        let raw_logs = raw_logs_from(String::from(raw_data));

        assert_eq!(2, raw_logs.len());

    }

    #[test]
    fn raw_log_from_str() {
        let raw_data = "josh/123";
        assert_eq!(RawLog {name: String::from("josh"), timestamp: 123i64 }, RawLog::from(raw_data));
    }
}
