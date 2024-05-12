mod stringlib;
mod user_input;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let user_option = match user_input::get_option_from_arguments(&args) {
        Ok(ref option) => {
            println!("You chose option: {option}");
            if option == "--help" {
                user_input::print_usage(&args);
                std::process::exit(1);
            }
            option.to_string()
        }
        Err(e) => {
            eprintln!("-----------------------------------------------------");
            eprintln!("{e}");
            user_input::print_usage(&args);
            std::process::exit(1);
        }
    };

    let input_string = user_input::get_string_from_user();

    match stringlib::run(input_string, user_option) {
        Err(e) => {
            eprintln!("Problem while transforming input");
            eprintln!("{}", e);
            std::process::exit(1);
        }
        Ok(output_string) => {
            println!();
            println!("Resulting string:");
            println!("{output_string}");
        }
    };
}
