# Running directly from this crate

To run directly, from directory rust/homework6 type: 

```
cargo run  [server or client] [optional host ip] [optional port] 
```
If no default host ip or ports are provided, localhost and 11111 will be used. 

Example: 

```
cargo run server localhost 8080
```
or 

```
cargo run client localhost 8080
```

# Using as a library

Otherwise, client and server can be imported as normal crates. 

```rust
use homework6::client;
use homework6::server;

// to run the server:
server::run(hostname, port);

// to launch one client
client::run(hostname, port);
```
Note that in that case, default hostname/port implementation is up to the user, the crates assume you already know which hostname/port you want to use. 

# Usage Details

 - Server must be running before any client is launched, otherwise clients can't connect
 - Only one server should run at once
 - As many clients as you want can run at the same time
 - Clients can be terminated at any time and the server can handle it
 - If the server terminates, all clients will panic and terminate. Client reconnection is out of scope for this homework. 

Usage: 
- to terminate the server, the only option is Control-C
- type q or quit to exit the client
- type file followed by the path of the file to send it. All other clients will save it wit the same name to files/xxx
- type image followed by the path to the image (in any valid image format) to send it. All clients will convert it to png and store it in images/timestamp.png
- type anything else and it will be forwarded as plain text to all clients

![image](https://github.com/Hugoargui/rust/assets/6458679/7d425d30-f50b-45c8-abec-58748f4ff258)


