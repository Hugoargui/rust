use slug::slugify;

use std::env;
use std::io::stdin;

fn print_usage_and_terminate(program_name: &String) {
    println!("-----------------------------------------------------");
    println!("USAGE: {} --argument", program_name);
    println!("\t --lowercase, convert the entire text to lowercase");
    println!("\t --uppercase, convert the entire text to uppercase");
    println!("\t --no-spaces, remove all spaces from the text");
    println!("\t --snake-case, remove all spaces and replace them by a '-'");
    println!("\t --slugify, convert the text in to a slug");
    println!("-----------------------------------------------------");
    std::process::exit(1);
}

fn to_snakecase(input_string: String) -> String {
    return input_string.trim().replace(' ', "-").to_lowercase();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_name = &args[0];
    let number_of_arguments = args.len() - 1;

    println!();

    if number_of_arguments == 0 {
        println!("Error while running program : {}", &program_name);
        println!("Not enough arguments, program expects exactly one argument");
        print_usage_and_terminate(program_name);
    } else if number_of_arguments > 1 {
        println!("Error while running program : {}", &program_name);
        println!("Too many arguments, program expects exactly one argument");
        print_usage_and_terminate(program_name);
    }

    let user_option = &args[1];
    match user_option.as_str() {
        "--lowercase" | "--uppercase" | "--no-spaces" | "--snake-case" | "--slugify" | "--help" => {
        }
        _ => {
            println!("Unrecognized argument");
            print_usage_and_terminate(program_name);
        }
    };

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

    let mut output_string = String::new();
    match user_option.as_str() {
        "--lowercase" => output_string = input_string.to_lowercase(),
        "--uppercase" => output_string = input_string.to_uppercase(),
        "--no-spaces" => output_string = input_string.replace(' ', ""),
        "--snake-case" => output_string = to_snakecase(input_string),
        "--slugify" => output_string = slugify(input_string),
        "--help" => print_usage_and_terminate(program_name),
        _ => {
            unreachable!("Unknown argumetns should have been handled before.");
        }
    };

    println!();
    println!("Resulting string:");
    println!("{output_string}");
}
