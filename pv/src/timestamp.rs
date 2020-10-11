use chrono::{Duration, NaiveDate, NaiveDateTime};
use std::convert::TryInto;

// calculates iso timestamp for time s, day, year
pub fn from_s_d_y(seconds: u64, day: u64, year: u64) -> NaiveDateTime {
    let iyear: i32 = year.try_into().unwrap();
    let iday: i64 = day.try_into().unwrap();
    let iseconds: i64 = seconds.try_into().unwrap();

    let date: NaiveDateTime = NaiveDate::from_ymd(iyear, 1, 1)
        .and_hms(0, 0, 0)
        .checked_add_signed(Duration::days(iday))
        .unwrap()
        .checked_add_signed(Duration::seconds(iseconds))
        .unwrap();
    return date;
}
