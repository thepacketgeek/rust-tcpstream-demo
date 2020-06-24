# Create a Messaging Protocol for TcpStream

In this series so far we've learned how to [read & write bytes](../raw) with our TcpStream and then how to [abstract over that with a `LinesCodec`](../lines) for sending and receiving `String` messages. In this demo, we'll look into what it takes to build a custom protocol for message passing more than a single type of thing (like a `String`).

To give our client and server more options for communicating, we'll be building a `Request` message that allow the client to request either:
- Echo a string
- Jumble a string with a specified amount of jumbling entropy

Along with a `Response` message for the server to respond with either of:
- The successfully echo/jumbled `String`
- An error message with details


# Serialization/Deserialization Libraries
As you can see, creating protocols for custom structs is a lot of work. The good news is that there are some really incredible, performant, and battle-tested libraries to make doing this easier! It's fun to see how this might be implemented, but I highly recommend checking out these libraries for your serialization needs:

- [Serde](https://docs.rs/serde/1.0.114/serde/index.html)
- [tokio_util::codec](https://docs.rs/tokio-util/0.3.1/tokio_util/codec/index.html)
- [bincode](https://github.com/servo/bincode)


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

**(Inspired by pingcap [talent-plan](https://github.com/pingcap/talent-plan/tree/master/courses/rust) workshop)**