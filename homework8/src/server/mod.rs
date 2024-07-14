use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::TcpListener;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::Mutex;
use tokio::sync::broadcast::Sender;
use tokio::sync::broadcast;

use crate::common::*;

async fn handle_client(
    mut stream: OwnedReadHalf,
    _stream_w: &Arc<Mutex<OwnedWriteHalf>>,
    origin_client_addr: SocketAddr,
    sender: Sender<(MessageType, SocketAddr)>,
) {
    loop {
        match read_message(&mut stream).await {
            Err(e) => {
                // If we fail to parse len of incoming connection we assume we lost connection.
                eprintln!("Error receiving message from client {origin_client_addr}: {e}");
                break;
            }
            Ok((message, _)) => {
                println!("client {origin_client_addr} gonna send");
                if sender.send((message, origin_client_addr)).is_err() {
                    break;
                };

            }

        }
    }
    eprintln!("Server dropping connection with client {origin_client_addr}");
}

async fn listen_and_accept(server_address: &str) {
    // Bind to TCP stream
    let listener = match TcpListener::bind(server_address).await {
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

    let (broadcast_channel_sender, _) = broadcast::channel(1024);

    // Keep waiting for clients to connect, and spawn a thread for each client.
    loop {
        let stream = listener.accept().await;

        match stream {
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
            Ok((client_stream, this_clients_address)) => {
                println!("Server established connection with client {}", this_clients_address);

                let channel_sender = broadcast_channel_sender.clone();
                let mut channel_receiver = broadcast_channel_sender.subscribe();

                let (stream_reader, stream_writer) = client_stream.into_split();
                let stream_writer = Arc::new(Mutex::new(stream_writer));
                let stream_clone = stream_writer.clone();

                // Handle this client
                let _receive_task = tokio::spawn(async move {
                    handle_client(
                        stream_reader,
                        &stream_writer.clone(),
                        this_clients_address,
                        channel_sender,
                    )
                    .await

                }); 

                // Respond to incoming messages from one client, forward to all other clients
                let _send_task = tokio::spawn(async move {
                    while let Ok((message, origin_client_addr)) = channel_receiver.recv().await {
                        // FIXME: When suddenly dropping client (CTRL-C) this task still tries to send to that client, getting a broken pipe error. 
                        // Nothing breaks, but that error is still annoying
                        // Find why we're still receiving old client from that channel_receiver
                        if this_clients_address == origin_client_addr {
                            // Don't send messages to ourselves
                            continue;
                        }
                        let mut stream = stream_clone.lock().await;

                        // Different forwarding depending on message type
                        match message {
                            MessageType::Text(..) => {
                                if let Err(e) = send_message(&mut stream, &message).await {
                                    eprint!("Failed to forward text from client {origin_client_addr} to client: {origin_client_addr}: {e}");
                                } else {
                                    println!("Forwarded text bytes from client {origin_client_addr} to client: {origin_client_addr}");
                                }
                            }
                            MessageType::File(ref path, _) => {
                                if let Err(e) = send_message(&mut stream, &message).await {
                                    eprint!("Failed to forward file {path} from client {origin_client_addr} to client: {origin_client_addr}: {e}");
                                } else {
                                    println!( "Forwarded file {path} from client {origin_client_addr} to client: {origin_client_addr}");
                                }
                            }
                            MessageType::Image(..) => {
                                if let Err(e) = send_message(&mut stream, &message).await {
                                    eprint!("Failed to forward image from client {origin_client_addr} to client: {origin_client_addr}: {e}");
                                } else {
                                    println!(
                                        "Forwarded image from client {origin_client_addr} to client: {origin_client_addr}");
                                }
                            }
                        };



                        drop(stream);
                    }
                });
            }
        }
    }
}

pub async fn run(hostname: String, port: String) {
    let server_addr = format!("{}:{}", hostname, port);
    listen_and_accept(&server_addr).await;
}
