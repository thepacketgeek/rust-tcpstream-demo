# Raw TCP Bytes

This example shows low-level data (raw bytes) reading & writing with Rust's [TcpStream](https://doc.rust-lang.org/stable/std/net/struct.TcpStream.html).  Subsequent examples add abstractions over this, but it's helpful to understand what's happening under the hood and why abstractions make things easier.

# Detailed Walkthrough
You can see more details about how this demo was built with the [accompanying blog article](https://thepacketgeek.com/rust-tcpstream-p-01-reading-and-writing/)

# Running the demo
From within the 'raw' directory we can start the server, and then in another terminal (tmux pane, ssh session, etc), run the client with a message of your choice

Server
```
$ cargo run --bin server
Starting server on '127.0.0.1:4000'
...
```

Client
```
$ cargo run --bin client -- Hello
Hello
```
