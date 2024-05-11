mod user_input;
use user_input::are_arguments_valid;
use user_input::get_string_from_user;
use user_input::print_usage;
mod stringlib;

use std::env;
// use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    if !are_arguments_valid(&args) {
        print_usage(&args);
        std::process::exit(1);
    }

    println!();

    let user_option = &args[1];

    println!("You chose option: {user_option}");

    let input_string = get_string_from_user();

    let output_string = match user_option.as_str() {
        "--lowercase" => stringlib::to_lowercase(input_string),
        "--uppercase" => stringlib::to_uppercase(input_string),
        "--no-spaces" => stringlib::to_no_spaces(input_string),
        "--snake-case" => stringlib::to_snakecase(input_string),
        "--slugify" => stringlib::to_slugified(input_string),
        _ => {
            unreachable!("Unknown arguments should have been handled before.");
        }
    };

    println!();
    println!("Resulting string:");
    println!("{output_string}");
}
