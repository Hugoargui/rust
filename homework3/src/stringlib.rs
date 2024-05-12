#[path = "csv_lib.rs"]
mod csv_lib;

use slug::slugify;

use std::error::Error;

pub fn run(input_string: String, user_option: String) -> Result<String, Box<dyn Error>> {
    let _csv = "Language,Paradigm,Year,Creator,Rust_Inspiration
                C,Imperative,1972,Dennis Ritchie,Medium
                Java,Object-Oriented,1995,James Gosling,Low
                Python,Multi-Paradigm,1991,Guido van Rossum,Medium
                Rust,Multi-Paradigm,2010,Graydon Hoare,High
                Haskell,Functional,1990,Lennart Augustsson,Medium"
        .to_string();
    let result = match user_option.as_str() {
        "--lowercase" => to_lowercase(input_string),
        "--uppercase" => to_uppercase(input_string),
        "--no-spaces" => to_no_spaces(input_string),
        "--snake-case" => to_snakecase(input_string),
        "--slugify" => to_slugified(input_string),
        "--csv" => csv_lib::parse_csv(_csv),
        _ => {
            unreachable!("Unknown arguments should have been handled before.");
        }
    };

    match result {
        Ok(output) => Ok(output),
        Err(e) => Err(e),
    }
}

fn is_valid_string(input_string: &str) -> bool {
    // !input_string.trim().is_empty() && input_string.trim().chars().all(char::is_alphabetic)
    !input_string.trim().is_empty()
}

pub fn to_lowercase(input_string: String) -> Result<String, Box<dyn Error>> {
    if is_valid_string(&input_string) {
        Ok(input_string.to_lowercase())
    } else {
        Err(From::from("Empty or broken string".to_string()))
    }
}

pub fn to_uppercase(input_string: String) -> Result<String, Box<dyn Error>> {
    if is_valid_string(&input_string) {
        Ok(input_string.to_uppercase())
    } else {
        Err(From::from("Empty or broken string".to_string()))
    }
}

pub fn to_no_spaces(input_string: String) -> Result<String, Box<dyn Error>> {
    if is_valid_string(&input_string) {
        Ok(input_string.trim().replace(' ', "-").to_lowercase())
    } else {
        Err(From::from("Empty or broken string".to_string()))
    }
}
pub fn to_snakecase(input_string: String) -> Result<String, Box<dyn Error>> {
    if is_valid_string(&input_string) {
        Ok(input_string.trim().replace(' ', "-").to_lowercase())
    } else {
        Err(From::from("Empty or broken string".to_string()))
    }
}

pub fn to_slugified(input_string: String) -> Result<String, Box<dyn Error>> {
    if is_valid_string(&input_string) {
        Ok(slugify(input_string))
    } else {
        Err(From::from("Empty or broken string".to_string()))
    }
}
