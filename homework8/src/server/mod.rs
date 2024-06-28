use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
// use std::thread;

use dashmap::DashMap;

use crate::common::*;

async fn handle_client(
    origin_client_addr: SocketAddr,
    stream: TcpStream,
    clients: &Arc<DashMap<SocketAddr, TcpStream>>,
) {
    // Loop through all incoming traffic from this particular client
    loop {
        match calculate_message_length(&stream) {
            Err(_) => {
                // If we fail to parse len of incoming connection we assume we lost connection.
                eprintln!("Server lost connection with client {}", origin_client_addr);
                clients.remove(&origin_client_addr);
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
                    let target_client_addr = match client.peer_addr() {
                        Ok(addr) => addr,
                        Err(_) => {
                            eprintln!("Failed to get client address");
                            continue;
                        }
                    };

                    if target_client_addr == origin_client_addr {
                        // Don't send message to client that sent it
                        continue;
                    }

                    // Different forwarding depending on message type
                    match message {
                        Err(ref why) => {
                            eprintln!("Failed to read message: {why}")
                        }
                        Ok(ref message) => handle_message(
                            message,
                            &origin_client_addr,
                            &target_client_addr,
                            &mut client,
                            len,
                        ),
                    }
                }
            }
        }
    }
}

fn handle_message(
    message: &MessageType,
    origin_client_addr: &SocketAddr,
    target_client_addr: &SocketAddr,
    client: &mut TcpStream,
    len: usize,
) {
    match message {
        MessageType::Text(..) => {
            if let Err(e) = send_message(client, message) {
                eprint!("Failed to forward {len} bytes from client {origin_client_addr} to client: {target_client_addr}: {e}");
            } else {
                println!("Forwarded {len} bytes from client {origin_client_addr} to client: {target_client_addr}");
            }
        }
        MessageType::File(ref path, _) => {
            if let Err(e) = send_message(client, message) {
                eprint!("Failed to forward file {path} of {len} bytes from client {origin_client_addr} to client: {target_client_addr}: {e}");
            } else {
                println!( "Forwarded file {path} of {len} bytes from client {origin_client_addr} to client: {target_client_addr}");
            }
        }
        MessageType::Image(..) => {
            if let Err(e) = send_message(client, message) {
                eprint!("Failed to forward image of {len} bytes from client {origin_client_addr} to client: {target_client_addr}: {e}");
            } else {
                println!(
                    "Forwarded image of {len} bytes from client {origin_client_addr} to client: {target_client_addr}");
            }
        }
    }
}

async fn listen_and_accept(server_address: &str) {
    // Bind to TCP stream
    // TODO: replace by tokio tcplistener
    // Only issue I have is how to split the tokio::stream for various clients, try_clone not possible
    let listener = match TcpListener::bind(server_address) {
        Ok(listener) => {
            println!("Server listening on: {server_address}");
            listener
        }
        Err(e) => {
            terminate_with_message(
                format!("Server failed to bind to address {server_address} with error: {e}"),
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
                let client_addr = stream.peer_addr().expect("Failed to get clients address");
                println!("Server established connection with client {}", client_addr);
                client_map.insert(
                    client_addr,
                    stream
                        .try_clone()
                        .expect("failed to insert client into client map"),
                );

                let clone = client_map.clone();
                tokio::spawn(async move {
                     handle_client(client_addr, stream, &clone).await;
                }); 
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}

pub async fn run(hostname: String, port: String) {
    let server_addr = format!("{}:{}", hostname, port);
    listen_and_accept(&server_addr).await;
}
