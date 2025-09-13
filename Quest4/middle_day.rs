use chrono::prelude::*;

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    let days = if is_leap_year(year as i32) {366} else {365};
	if days % 2 == 0 {
		return None;
	}
	let middle = days / 2 + 1;
	let date = NaiveDate::from_yo_opt(year as i32, middle)?;
	Some(date.weekday())
}
