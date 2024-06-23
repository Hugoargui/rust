use std::fs::{self, OpenOptions};
use std::io::{self, Cursor, Seek, SeekFrom, Write};
use std::net::TcpStream;
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;

use anyhow::Result;
use chrono::Utc;
use image::io::Reader as ImageReader;

use crate::common::*;

// Thread that reads user input and sends relevant data to server
fn sending_thread(stream: Arc<Mutex<TcpStream>>) {
    let mut stream = stream.lock().expect("FATAL ERROR: failed to lock stream");
    loop {
        println!("> Enter text to send (or file <path>, image <path>, quit)");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");
        let input = input.trim();
        let (command, path) = input.split_once(' ').unwrap_or((input, ""));

        match command {
            "q" | "quit" => {
                println!("Quiting client");
                process::exit(1);
            }
            "file" => match generate_file_message(path) {
                Err(why) => {
                    eprintln!("Couldn't open {path} with error: {why}")
                }
                Ok(message) => {
                    if let Err(e) = send_message(&mut stream, &message) {
                        eprint!("Failed to send file {path}: {e}");
                    } else {
                        println!("Sending file: {path}");
                    }
                }
            },
            "image" => match generate_image_message(path) {
                Err(why) => {
                    eprintln!("Couldn't open {path} with error: {why}")
                }
                Ok(message) => {
                    if let Err(e) = send_message(&mut stream, &message) {
                        eprint!("Failed to send image {path}: {e}");
                    } else {
                        println!("Sending image: {path}");
                    }
                }
            },
            _ => {
                // no image or file, forward all data as plain text
                let new_message = MessageType::Text(input.to_string());
                if let Err(e) = send_message(&mut stream, &new_message) {
                    eprint!("Failed to send message: {e}");
                } else {
                    println!("Sending {new_message:?}");
                }
            }
        }
    }
}

// Thread that listens to incoming data from server and prints it to stdout.
fn receiving_thread(stream: Arc<Mutex<TcpStream>>) {
    let stream = stream.lock().expect("FATAL ERROR: Failed to lock stream");
    // Loop through all incoming messages
    loop {
        match calculate_message_length(&stream) {
            Err(e) => {
                // If server goes down or any error happens, lenght parsing will fail.
                eprintln!("Client lost connection with server with error: {}", e);
                break;
            }
            Ok(len) => {
                let message =
                    read_message(stream.try_clone().expect("failed to clone stream"), len);

                match message {
                    Err(why) => {
                        eprintln!("Failed to read message: {why}")
                    }
                    Ok(message) => match message {
                        MessageType::Text(ref raw_message) => {
                            println!("Received text: {raw_message}");
                        }
                        MessageType::File(ref path, ref file_contents) => {
                            handle_incoming_file(path, file_contents);
                        }
                        MessageType::Image(ref bytes) => {
                            handle_incoming_image(bytes);
                        }
                    },
                }

                // Done with processing, print usage again.
                println!("> Enter text to send (or .file <path>, .image <path>, .quit)");
            }
        }
    }
}

pub fn run(hostname: String, port: String) {
    let address = format!("{}:{}", hostname, port);

    println!("Client trying to connect to to: {address}");
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

    // Spawn the two threads. They share stdout and the stream but otherwise don't interact with each other.
    let thread_handle_1 = thread::spawn(move || sending_thread(stream1));
    let thread_handle_2 = thread::spawn(move || receiving_thread(stream2));

    let _ = thread_handle_1.join();
    let _ = thread_handle_2.join();
}

fn generate_file_message(path: &str) -> Result<MessageType> {
    match fs::read_to_string(path) {
        Err(why) => Err(why.into()),
        Ok(message) => Ok(MessageType::File(
            path.to_string(),
            message.as_bytes().to_vec(),
        )),
    }
}

fn generate_image_message(path: &str) -> Result<MessageType> {
    // Don't bother decoding image here, just forward it as raw bytes
    match std::fs::read(path) {
        Ok(image) => Ok(MessageType::Image(image)),
        Err(why) => Err(why.into()),
    }
}

fn handle_incoming_file(path: &str, raw_bytes: &[u8]) {
    fs::create_dir_all("files").expect("Cannot create directory");
    let path = format! {"files/{path}"};

    match OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path.clone())
    {
        Err(why) => {
            eprint!("Cannot store into {path} with error: {why}");
        }
        Ok(mut file) => {
            file.seek(SeekFrom::Start(0))
                .expect("file seek should never fail");
            match file.write_all(raw_bytes) {
                Err(why) => {
                    eprintln!("Failed to write into {path} with error: {why}");
                }
                Ok(_) => {
                    println!("File received and stored into {path}");
                }
            }
        }
    }
}

fn handle_incoming_image(raw_bytes: &[u8]) {
    // We receive raw data, it could be any image format, decode it as soo
    let reader = ImageReader::new(Cursor::new(raw_bytes))
        .with_guessed_format()
        .expect("Cursor io never fails");
    println!("Received image with format {:?}", reader.format());

    match reader.decode() {
        Err(why) => {
            eprint!("Failed to decode image with error: {why}");
        }
        Ok(image) => {
            let timestamp = Utc::now().timestamp();
            fs::create_dir_all("images").expect("Cannot create directory");
            let path = format! {"images/{timestamp}.png"};

            match image.save_with_format(path.clone(), image::ImageFormat::Png) {
                Err(why) => {
                    eprint!("Cannot store into {path} with error: {why}");
                }
                Ok(_) => {
                    println!("Image received and stored into {path}");
                }
            }
        }
    }
}
