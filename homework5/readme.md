To run, from directory rust/homework5 type: 

```
cargo run --bin [server or client] [optional host ip] [optional port] 
```
If no default host ip or ports are provided, localhost and 11111 will be used. 

Example: 

```
cargo run --bin server localhost 8080
```

or 

```
cargo run --bin client localhost 8080
```

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

![image](https://github.com/Hugoargui/rust/assets/6458679/85d62d4d-6f36-44ed-a4d8-0b879faf4a86)

