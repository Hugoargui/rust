use std::fs::{self, OpenOptions};
use std::io::{self, Cursor, Seek, SeekFrom, Write};

use anyhow::Result;
use chrono::Utc;
use image::io::Reader as ImageReader;
use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

use crate::common::*;

// Thread that reads user input and sends relevant data to server
async fn sending_thread(mut stream: OwnedWriteHalf) {
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
                terminate_with_message("Quitting program", 0);
            }
            "file" => match generate_file_message(path) {
                Err(why) => {
                    eprintln!("Couldn't open {path} with error: {why}")
                }
                Ok(message) => {
                    if let Err(e) = send_message(&mut stream, &message).await {
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
                    if let Err(e) = send_message(&mut stream, &message).await {
                        eprint!("Failed to send image {path}: {e}");
                    } else {
                        println!("Sending image: {path}");
                    }
                }
            },
            _ => {
                // no image or file, forward all data as plain text
                let new_message = MessageType::Text(input.to_string());
                if let Err(e) = send_message(&mut stream, &new_message).await {
                    eprint!("Failed to send message: {e}");
                } else {
                    println!("Sending {new_message:?}");
                }
            }
        }
    }
}

// Thread that listens to incoming data from server and prints it to stdout.
async fn receiving_thread(mut stream: OwnedReadHalf) {
    // Loop through all incoming m<F15>essages
    loop {
        match read_message(&mut stream).await {
            Err(e) => {
                // If server goes down or any error happens, lenght parsing will fail.
                eprintln!("Client lost connection with server with error: {}", e);
                break;
            }
            Ok((message, _)) => {
                match message {
                    MessageType::Text(ref raw_message) => {
                        println!("Received text: {raw_message}");
                    }
                    MessageType::File(ref path, ref file_contents) => {
                        handle_incoming_file(path, file_contents);
                    }
                    MessageType::Image(ref bytes) => {
                        handle_incoming_image(bytes);
                    }
                }

                // Done with processing, print usage again.
                println!("> Enter text to send (or .file <path>, .image <path>, .quit)");
            }
        }
    }
}

pub async fn run(hostname: String, port: String) {
    let address = format!("{}:{}", hostname, port);

    println!("Client trying to connect to to: {address}");
    let stream = match TcpStream::connect(&address).await {
        Ok(stream) => stream,
        Err(e) => {
            terminate_with_message(format!("Failed connect to address {address}: {e}"), 1);
        }
    };

    let (read_half, write_half) = stream.into_split();

    // Spawn the two threads. They share stdout and the stream but otherwise don't interact with each other.
    let send_task = tokio::spawn(async move { sending_thread(write_half).await; });
    let receive_task = tokio::spawn(async move { receiving_thread(read_half).await; });
    tokio::try_join!(send_task, receive_task).expect("Fatal error: failed to join tasks at end of program");
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
