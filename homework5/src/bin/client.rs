#![allow(dead_code)]
#![allow(unused_imports)]

use std::env;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::{self, Read, Seek, SeekFrom, Write};
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
                let message = generate_file_message(path);
                send_message(&mut stream, &message);
            }
            "image" | ".image" => {
                println!("Sending image at path: {path}");
                let message = generate_image_message();
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
                eprintln!("Client lost connection with server with error: {}", e);
                break;
            }
            Ok(len) => {
                let message =
                    read_message(stream.try_clone().expect("failed to clone stream"), len);

                match message {
                    MessageType::Text(ref raw_message) => {
                        println!("{raw_message}");
                    }
                    MessageType::File(ref path, ref file_contents) => {
                        handle_incoming_file(path, file_contents);
                    }
                    MessageType::Image(ref bytes) => {
                        handle_incoming_image(bytes);
                    }
                }
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

fn generate_file_message(path: &str) -> MessageType {
    let message: String = fs::read_to_string(path).unwrap();
    println!("{}", message);

    MessageType::File(path.to_string(), message.as_bytes().to_vec())
}

fn generate_image_message() -> MessageType {
    MessageType::Image(vec![0, 1, 2, 3])
}

fn handle_incoming_file(path: &str, raw_bytes: &[u8]) {
    let string = String::from_utf8(raw_bytes.to_vec()).expect("Our bytes should be valid utf8");
    println!("Received file {path} with content {string}");
    // let mut file = OpenOptions::new()
    //     .read(true)
    //     .write(true) // <--------- this
    //     .truncate(true)
    //     .create(true)
    //     .open("/file/foo.txt")
    //     .unwrap();
    // file.seek(SeekFrom::Start(0)).unwrap();
    // file.write_all(raw_bytes).unwrap();
}

fn handle_incoming_image(raw_bytes: &[u8]) {
    println!("Received image with content {raw_bytes:?}");
}
