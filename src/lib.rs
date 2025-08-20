//! holiday-rs: MVP library to get holidays and business-day utils.
//!
//! - Countries: FR (France) for v0.1.0
//! - Exports: JSON/CSV helpers
//! - Utils: `next_holiday`, `is_business_day`, `business_days_between`

mod data;
mod calc;
pub mod export;

use chrono::{NaiveDate, Datelike};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Holiday {
    pub name: String,
    pub date: NaiveDate,
    pub country: String,
}

/// Return the list of holidays for a given `year` and ISO-like country code (MVP: "FR").
pub fn get_holidays(year: i32, country: &str) -> Vec<Holiday> {
    match country.to_uppercase().as_str() {
        "FR" => data::fr::holidays_fr(year),
        _ => Vec::new(),
    }
}

/// Return the next holiday strictly after `date` for `country`.
pub fn next_holiday(date: NaiveDate, country: &str) -> Option<Holiday> {
    let mut list = get_holidays(date.year(), country);
    if date.month() == 12 && date.day() >= 20 {
        list.extend(get_holidays(date.year() + 1, country));
    }
    list.into_iter().filter(|h| h.date > date).min_by_key(|h| h.date)
}

/// Returns true if the date is a business day (Mon-Fri and not a holiday for `country`).
pub fn is_business_day(date: NaiveDate, country: &str) -> bool {
    let weekday = date.weekday().number_from_monday(); // 1..=7
    if weekday >= 6 {
        return false;
    }
    let holidays = get_holidays(date.year(), country);
    !holidays.iter().any(|h| h.date == date)
}

/// Count business days in [start, end). If start >= end -> 0.
pub fn business_days_between(start: NaiveDate, end: NaiveDate, country: &str) -> u32 {
    if start >= end { return 0; }
    let mut count = 0u32;
    let mut d = start;
    while d < end {
        if is_business_day(d, country) { count += 1; }
        d = d.succ_opt().unwrap();
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_fr_2025_contains_may1() {
        let v = get_holidays(2025, "FR");
        assert!(v.iter().any(|h| h.name.contains("Fête du Travail") && h.date == NaiveDate::from_ymd_opt(2025,5,1).unwrap()));
    }

    #[test]
    fn test_business_days_between_simple() {
        let s = NaiveDate::from_ymd_opt(2025, 5, 2).unwrap(); // Fri
        let e = NaiveDate::from_ymd_opt(2025, 5, 6).unwrap(); // Tue
        assert_eq!(business_days_between(s, e, "FR"), 2); // Fri + Mon
    }
}

