use calc::raw_log::RawLog;
use std::collections::HashMap;

const MAX_SECONDS_BETWEEN_RECORDS_IN_SPAN: u64 = 5 * 60;

pub struct Span {
    pub name: String,
    pub start: u64,
    pub end: u64,
}

impl Span {
    pub fn duration(&self) -> u64 {
        self.end-self.start
    }
}

pub fn get_spans_from(mut raw_logs: Vec<RawLog>) -> Vec<Span> {
    if raw_logs.len() == 0 { return vec![] }

    let mut spans = vec![];

    let first_log = raw_logs.remove(0);

    let mut span = Span {name: String::from(first_log.name), start: first_log.timestamp, end: first_log.timestamp};
    for log in raw_logs {
        let same_name = log.name == span.name;
        let small_time_gap = log.timestamp - span.end < MAX_SECONDS_BETWEEN_RECORDS_IN_SPAN;
        let new_log_is_part_of_existing_span = same_name && small_time_gap;

        if new_log_is_part_of_existing_span  {
            span.end = log.timestamp;
        } else {
            spans.push(span);
            span = Span {name: String::from(log.name), start: log.timestamp, end: log.timestamp};
        }
    }
    spans.push(span);
    spans
}

pub fn get_last_timestamp_per_project(spans: &Vec<Span>) -> HashMap<String,u64> {
    let mut map = HashMap::new();

    for span in spans {
        // TODO is there a more efficient way to do this?
        match map.remove(&span.name) {
            None => { map.insert(span.name.clone(), span.end); },
            Some(map_time) => {
                if map_time < span.end {
                    map.insert(span.name.clone(), span.end);
                } else {
                    map.insert(span.name.clone(), map_time);
                }
            },
        }
    }

    map
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
    fn get_last_timestamp_per_project_no_spans() {
        let spans = vec![];
        let last_timestamp_per_project = get_last_timestamp_per_project(&spans);

        assert!(last_timestamp_per_project.is_empty());
    }

    #[test]
    fn get_last_timestamp_per_project_several_spans() {
        let mut spans = vec![];
        let span1a = Span {
            name: String::from("testproj1"),
            start: 0,
            end: 1,
        };
        let span1b = Span {
            name: String::from("testproj1"),
            start: 0,
            end: 2,
        };
        let span2a = Span {
            name: String::from("testproj2"),
            start: 0,
            end: 1,
        };

        spans.push(span1a);
        spans.push(span1b);
        spans.push(span2a);

        let last_timestamp_per_project = get_last_timestamp_per_project(&spans);

        assert_eq!(&2u64, last_timestamp_per_project.get("testproj1").expect("testproj1 not found"));
    }
}