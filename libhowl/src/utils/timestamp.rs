use chrono::{DateTime, NaiveDateTime, Utc};

pub trait TimestampExt {
    fn to_chrono(&self) -> DateTime<Utc>;
}

impl TimestampExt for u64 {
    fn to_chrono(&self) -> DateTime<Utc> {
        let naive = NaiveDateTime::from_timestamp(*self as i64, 0);
        // Create a normal DateTime from the NaiveDateTime
        return DateTime::from_utc(naive, Utc);
    }
}