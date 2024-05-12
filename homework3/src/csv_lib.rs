use std::error::Error;

pub fn parse_csv(input_text: String) -> Result<String, Box<dyn Error>> {
    if true {
        Ok(input_text.to_lowercase())
    } else {
        Err(From::from("Empty or broken string".to_string()))
    }
}
