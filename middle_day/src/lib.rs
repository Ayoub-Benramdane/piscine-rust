use chrono::{NaiveDate, Datelike};

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    if (year % 100 != 0 && year % 4 == 0) || year % 400 == 0 {
        return None
    }
    Some(NaiveDate::from_yo_opt(year as i32, 183).unwrap().weekday())
}
