use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum TimeTrackerError {
    InvalidLineError(String),
    InvalidTimestampError(String),
}

impl Error for TimeTrackerError {
    fn description(&self) -> &str {
        match *self {
            TimeTrackerError::InvalidLineError(..) => "could not parse line",
            TimeTrackerError::InvalidTimestampError(..) => "could not parse line",
        }
    }
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            TimeTrackerError::InvalidLineError(..) => None,
            TimeTrackerError::InvalidTimestampError(..) => None,
        }
    }
}

impl fmt::Display for TimeTrackerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: ", self.description())?;
        match *self {
            TimeTrackerError::InvalidLineError(ref v) => write!(f, "could not parse line: {}", v),
            TimeTrackerError::InvalidTimestampError(ref v) => {
                write!(f, "could not parse line: {}", v)
            }
        }
    }
}
