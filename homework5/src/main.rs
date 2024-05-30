use homework5::{read_message, send_message, MessageType};
use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener, TcpStream};

fn listen_and_accept(address: &str) {
    let listener = TcpListener::bind(address).unwrap();

    let mut clients: HashMap<SocketAddr, TcpStream> = HashMap::new();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let addr = stream.peer_addr().unwrap();
        clients.insert(addr, stream.try_clone().unwrap());

        println!("Waiting for message");
        let message = read_message(clients.get(&addr).unwrap().try_clone().unwrap());

        println!("Received: {message:?}");

        let _ = MessageType::Text("Received".to_string());

        println!("Responded: {message:?}");
        send_message(&mut stream, &message);
    }
}

fn main() {
    let default_host = "localhost".to_string();
    let default_port = "11111".to_string();
    let hostname = default_host;
    let port = default_port;
    let address = format!("{}:{}", hostname, port);

    println!("Server listening on: {address}");

    listen_and_accept(&address);
}
