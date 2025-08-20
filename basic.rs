use holiday_rs::{get_holidays, is_business_day, business_days_between};
use chrono::NaiveDate;

#[test]
fn france_2025_includes_main_fixed_dates() {
    let list = get_holidays(2025, "FR");
    let names: Vec<_> = list.iter().map(|h| h.name.as_str()).collect();

    assert!(names.contains(&"Jour de l'An"));
    assert!(names.contains(&"Fête du Travail"));
    assert!(names.contains(&"Noël"));
    assert!(names.contains(&"Victoire 1945"));
    assert!(names.contains(&"Fête Nationale"));
}

#[test]
fn business_day_logic_weekend() {
    let sat = NaiveDate::from_ymd_opt(2025, 5, 3).unwrap(); // samedi
    let sun = NaiveDate::from_ymd_opt(2025, 5, 4).unwrap(); // dimanche

    assert!(!is_business_day(sat, "FR"));
    assert!(!is_business_day(sun, "FR"));
}

#[test]
fn business_days_between_span() {
    let start = NaiveDate::from_ymd_opt(2025, 5, 2).unwrap(); // vendredi
    let end = NaiveDate::from_ymd_opt(2025, 5, 6).unwrap();   // mardi

    // Vendredi (jour ouvrable), samedi (non), dimanche (non), lundi (ouvrable) → 2 jours
    assert_eq!(business_days_between(start, end, "FR"), 2);
}

#[test]
fn next_holiday_after_date() {
    let date = NaiveDate::from_ymd_opt(2025, 5, 2).unwrap();
    let next = holiday_rs::next_holiday(date, "FR").unwrap();

    // Le prochain jour férié après le 2 mai 2025 doit être "Victoire 1945" le 8 mai
    assert_eq!(next.name, "Victoire 1945");
    assert_eq!(next.date, NaiveDate::from_ymd_opt(2025, 5, 8).unwrap());
}
