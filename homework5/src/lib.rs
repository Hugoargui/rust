use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};
use std::net::TcpStream;

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Text(String),
    Image(Vec<u8>),
    File(String, Vec<u8>),
}

pub fn serialize_message(message: &MessageType) -> String {
    serde_json::to_string(&message).unwrap()
}

pub fn deserialize_message(data: &[u8]) -> MessageType {
    serde_json::from_slice(data).unwrap()
}

pub fn calculate_message_length(mut stream: &TcpStream) -> io::Result<usize> {
    let mut len_bytes = [0u8; 4];
    match stream.read_exact(&mut len_bytes) {
        Ok(()) => Ok(u32::from_be_bytes(len_bytes) as usize),
        Err(e) => Err(e),
    }
}

pub fn read_message(mut stream: TcpStream, len: usize) -> MessageType {
    let mut buffer = vec![0u8; len];

    stream.read_exact(&mut buffer).unwrap();

    deserialize_message(&buffer)
}

pub fn send_message(stream: &mut TcpStream, message: &MessageType) {
    let serialized = serialize_message(message);

    // Send the length of the serialized message (as 4-byte value).
    let len = serialized.len() as u32;
    let _ = stream.write(&len.to_be_bytes()).unwrap();

    // Send the serialized message.
    stream.write_all(serialized.as_bytes()).unwrap();
}

pub fn get_address_from_arguments(args: Vec<String>) -> String {
    let default_host = &String::from("localhost");
    let default_port = &String::from("11111");

    let hostname = args.get(1).unwrap_or(default_host);
    let port = args.get(2).unwrap_or(default_port);
    format!("{}:{}", hostname, port)
}
