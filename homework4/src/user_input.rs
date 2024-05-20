use std::error::Error;
use std::io;

pub fn get_string_from_user() -> Result<(String, String), Box<dyn Error>> {
    println!();
    print_usage();
    println!("Please enter command");
    println!("PRESS CTRL-D to finish text input");
    match io::read_to_string(io::stdin()) {
        Err(e) => Err(From::from(format!("Failed to read from stdin, error: {e}"))),
        Ok(input_text) => match parse_user_input(input_text) {
            Err(e) => Err(From::from(format!(
                "Failed to parse user input, error : {e}"
            ))),
            Ok((user_command, user_text)) => Ok((user_command, user_text)),
        },
    }
}

pub fn parse_user_input(user_input: String) -> Result<(String, String), Box<dyn Error>> {
    let parts: Vec<&str> = user_input.trim().splitn(2, ' ').collect();
    let user_command = parts[0];
    let user_text = parts[1];
    match user_command {
        "lowercase" | "uppercase" | "no-spaces" | "snake-case" | "slugify" | "csv" | "help" => {
            Ok((user_command.to_string(), user_text.to_string()))
        }
        _ => Err(From::from(format!("Unrecognized argument: {user_command}"))),
    }
}

pub fn print_usage() {
    eprintln!("-----------------------------------------------------");
    eprintln!("USAGE: <command> <input text> ");
    eprintln!("Valid commands: ");
    eprintln!("\t lowercase, convert the entire text to lowercase");
    eprintln!("\t uppercase, convert the entire text to uppercase");
    eprintln!("\t no-spaces, remove all spaces from the text");
    eprintln!("\t snake-case, remove all spaces and replace them by a '-'");
    eprintln!("\t slugify, convert the text in to a slug");
    eprintln!("\t csv, parse input text as csv");
    eprintln!("\t help, show this help menu");
    eprintln!("-----------------------------------------------------");
}
