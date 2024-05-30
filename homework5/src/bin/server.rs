#![allow(dead_code)]
#![allow(unused_imports)]
// use std::collections::HashMap;
use std::env;
use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::process;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

use dashmap::DashMap;

use homework5::*;

fn handle_client(addr: SocketAddr, mut stream: TcpStream) {
    loop {
        match calculate_message_length(&stream) {
            Err(_) => {
                eprintln!("Server lost connection with client {}", addr);
                break;
            }
            Ok(len) => {
                let message =
                    read_message(stream.try_clone().expect("failed to clone stream"), len);
                println!("Received: {message:?}");

                let _ = MessageType::Text("Received".to_string());

                println!("Responded: {message:?}");
                send_message(&mut stream, &message);
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

    // let client_map: DashMap<SocketAddr, TcpStream> = DashMap::new();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let addr = stream.peer_addr().expect("Failed to get clients address");
                println!("Server established connection with client {}", addr);
                // client_map.insert(addr, stream.try_clone().expect("failed to insert client into client map"));

                thread::spawn(move || handle_client(addr, stream));
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }

    // let client_map: ClientMap = Arc::new(RwLock::new(HashMap::new()));
    // pub type ClientMap = Arc<RwLock<HashMap<SocketAddr, TcpStream>>>;
    // let clients = Arc::clone(&client_map);

    // let mut clients: HashMap<SocketAddr, TcpStream> = HashMap::new();
    //
    // for stream in listener.incoming() {
    //     let mut stream = stream.unwrap();
    //     let addr = stream.peer_addr().unwrap();
    //     clients.insert(addr, stream.try_clone().unwrap());
    //
    //     let message = read_message(clients.get(&addr).unwrap().try_clone().unwrap());
    //
    //     println!("Received: {message:?}");
    //
    //     let _ = MessageType::Text("Received".to_string());
    //
    //     println!("Responded: {message:?}");
    //     send_message(&mut stream, &message);
    // }
}

fn main() {
    let address = get_address_from_arguments(env::args().collect());

    listen_and_accept(&address);
}
