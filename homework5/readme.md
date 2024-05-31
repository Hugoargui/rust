To run, from directory rust/homework5 type: 

```
cargo run --bin server localhost 8080
```

or 

```
cargo run --bin client localhost 8080
```

 Server must be running before any client is launched, otherwise they can't connect
 Only one server should run at once
 As many clients as you want can run at the same time
 Clients can be terminated at any time and the server can handle it
 If the server terminates, all clients will panic and terminate.

Usage: 
- to terminate server, the only option is Control-C
- type q, quit or .quit to exit client
- type file or .file followed by the path of the file to send it. All other clients will save it wit the same name to files/xxx
- type image or .image followed by the path to the image to send it. All clients will convert it to png and store it in images/timestamp.png
- type anything else and it will be forwarded as plain text to all clients
