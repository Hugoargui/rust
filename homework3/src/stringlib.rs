#[path = "csv_lib.rs"]
mod csv_lib;

use slug::slugify;

use std::error::Error;

pub fn run(input_string: &str, user_option: &str) -> Result<String, Box<dyn Error>> {
    match user_option {
        "--lowercase" => to_lowercase(input_string),
        "--uppercase" => to_uppercase(input_string),
        "--no-spaces" => to_no_spaces(input_string),
        "--snake-case" => to_snakecase(input_string),
        "--slugify" => to_slugified(input_string),
        "--csv" => csv_lib::parse_csv(input_string),
        _ => {
            unreachable!("Unknown arguments should have been handled before.");
        }
    }
}

fn is_valid_string(input_string: &str) -> bool {
    !input_string.trim().is_empty()
}

pub fn to_lowercase(input_string: &str) -> Result<String, Box<dyn Error>> {
    if is_valid_string(input_string) {
        Ok(input_string.to_lowercase())
    } else {
        Err(From::from("Input text is empty".to_string()))
    }
}

pub fn to_uppercase(input_string: &str) -> Result<String, Box<dyn Error>> {
    if is_valid_string(input_string) {
        Ok(input_string.to_uppercase())
    } else {
        Err(From::from("Input text is empty".to_string()))
    }
}

pub fn to_no_spaces(input_string: &str) -> Result<String, Box<dyn Error>> {
    if is_valid_string(input_string) {
        Ok(input_string.trim().replace(' ', "-").to_lowercase())
    } else {
        Err(From::from("Input text is empty".to_string()))
    }
}
pub fn to_snakecase(input_string: &str) -> Result<String, Box<dyn Error>> {
    if is_valid_string(input_string) {
        Ok(input_string.trim().replace(' ', "-").to_lowercase())
    } else {
        Err(From::from("Input text is empty".to_string()))
    }
}

pub fn to_slugified(input_string: &str) -> Result<String, Box<dyn Error>> {
    if is_valid_string(input_string) {
        Ok(slugify(input_string))
    } else {
        Err(From::from("Input text is empty".to_string()))
    }
}
