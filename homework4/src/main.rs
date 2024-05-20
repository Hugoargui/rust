mod stringlib;
mod user_input;

use std::{env, sync::mpsc, thread};

fn main() {
    // let args: Vec<String> = env::args().collect();

    let (tx, rx) = mpsc::channel();

    let input_thread = thread::spawn(move || loop {
        let (user_command, user_input) = match user_input::get_string_from_user() {
            Ok((command, input)) => {
                if command == "help" {
                    user_input::print_usage();
                    std::process::exit(0);
                }
                (command.to_string(), input.to_string())
            }
            Err(e) => {
                eprintln!("\n-----------------------------------------------------");
                eprintln!("{e}");
                user_input::print_usage();
                continue;
            }
        };

        let _ = tx.send((user_command, user_input));

        // Sleep thread so that this loop doesn't run again a print otput before we get results, which makes it hard to read
        // In a real application this wouldn't be needed, as would not printing all the time to screen
        thread::sleep(std::time::Duration::from_secs(1));
    });

    let parsing_thread = thread::spawn(move || loop {
        let (command, text) = rx.recv().unwrap();
        match stringlib::run(&text, &command) {
            Err(e) => {
                eprintln!("Problem while transforming input");
                eprintln!("{}", e);
                std::process::exit(1);
            }
            Ok(output_string) => {
                println!();
                println!();
                println!("Resulting string:");
                println!("{output_string}");
            }
        };
    });

    input_thread.join().expect("The sender thread has panicked");
    parsing_thread
        .join()
        .expect("The receiver thread has panicked");
    println!("All threads finished execution")
}
