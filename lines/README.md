# LinesCodec

This example adds some abstraction around serializing/deserializing `Strings` and interaction with Rust's [TcpStream](https://doc.rust-lang.org/stable/std/net/struct.TcpStream.html).

Simply for a little added flair, our server will reverse received `String`s before echoing them back to the client.

**(Inspired by the now removed [tokio example](https://github.com/tokio-rs/tokio/blob/9d4d076189822e32574f8123efe21c732103f4d4/examples/chat.rs))**

# Running the demo
From within the 'lines' directory we can start the server, and then in another terminal (tmux pane, ssh session, etc), run the client with a message of your choice

Server
```sh
$ cargo run --bin server
Starting server on '127.0.0.1:4000'
...
```

Client
```sh
$ cargo run --bin client -- Testing
gnitseT
```
