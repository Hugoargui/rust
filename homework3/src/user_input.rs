use std::io::stdin;

pub fn get_string_from_user() -> String {
    println!();
    println!("Please enter line to transform");
    let mut input_string = String::new();
    stdin()
        .read_line(&mut input_string)
        .expect("Failed to read user text from stdin");

    println!();
    println!("Input string:");
    println!("{input_string}");

    input_string
}

pub fn are_arguments_valid(arguments: &Vec<String>) -> bool {
    let program_name = &arguments[0];
    let number_of_arguments = arguments.len() - 1;

    if number_of_arguments == 0 {
        eprintln!("Error while running program : {}", &program_name);
        eprintln!("Not enough arguments, program expects exactly one argument");
        return false;
    } else if number_of_arguments > 1 {
        eprintln!("Error while running program : {}", &program_name);
        eprintln!("Too many arguments, program expects exactly one argument");
        return false;
    } else {
        let user_option = &arguments[1];
        match user_option.as_str() {
            "--lowercase" | "--uppercase" | "--no-spaces" | "--snake-case" | "--slugify"
            | "--help" => {
                return true;
            }
            _ => {
                eprintln!("Unrecognized argument");
                return false;
            }
        };
    }
}

pub fn print_usage(arguments: &Vec<String>) {
    let program_name = &arguments[0];
    eprintln!("-----------------------------------------------------");
    eprintln!("USAGE: {} --argument", program_name);
    eprintln!("\t --lowercase, convert the entire text to lowercase");
    eprintln!("\t --uppercase, convert the entire text to uppercase");
    eprintln!("\t --no-spaces, remove all spaces from the text");
    eprintln!("\t --snake-case, remove all spaces and replace them by a '-'");
    eprintln!("\t --slugify, convert the text in to a slug");
    eprintln!("-----------------------------------------------------");
}
