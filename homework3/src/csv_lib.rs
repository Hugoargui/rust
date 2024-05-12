use comfy_table::{self, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};

use std::error::Error;

pub fn parse_csv(input_text: String) -> Result<String, Box<dyn Error>> {
    if !input_text.trim().is_empty() {
        let mut reader = csv::Reader::from_reader(input_text.as_bytes());

        let headers = comfy_table::Row::from(reader.headers()?.clone().iter());

        let mut table = comfy_table::Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS);
        table.set_header(headers);

        for record in reader.records() {
            let record = record?;
            let row = comfy_table::Row::from(record.iter());
            table.add_row(row);
        }
        Ok(table.to_string())
    } else {
        Err(From::from("Empty or broken string".to_string()))
    }
}
