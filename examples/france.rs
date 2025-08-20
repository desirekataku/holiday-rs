use holiday_rs::{get_holidays, export, next_holiday};
use chrono::NaiveDate;

fn main() {
    let year = 2025;
    let list = get_holidays(year, "FR");

    println!("Jours fériés en France pour l'année {} ({} jours):", year, list.len());
    for h in &list {
        println!("- {}: {}", h.name, h.date);
    }

    // Affichage en JSON
    println!("\nEn JSON:\n{}", export::to_pretty_json(&list));

    // Affichage en CSV
    println!("\nEn CSV:\n{}", export::to_csv_string(&list));

    // Prochain jour férié après une date donnée
    let today = NaiveDate::from_ymd_opt(2025, 5, 2).unwrap();
    if let Some(next) = next_holiday(today, "FR") {
        println!("Prochain jour férié après {} : {} le {}", today, next.name, next.date);
    } else {
        println!("Pas de prochain jour férié trouvé après {}", today);
    }
}
