use serde::{Deserialize, Serialize};
use std::io::prelude::*;
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

fn calculate_message_length(mut stream: &TcpStream) -> usize {
    let mut len_bytes = [0u8; 4];
    stream.read_exact(&mut len_bytes).unwrap();
    u32::from_be_bytes(len_bytes) as usize
}

pub fn read_message(mut stream: TcpStream) -> MessageType {
    let len = calculate_message_length(&stream);
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
