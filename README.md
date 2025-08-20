# holiday-rs

`holiday-rs` est une bibliothèque Rust légère pour le calcul des jours fériés et des jours ouvrables en France 🇫🇷.

## Fonctionnalités

- 📅 Obtenez les jours fériés pour une année donnée
- 🧾 Exporte les résultats en JSON ou CSV
- 📈 Calcule les jours ouvrables entre deux dates

## Exemple d'utilisation

```rust
use holiday_rs::{get_holidays, export, next_holiday};
use chrono::NaiveDate;

fn main() {
    let year = 2025;
    let holidays = get_holidays(year, "FR");
    println!("Jours fériés en France pour l'année {}:", year);
    for holiday in &holidays {
        println!("- {}: {}", holiday.name, holiday.date);
    }

    println!("\nEn JSON:\n{}", export::to_pretty_json(&holidays));
    println!("\nEn CSV:\n{}", export::to_csv_string(&holidays));

    let today = NaiveDate::from_ymd_opt(2025, 5, 2).unwrap();
    if let Some(next) = next_holiday(today, "FR") {
        println!("Prochain jour férié après {}: {} le {}", today, next.name, next.date);
    }
}
