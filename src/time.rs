use evtc_parse::{Log, StateChange};

/// A point in time.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Time {
    pub absolute: u64,
}

impl Time {
    /// Creates a new time point.
    pub fn new(absolute: u64) -> Self {
        Self { absolute }
    }

    /// Creates a new time point from a log start event.
    pub fn log_start(log: &Log) -> Self {
        let time = log
            .events
            .iter()
            .find(|event| event.get_statechange() == StateChange::LogStart)
            .map(|event| event.time)
            .expect("no log start event");
        Self::new(time)
    }

    /// Converts the absolute time into relative time.
    pub fn relative(&self, absolute: u64) -> i32 {
        if let Some(diff) = absolute.checked_sub(self.absolute) {
            diff as _
        } else {
            let diff = (self.absolute - absolute) as i32;
            -diff
        }
    }

    /// Converts the relative time into absolute time.
    pub fn absolute(&self, relative: i32) -> u64 {
        if let Ok(relative) = u64::try_from(relative) {
            self.absolute + relative
        } else {
            self.absolute - relative.unsigned_abs() as u64
        }
    }
}
