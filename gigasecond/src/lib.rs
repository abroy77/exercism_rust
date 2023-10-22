use time::Duration;
use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let gigasecond = Duration::new(10i64.pow(9), 0);
    start + gigasecond
}
