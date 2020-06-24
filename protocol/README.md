# Message Protocol for TcpStream

This example adds even more abstraction around custom structs and serializing/deserializing with Rust's [TcpStream](https://doc.rust-lang.org/stable/std/net/struct.TcpStream.html).

Our protocol defines a `Request` message that allow the client to request either:
- Echo a string
- Jumble a string with a specified amount of jumbling entropy

Along with a `Response` message for the server to respond with either of:
- The successfully echo/jumbled `String`
- An error message with reason

**(Inspired by pingcap [talent-plan](https://github.com/pingcap/talent-plan/tree/master/courses/rust) workshop)**

# Running the demo
From within the 'protocol' directory we can start the server, and then in another terminal (tmux pane, ssh session, etc), run the client with a message of your choice

Server
```sh
$ cargo run --bin server
Starting server on '127.0.0.1:4000'
...
```

Client
```sh
$ cargo run --bin client -- Hello
Connecting to 127.0.0.1:4000
'Hello' from the other side!
$ cargo run --bin client -- "This is my message" -j 100
Connecting to 127.0.0.1:4000
issageThis s my me
```
