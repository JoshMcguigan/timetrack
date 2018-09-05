use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(PartialEq, Debug)]
pub struct RawLog {
    pub name: String,
    pub timestamp: u64,
}

pub fn raw_logs_from(raw_data: &str) -> Vec<RawLog> {
    let mut raw_logs = vec![];

    for line in raw_data.lines() {
        raw_logs.push(RawLog::from(line));
    }

    raw_logs
}

impl<'a> From<&'a str> for RawLog {
    fn from(raw_data: &'a str) -> Self {
        // TODO convert this to try_from
        let mut parts = raw_data.split('/');
        RawLog {
            name: parts.next().unwrap().to_string(),
            timestamp: parts.next().unwrap().parse::<u64>().unwrap(),
        }
    }
}

impl Display for RawLog {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.name, self.timestamp,)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_logs_from_string() {
        let raw_data = "testproj1/123\ntestproj2/456\n";

        let raw_logs = raw_logs_from(raw_data);

        assert_eq!(2, raw_logs.len());
    }

    #[test]
    fn raw_log_from_str() {
        let raw_data = "josh/123";
        assert_eq!(
            RawLog {
                name: String::from("josh"),
                timestamp: 123u64
            },
            RawLog::from(raw_data)
        );
    }

    #[test]
    fn raw_log_display() {
        let raw_log = RawLog {
            name: String::from("testproj1"),
            timestamp: 123,
        };

        assert_eq!("testproj1/123", format!("{}", raw_log));
    }
}
