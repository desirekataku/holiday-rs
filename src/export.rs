use crate::Holiday;
use serde_json;
use csv::WriterBuilder;

/// Serialize holidays to pretty JSON string.
pub fn to_pretty_json(list: &[Holiday]) -> String {
    serde_json::to_string_pretty(&list).unwrap()
}

/// Serialize holidays to CSV (headers: name,date,country).
pub fn to_csv_string(list: &[Holiday]) -> String {
    let mut wtr = WriterBuilder::new().has_headers(true).from_writer(vec![]);
    for h in list {
        let date = h.date.format("%Y-%m-%d").to_string();
        wtr.serialize((&h.name, date, &h.country)).unwrap();
    }
    let bytes = wtr.into_inner().unwrap();
    String::from_utf8(bytes).unwrap()
}
