use comfy_table::{self, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};

use std::error::Error;
use std::fs::File;

pub fn parse_csv() -> Result<String, Box<dyn Error>> {
    let file_name = "src/data.csv".to_string();
    let file = File::open(file_name)?;

    let mut reader = csv::Reader::from_reader(file);

    let header = comfy_table::Row::from(reader.headers()?.clone().iter());

    let mut table = comfy_table::Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(header);

    for record in reader.records() {
        let record = match record {
            Err(e) => {
                return Err(From::from(
                    "Failed to parse csv".to_string() + &e.to_string(),
                ));
            }

            Ok(record) => record,
        };

        let row = comfy_table::Row::from(record.iter());
        table.add_row(row);
    }
    Ok(table.to_string())
}
