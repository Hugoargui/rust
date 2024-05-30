#![allow(dead_code)]
#![allow(unused_imports)]

use std::env;
use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::process;
use std::sync::{Arc, Mutex};

use homework5::*;

fn sending_thread(mut stream: TcpStream) {
    loop {
        println!("Enter text to send (or .file <path>, .image<path>, .quit)");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");
        let input = input.trim().to_string();

        if input == "q" || input == "quit" {
            println!("Quiting client");
            break;
        };
        let new_message = MessageType::Text(input);
        println!("Sending {new_message:?}");
        send_message(&mut stream, &new_message);

        // match calculate_message_length(&stream) {
        //     Err(e) => {
        //         eprintln!("Server lost connection with server with error: {}", e);
        //         break;
        //     }
        //     Ok(len) => {
        //         let message =
        //             read_message(stream.try_clone().expect("failed to clone stream"), len);
        //         println!("Received: {message:?}");
        //     }
        // }

        // let response = read_message(stream.try_clone().expect("failed to read message"));
        // println!("Received {response:#?}");
    }
}

fn main() {
    let address = get_address_from_arguments(env::args().collect());

    println!("Client connected to: {address}");
    let stream = match TcpStream::connect(&address) {
        Ok(stream) => stream,
        Err(e) => {
            eprint!("Failed connect to address {}: {}", address, e);
            process::exit(1);
        }
    };

    sending_thread(stream);

    // let stream_arc = Arc::new(Mutex::new(
    //     stream.try_clone().expect("Failed to clone stream!"),
    // ));

    // thread::spawn(move || sending_thread(stream_arc));
}
