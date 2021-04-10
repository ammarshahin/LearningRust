use chrono::{DateTime, Duration, Utc};

const GIGASECOND: i32 = 1_000_000_000;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + Duration::seconds(i64::from(GIGASECOND))
}
