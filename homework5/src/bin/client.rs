#![allow(dead_code)]
#![allow(unused_imports)]

use std::env;
use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;

use homework5::*;

fn sending_thread(stream: Arc<Mutex<TcpStream>>) {
    let mut stream = stream.lock().unwrap();
    loop {
        println!("Enter text to send (or .file <path>, .image <path>, .quit)");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");
        let input = input.trim();
        let (command, path) = input.split_once(' ').unwrap_or((input, ""));

        match command {
            "q" | "quit" | ".quit" => {
                println!("Quiting client");
                process::exit(1);
            }
            "file" | ".file" => {
                println!("Sending file at path: {path}");
                let message = MessageType::File(path.to_string(), vec![0, 1, 2, 3]);
                send_message(&mut stream, &message);
            }
            "image" | ".image" => {
                println!("Sending image at path: {path}");
                let message = MessageType::Image(vec![0, 1, 2, 3]);
                send_message(&mut stream, &message);
            }
            _ => {
                let new_message = MessageType::Text(input.to_string());
                println!("Sending {new_message:?}");
                send_message(&mut stream, &new_message);
            }
        }
    }
}

fn receiving_thread(stream: Arc<Mutex<TcpStream>>) {
    let stream = stream.lock().unwrap();
    loop {
        match calculate_message_length(&stream) {
            Err(e) => {
                eprintln!("Server lost connection with server with error: {}", e);
                break;
            }
            Ok(len) => {
                let message =
                    read_message(stream.try_clone().expect("failed to clone stream"), len);
                println!("Received: {message:?}");
            }
        }
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

    // sending_thread(stream);

    let stream1 = Arc::new(Mutex::new(
        stream.try_clone().expect("Failed to clone stream!"),
    ));
    let stream2 = Arc::new(Mutex::new(
        stream.try_clone().expect("Failed to clone stream!"),
    ));

    let thread_handle_1 = thread::spawn(move || sending_thread(stream1));
    let thread_handle_2 = thread::spawn(move || receiving_thread(stream2));

    let _ = thread_handle_1.join();
    let _ = thread_handle_2.join();
}
