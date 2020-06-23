# Raw TCP Bytes

For the start of our journey, let's look at what it takes to send and receive bytes with [TcpStream](https://doc.rust-lang.org/stable/std/net/struct.TcpStream.html)


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
'Hello' from the other side!
```
