use calc::raw_log::RawLog;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

const MAX_SECONDS_BETWEEN_RECORDS_IN_SPAN: u64 = 5 * 60;

pub struct Span {
    pub name: String,
    pub start: u64,
    pub end: u64,
}

impl Span {
    pub fn duration(&self) -> u64 {
        self.end - self.start
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}/{}/{}", self.name, self.start, self.end,)
    }
}

pub fn spans_from(processed_data: &str) -> Vec<Span> {
    let mut spans = vec![];

    for line in processed_data.lines() {
        spans.push(Span::from(line));
    }

    spans
}

impl<'a> From<&'a str> for Span {
    fn from(raw_data: &'a str) -> Self {
        let mut parts = raw_data.split('/');
        Span {
            name: parts.next().unwrap().to_string(),
            start: parts.next().unwrap().parse::<u64>().unwrap(),
            end: parts.next().unwrap().parse::<u64>().unwrap(),
        }
    }
}

pub fn get_spans_from(mut raw_logs: Vec<RawLog>) -> Vec<Span> {
    if raw_logs.is_empty() {
        return vec![];
    }

    let mut spans = vec![];

    let first_log = raw_logs.remove(0);

    let mut span = Span {
        name: first_log.name,
        start: first_log.timestamp,
        end: first_log.timestamp,
    };
    for log in raw_logs {
        let same_name = log.name == span.name;
        let small_time_gap =
            log.timestamp.saturating_sub(span.end) < MAX_SECONDS_BETWEEN_RECORDS_IN_SPAN;

        match (same_name, small_time_gap) {
            (true, true) => span.end = max(log.timestamp, span.end),
            (false, true) => {
                let mid_point_time = (max(log.timestamp, span.end) - min(log.timestamp, span.end))
                    / 2
                    + min(log.timestamp, span.end);
                span.end = mid_point_time;
                spans.push(span);
                span = Span {
                    name: log.name,
                    start: mid_point_time,
                    end: log.timestamp,
                };
            }
            (_, false) => {
                spans.push(span);
                span = Span {
                    name: log.name,
                    start: log.timestamp,
                    end: log.timestamp,
                };
            }
        };
    }
    spans.push(span);
    spans
}

pub fn get_last_timestamp_per_project(spans: &[Span]) -> HashMap<String, u64> {
    let mut map = HashMap::new();

    for span in spans {
        // TODO is there a more efficient way to do this?
        match map.remove(&span.name) {
            None => {
                map.insert(span.name.clone(), span.end);
            }
            Some(map_time) => {
                if map_time < span.end {
                    map.insert(span.name.clone(), span.end);
                } else {
                    map.insert(span.name.clone(), map_time);
                }
            }
        }
    }

    map
}

pub fn get_vec_raw_logs_from_map_last_timestamp_per_project(
    map: HashMap<String, u64>,
) -> Vec<RawLog> {
    let mut raw_logs: Vec<RawLog> = map
        .into_iter()
        .map(|(project_name, timestamp)| RawLog {
            name: project_name,
            timestamp,
        }).collect();

    raw_logs.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    raw_logs
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
        let raw_log_1 = RawLog {
            name: String::from(project_name),
            timestamp: 0,
        };
        let raw_log_2 = RawLog {
            name: String::from(project_name),
            timestamp: 5,
        };
        let raw_log_3 = RawLog {
            name: String::from(project_name),
            timestamp: 20,
        };
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
        let raw_log_1 = RawLog {
            name: String::from(project_name),
            timestamp: 0,
        };
        let raw_log_2 = RawLog {
            name: String::from(project_name),
            timestamp: 6,
        };
        let raw_log_3 = RawLog {
            name: String::from(project_2_name),
            timestamp: 18,
        };
        let raw_log_4 = RawLog {
            name: String::from(project_2_name),
            timestamp: 26,
        };
        let raw_logs = vec![raw_log_1, raw_log_2, raw_log_3, raw_log_4];

        let mut spans = get_spans_from(raw_logs);

        assert_eq!(2, spans.len());

        let span_1 = spans.remove(0);
        assert_eq!(project_name, span_1.name);
        assert_eq!(6 + 6, span_1.duration()); // time between project timestamps is split equally between projects

        let span_2 = spans.remove(0);
        assert_eq!(project_2_name, span_2.name);
        assert_eq!(8 + 6, span_2.duration());
    }

    #[test]
    fn raw_log_to_span_two_projects_interleaved() {
        let project_1_name = "test_proj";
        let project_2_name = "test_proj2";
        let raw_log_1 = RawLog {
            name: String::from(project_1_name),
            timestamp: 0,
        };
        let raw_log_2 = RawLog {
            name: String::from(project_1_name),
            timestamp: 5,
        };
        let raw_log_3 = RawLog {
            name: String::from(project_2_name),
            timestamp: 20,
        };
        let raw_log_4 = RawLog {
            name: String::from(project_2_name),
            timestamp: 24,
        };
        let raw_log_5 = RawLog {
            name: String::from(project_1_name),
            timestamp: 30,
        };
        let raw_log_6 = RawLog {
            name: String::from(project_1_name),
            timestamp: 36,
        };
        let raw_logs = vec![
            raw_log_1, raw_log_2, raw_log_3, raw_log_4, raw_log_5, raw_log_6,
        ];

        let mut spans = get_spans_from(raw_logs);

        assert_eq!(3, spans.len());

        let span_1 = spans.remove(0);
        assert_eq!(project_1_name, span_1.name);
        assert_eq!(12, span_1.duration());

        let span_2 = spans.remove(0);
        assert_eq!(project_2_name, span_2.name);
        assert_eq!(15, span_2.duration());

        let span_3 = spans.remove(0);
        assert_eq!(project_1_name, span_3.name);
        assert_eq!(9, span_3.duration());

        assert_eq!(
            36,
            span_1.duration() + span_2.duration() + span_3.duration()
        );
    }

    #[test]
    fn raw_log_to_span_large_timegap() {
        let project_name = "test_proj";
        let raw_log_1 = RawLog {
            name: String::from(project_name),
            timestamp: 0,
        };
        let raw_log_2 = RawLog {
            name: String::from(project_name),
            timestamp: 5,
        };
        let raw_log_3 = RawLog {
            name: String::from(project_name),
            timestamp: 555520,
        };
        let raw_log_4 = RawLog {
            name: String::from(project_name),
            timestamp: 555526,
        };
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
            end: 30,
        };
        let span1b = Span {
            name: String::from("testproj1"),
            start: 10030,
            end: 10060,
        };
        let span2a = Span {
            name: String::from("testproj2"),
            start: 530,
            end: 560,
        };

        spans.push(span1a);
        spans.push(span1b);
        spans.push(span2a);

        let last_timestamp_per_project = get_last_timestamp_per_project(&spans);

        assert_eq!(
            &10060u64,
            last_timestamp_per_project
                .get("testproj1")
                .expect("testproj1 not found")
        );
    }

    #[test]
    fn get_vec_raw_logs_from_map_last_timestamp_per_project_empty() {
        let last_timestamp_per_project = HashMap::new();

        assert_eq!(
            0,
            get_vec_raw_logs_from_map_last_timestamp_per_project(last_timestamp_per_project).len()
        );
    }

    #[test]
    fn get_vec_raw_logs_from_map_last_timestamp_per_project_several_projects() {
        let mut last_timestamp_per_project = HashMap::new();
        last_timestamp_per_project.insert(String::from("proj1"), 1);
        last_timestamp_per_project.insert(String::from("proj2"), 2);
        last_timestamp_per_project.insert(String::from("proj3"), 3);

        let last_timestamp_as_vec =
            get_vec_raw_logs_from_map_last_timestamp_per_project(last_timestamp_per_project);

        assert_eq!(3, last_timestamp_as_vec.len());
        assert_eq!(1, last_timestamp_as_vec.get(0).unwrap().timestamp);
    }
}
