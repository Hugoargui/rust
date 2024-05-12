// use csv::ReaderBuilder;

use std::error::Error;

pub fn parse_csv(input_text: String) -> Result<String, Box<dyn Error>> {
    if true {
        let mut reader = csv::Reader::from_reader(input_text.as_bytes());
        for record in reader.records() {
            let record = record?;
            println!(
                "In {}, {} built the {} model. It is a {}.",
                &record[0], &record[1], &record[2], &record[3]
            );
        }
        Ok("return".to_string())
    } else {
        Err(From::from("Empty or broken string".to_string()))
    }
}
