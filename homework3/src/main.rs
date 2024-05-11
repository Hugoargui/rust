mod stringlib;

use std::env;
// use std::error::Error;
use std::io::stdin;

// fn to_snakecase(input_string: String) -> String {
//     return input_string.trim().replace(' ', "-").to_lowercase();
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    if !stringlib::are_arguments_valid(&args) {
        std::process::exit(1);
    }

    println!();

    let user_option = &args[1];

    println!("You chose option: {user_option}");
    println!();

    println!("Please enter line to transform");
    let mut input_string = String::new();
    stdin()
        .read_line(&mut input_string)
        .expect("Failed to read user text from stdin");

    println!();
    println!("Input string:");
    println!("{input_string}");

    let output_string = match user_option.as_str() {
        "--lowercase" => stringlib::to_lowercase(input_string),
        "--uppercase" => stringlib::to_uppercase(input_string),
        "--no-spaces" => stringlib::to_no_spaces(input_string),
        "--snake-case" => stringlib::to_snakecase(input_string),
        "--slugify" => stringlib::to_slugified(input_string),
        _ => {
            unreachable!("Unknown argumetns should have been handled before.");
        }
    };

    println!();
    println!("Resulting string:");
    println!("{output_string}");
}
