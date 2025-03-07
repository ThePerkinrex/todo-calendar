use chrono::{DateTime, FixedOffset, NaiveTime, TimeZone};

#[derive(Debug)]
pub struct Deadline {
    name: String,
    date: DateTime<FixedOffset>,
}

pub const DATE_FINAL_TIME: NaiveTime = NaiveTime::from_hms_opt(23, 59, 00).unwrap();

impl Deadline {
    pub fn new<Tz: TimeZone>(name: String, date: &DateTime<Tz>) -> Self {
        Self {
            name,
            date: date.fixed_offset(),
        }
    }
}
