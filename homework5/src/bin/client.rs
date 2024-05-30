#![allow(unused)]

use homework5::{deserialize_message, read_message, send_message, serialize_message, MessageType};
use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};

fn main() {
    let default_host = "localhost".to_string();
    let default_port = "11111".to_string();
    let hostname = default_host;
    let port = default_port;
    let address = format!("{}:{}", hostname, port);

    let new_message = MessageType::Text("hello".to_string());

    let serialized = serialize_message(&new_message);

    println!("Client connected to: {address}");
    let mut stream = TcpStream::connect(address).unwrap();

    println!("Sending {new_message:?}");
    send_message(&mut stream, &new_message);

    let response = read_message(stream);
    println!("Received {response:#?}");
}
