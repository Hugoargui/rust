use slug::slugify;

pub fn to_lowercase(input_string: String) -> String {
    input_string.to_lowercase()
}
pub fn to_uppercase(input_string: String) -> String {
    input_string.to_uppercase()
}
pub fn to_no_spaces(input_string: String) -> String {
    input_string.trim().replace(' ', "-").to_lowercase()
}
pub fn to_snakecase(input_string: String) -> String {
    input_string.trim().replace(' ', "-").to_lowercase()
}
pub fn to_slugified(input_string: String) -> String {
    slugify(input_string)
}

pub fn are_arguments_valid(arguments: &Vec<String>) -> bool {
    let program_name = &arguments[0];
    let number_of_arguments = arguments.len() - 1;

    if number_of_arguments == 0 {
        eprintln!("Error while running program : {}", &program_name);
        eprintln!("Not enough arguments, program expects exactly one argument");
        print_usage(program_name);
        return false;
    } else if number_of_arguments > 1 {
        eprintln!("Error while running program : {}", &program_name);
        eprintln!("Too many arguments, program expects exactly one argument");
        print_usage(program_name);
        return false;
    } else {
        let user_option = &arguments[1];
        match user_option.as_str() {
            "--lowercase" | "--uppercase" | "--no-spaces" | "--snake-case" | "--slugify"
            | "--help" => {}
            _ => {
                eprintln!("Unrecognized argument");
                print_usage(program_name);
                return false;
            }
        };
    }

    true
}

fn print_usage(program_name: &String) {
    eprintln!("-----------------------------------------------------");
    eprintln!("USAGE: {} --argument", program_name);
    eprintln!("\t --lowercase, convert the entire text to lowercase");
    eprintln!("\t --uppercase, convert the entire text to uppercase");
    eprintln!("\t --no-spaces, remove all spaces from the text");
    eprintln!("\t --snake-case, remove all spaces and replace them by a '-'");
    eprintln!("\t --slugify, convert the text in to a slug");
    eprintln!("-----------------------------------------------------");
}
