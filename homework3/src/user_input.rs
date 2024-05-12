use std::error::Error;
use std::io;

pub fn get_string_from_user() -> Result<String, Box<dyn Error>> {
    println!();
    println!("Please enter text to transform");
    println!("PRESS CTRL-D to finish text input");
    match io::read_to_string(io::stdin()) {
        Err(e) => Err(From::from(
            "Failed to read user input, error: ".to_string() + &e.to_string(),
        )),
        Ok(input_text) => Ok(input_text),
    }
}

#[allow(clippy::needless_return)]
pub fn get_option_from_arguments(arguments: &[String]) -> Result<String, Box<dyn Error>> {
    // let program_name = &arguments[0];
    let number_of_arguments = arguments.len() - 1;

    if number_of_arguments == 0 {
        return Err(From::from(
            "Not enough arguments, program expects exactly one argument".to_string(),
        ));
    } else if number_of_arguments > 1 {
        return Err(From::from(
            "Too many arguments, program expects exactly one argument".to_string(),
        ));
    } else {
        let user_option = &arguments[1];
        match user_option.as_str() {
            "--lowercase" | "--uppercase" | "--no-spaces" | "--snake-case" | "--slugify"
            | "--csv" | "--help" => {
                return Ok(user_option.to_string());
            }
            _ => {
                return Err(From::from("Unrecognized argument".to_string()));
            }
        };
    }
}

pub fn print_usage(arguments: &[String]) {
    let program_name = &arguments[0];
    eprintln!("-----------------------------------------------------");
    eprintln!("USAGE: {} --argument", program_name);
    eprintln!("\t --lowercase, convert the entire text to lowercase");
    eprintln!("\t --uppercase, convert the entire text to uppercase");
    eprintln!("\t --no-spaces, remove all spaces from the text");
    eprintln!("\t --snake-case, remove all spaces and replace them by a '-'");
    eprintln!("\t --slugify, convert the text in to a slug");
    eprintln!("\t --csv, parse input text as csv");
    eprintln!("\t --help, show this help menu");
    eprintln!("-----------------------------------------------------");
}
