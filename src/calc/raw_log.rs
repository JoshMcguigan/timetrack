use crate::TimeTrackerError;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(PartialEq, Debug)]
pub struct RawLog {
    pub name: String,
    pub timestamp: u64,
}

pub fn raw_logs_from(raw_data: &str) -> Result<Vec<RawLog>, TimeTrackerError> {
    let mut raw_logs = vec![];

    for line in raw_data.lines() {
        let raw_log = RawLog::try_from(line)?;
        raw_logs.push(raw_log);
    }

    Ok(raw_logs)
}

impl<'a> TryFrom<&'a str> for RawLog {
    type Error = TimeTrackerError;
    fn try_from(raw_data: &'a str) -> Result<Self, Self::Error> {
        let mut parts = raw_data.split('/');
        let name = match parts.next() {
            Some(v) => v.to_string(),
            None => return Err(TimeTrackerError::InvalidLineError(raw_data.to_string())),
        };
        let timestamp = match parts.next() {
            Some(v) => match v.parse::<u64>() {
                Ok(parsed) => parsed,
                Err(_) => return Err(TimeTrackerError::InvalidTimestampError(v.to_string())),
            },
            None => return Err(TimeTrackerError::InvalidLineError(raw_data.to_string())),
        };
        Ok(RawLog { name, timestamp })
    }
}

impl Display for RawLog {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

        assert_eq!(2, raw_logs.unwrap().len());
    }

    #[test]
    fn raw_log_from_str() {
        let raw_data = "josh/123";
        assert_eq!(
            RawLog {
                name: String::from("josh"),
                timestamp: 123u64
            },
            RawLog::try_from(raw_data).unwrap()
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
