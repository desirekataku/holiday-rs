use chrono::NaiveDate;
use crate::Holiday;
use crate::calc::easter_sunday_gregorian;

pub mod fr {
    use super::*;

    /// France national holidays (metropolitan, generic set)
    pub fn holidays_fr(year: i32) -> Vec<Holiday> {
        let mut v = Vec::new();
        // Fixed-date holidays
        let fixed = vec![
            ("Jour de l'An", (1, 1)),
            ("Fête du Travail", (5, 1)),
            ("Victoire 1945", (5, 8)),
            ("Fête Nationale", (7, 14)),
            ("Assomption", (8, 15)),
            ("La Toussaint", (11, 1)),
            ("Armistice 1918", (11, 11)),
            ("Noël", (12, 25)),
        ];
        for (name, (m, d)) in fixed {
            if let Some(date) = NaiveDate::from_ymd_opt(year, m, d) {
                v.push(Holiday { name: name.to_string(), date, country: "FR".into() });
            }
        }

        // Moveable feasts based on Easter
        if let Some(easter) = easter_sunday_gregorian(year) {
            if let Some(d) = easter.checked_add_days(chrono::Days::new(1)) {
                v.push(Holiday { name: "Lundi de Pâques".into(), date: d, country: "FR".into() });
            }
            if let Some(d) = easter.checked_add_days(chrono::Days::new(39)) {
                v.push(Holiday { name: "Ascension".into(), date: d, country: "FR".into() });
            }
            if let Some(d) = easter.checked_add_days(chrono::Days::new(50)) {
                v.push(Holiday { name: "Lundi de Pentecôte".into(), date: d, country: "FR".into() });
            }
        }

        v.sort_by_key(|h| h.date);
        v
    }
}
