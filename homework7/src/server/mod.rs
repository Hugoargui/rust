use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

use dashmap::DashMap;

use crate::common::*;

fn handle_client(
    addr: SocketAddr,
    stream: TcpStream,
    clients: &Arc<DashMap<SocketAddr, TcpStream>>,
) {
    // Loop through all incoming traffic from this particular client
    loop {
        match calculate_message_length(&stream) {
            Err(_) => {
                // If we fail to parse len of incoming connection we assume we lost connection.
                eprintln!("Server lost connection with client {}", addr);
                clients.remove(&addr);
                break;
            }
            Ok(len) => {
                let message = read_message(
                    stream
                        .try_clone()
                        .expect("FATAL ERROR: failed to clone stream"),
                    len,
                );

                // Loop through all other clients in the client HashMap and forward the message
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

                    // Different forwarding depending on message type
                    match message {
                        Err(ref why) => {
                            eprintln!("Failed to read message: {why}")
                        }
                        Ok(ref message) => match message {
                            MessageType::Text(..) => {
                                if let Err(e) = send_message(&mut client, message) {
                                    eprint!("Failed to forward {len} bytes from client {addr} to client: {client_addr}: {e}");
                                } else {
                                    println!( "Forwarded {len} bytes from client {addr} to client: {client_addr}");
                                }
                            }
                            MessageType::File(ref path, _) => {
                                if let Err(e) = send_message(&mut client, message) {
                                    eprint!("Failed to forward file {path} of {len} bytes from client {addr} to client: {client_addr}: {e}");
                                } else {
                                    println!( "Forwarded file {path} of {len} bytes from client {addr} to client: {client_addr}");
                                }
                            }
                            MessageType::Image(..) => {
                                if let Err(e) = send_message(&mut client, message) {
                                    eprint!("Failed to forward image of {len} bytes from client {addr} to client: {client_addr}: {e}");
                                } else {
                                    println!("Forwarded image of {len} bytes from client {addr} to client: {client_addr}");
                                }
                            }
                        },
                    }
                }
            }
        }
    }
}

fn listen_and_accept(address: &str) {
    // Bind to TCP stream
    let listener = match TcpListener::bind(address) {
        Ok(listener) => {
            println!("Server listening on: {address}");
            listener
        }
        Err(e) => {
            terminate_with_message(
                format!("Server failed to bind to address {address} with error: {e}"),
                1,
            );
        }
    };

    // Hashmap to keep track of all connected clients.
    // Dashmap is thread safe so id doesn't need Mutex/rwlock.
    let client_map: Arc<DashMap<SocketAddr, TcpStream>> = Arc::new(DashMap::new());

    // Keep waiting for clients to connect, and spawn a thread for each client.
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
