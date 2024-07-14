use std::process;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::io::{ AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageType {
    Text(String),
    Image(Vec<u8>),
    File(String, Vec<u8>),
}

pub fn serialize_message(message: &MessageType) -> Result<String> {
    Ok(serde_json::to_string(&message)?)
}

pub fn deserialize_message(data: &[u8]) -> Result<MessageType> {
    Ok(serde_json::from_slice(data)?)
}

// pub async fn read_message(stream: &mut ReadHalf<TcpStream>) -> Result<(MessageType, usize)> {
pub async fn read_message(stream: &mut OwnedReadHalf) -> Result<(MessageType, usize)> {

    // Parse an incoming message and take the first 4 bytes as the lenght
    // Disconnection requests by the client have less than 4 bytes, so they will fail the parsing.
    let mut len_bytes = [0u8; 4];
    let mut reader = BufReader::new(stream);
    let _ =  reader.read_exact(&mut len_bytes).await?;
    let len = u32::from_be_bytes(len_bytes) as usize;

    // We have caltulcated the len, use it to read
    let mut buffer = vec![0u8; len];
    reader.read_exact(&mut buffer).await?;
    let message = deserialize_message(&buffer)?;
    Ok((message, len))
}

pub async fn send_message(stream: &mut OwnedWriteHalf, message: &MessageType) -> Result<()> {
    let serialized = serialize_message(message)?;

    // Send the length of the serialized message (as 4-byte value).
    let len = serialized.len() as u32;
    let _ = stream.write(&len.to_be_bytes()).await?;

    // Send the serialized message.
    stream.write_all(serialized.as_bytes()).await?;

    Ok(())
}

pub fn terminate_with_message<S: AsRef<str>>(message: S, code: i32) -> ! {
    eprintln!("{}", message.as_ref());
    process::exit(code);
}
