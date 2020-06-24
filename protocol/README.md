# Message Protocol for TcpStream

This example adds even more abstraction around custom structs and serializing/deserializing with Rust's [TcpStream](https://doc.rust-lang.org/stable/std/net/struct.TcpStream.html).

Our protocol defines a `Request` message that allow the client to request either:
- Echo a string
- Jumble a string with a specified amount of jumbling entropy

Along with a `Response` message for the server to respond with either of:
- The successfully echo/jumbled `String`
- An error message with reason

**(Inspired by pingcap [talent-plan](https://github.com/pingcap/talent-plan/tree/master/courses/rust) workshop)**

# Building a messaging Protocol for TcpStream

In this series so far we've learned how to [read & write bytes](../raw) with our TcpStream and then how to [abstract over that with a `LinesCodec`](../lines) for sending and receiving `String` messages. In this article, we'll look into what it takes to build a custom protocol for message passing more than a single type of thing (like a `String`).

- A LinesCodec that abstracts away `String` serialization/deserialization & TcpStream I/O
- A Client that uses the LinesCodec to send and print returned `String`s
- A Server that also uses the LinesCodec and reverses Strings before echoing them back

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
