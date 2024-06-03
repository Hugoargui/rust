use std::net::{SocketAddr, TcpListener, TcpStream};
use std::process;
use std::sync::Arc;
use std::thread;

use dashmap::DashMap;

use crate::common::*;

fn handle_client(
    addr: SocketAddr,
    stream: TcpStream,
    clients: &Arc<DashMap<SocketAddr, TcpStream>>,
) {
    loop {
        match calculate_message_length(&stream) {
            Err(_) => {
                eprintln!("Server lost connection with client {}", addr);
                clients.remove(&addr);
                break;
            }
            Ok(len) => {
                let message =
                    read_message(stream.try_clone().expect("failed to clone stream"), len);

                for mut client in clients.iter_mut() {
                    let client_addr = match client.peer_addr() {
                        Ok(addr) => addr,
                        Err(_) => {
                            eprintln!("Failed to get client address");
                            continue;
                        }
                    };

                    if client_addr == addr {
                        // Don't send message to ourselves
                        continue;
                    }

                    match message {
                        MessageType::Text(..) => {
                            println!(
                                "Forwarded {len} bytes from client {addr} to client: {client_addr}"
                            );
                            send_message(&mut client, &message);
                        }
                        MessageType::File(ref path, _) => {
                            println!( "Forwarded file {path} of {len} bytes from client {addr} to client: {client_addr}");
                            send_message(&mut client, &message);
                        }
                        MessageType::Image(..) => {
                            println!("Forwarded image of {len} bytes from client {addr} to client: {client_addr}");
                            send_message(&mut client, &message);
                        }
                    }
                }
            }
        }
    }
}

fn listen_and_accept(address: &str) {
    let listener = match TcpListener::bind(address) {
        Ok(listener) => {
            println!("Server listening on: {address}");
            listener
        }
        Err(e) => {
            eprintln!(
                "Server failed to bind to address {} with error: {}",
                address, e
            );
            process::exit(1);
        }
    };

    let client_map: Arc<DashMap<SocketAddr, TcpStream>> = Arc::new(DashMap::new());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let addr = stream.peer_addr().expect("Failed to get clients address");
                println!("Server established connection with client {}", addr);
                client_map.insert(
                    addr,
                    stream
                        .try_clone()
                        .expect("failed to insert client into client map"),
                );

                let clone = client_map.clone();
                thread::spawn(move || handle_client(addr, stream, &clone));
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}

pub fn run(hostname: String, port: String) {
    let address = format!("{}:{}", hostname, port);
    listen_and_accept(&address);
}
