use std::env;

use homework6::client;
use homework6::server;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_name = &args[0];
    let number_of_arguments = args.len() - 1;

    println!();

    if number_of_arguments == 0 {
        eprintln!("Error while running program : {}", &program_name);
        eprintln!("Not enough arguments, program expects at least one argument");
        eprintln!("Usage: cargo run [client/server] [optional ip (default localhost)] [optional port (default 8080)]");
        std::process::exit(1);
    }

    let default_host = &String::from("localhost");
    let default_port = &String::from("11111");
    let hostname = args.get(2).unwrap_or(default_host).to_string();
    let port = args.get(3).unwrap_or(default_port).to_string();

    let user_option = &args[1];
    match user_option.as_str() {
        "client" => {
            client::run(hostname, port);
        }
        "server" => {
            server::run(hostname, port);
        }
        _ => {
            eprintln!("Unrecognized argument, only valid arguments are client and server");
            eprintln!("Usage: cargo run [client/server] [optional ip (default localhost)] [optional port (default 8080)]");
            std::process::exit(1);
        }
    };
}
