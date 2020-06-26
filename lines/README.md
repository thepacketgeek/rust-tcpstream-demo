# Building a LinesCodec for TcpStream

In the [previous demo](../raw) we learned how to read & write bytes with our TcpStream, but the calling code had to be aware of that and do the serialization/deserialization.  In this demo, we'll continue using `BufRead` and `BufReader` to build:

- A `LinesCodec` that abstracts away `String` serialization/deserialization & TcpStream I/O
- A client that uses the `LinesCodec` to send and print returned `String`s
- A server that also uses the `LinesCodec` and reverses `String`s before echoing them back

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


# LinesCodec
Our goals for this LinesCodec implementation are to abstract away:
- `TcpStream` reading & writing
  - We know enough about this now to know that the client & server shouldn't worry about the correct incantation to send & receive data
  - Even copy/pasting the code around is bound to cause an issue (like a forgotten `stream.flush()`)
- `String` serialization & deserialization
  - The client/server code shouldn't care how the data is represented on the wire, our codec will take care of that!

## A Type to the rescue!
Starting off with the I/O management abstraction, let's define a new type to own and interact with `TcpStream`. From the previous demo, we know we'll want to use the [BufReader](https://doc.rust-lang.org/std/io/struct.BufReader.html) and its `read_line()` method for reading data.

For writing data, we have three options:
  - Use TcpStream directly, identical to what we did in the previous demo
  - Use [BufWriter](https://doc.rust-lang.org/std/io/struct.BufWriter.html) which is a good logical jump given our use of `BufReader`
  - Use [LineWriter](https://doc.rust-lang.org/stable/std/io/struct.LineWriter.html) which sounds like an even closer match to what we want


I'm choosing to use `LineWriter` because it seems like a better approach to what we're wanting to do, based of of its documentation:

> Wraps a writer and buffers output to it, flushing whenever a newline (0x0a, `\n`) is detected.
> 
> The `BufWriter` struct wraps a writer and buffers its output. But it only does this batched write when it goes out of scope, or when the internal buffer is full. Sometimes, you'd prefer to write each line as it's completed, rather than the entire buffer at once. Enter LineWriter. It does exactly that.

Great! Let's define our type and define `LinesCodec::new()` to build new instances:

```rust
use std::io::{self, BufRead, Write};
use std::net::TcpStream;

pub struct LinesCodec {
    // Our buffered reader & writers
    reader: io::BufReader<TcpStream>,
    writer: io::LineWriter<TcpStream>,
}

impl LinesCodec {
    /// Encapsulate a TcpStream with buffered reader/writer functionality
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        // Both BufReader and LineWriter need to own a stream
        // We can clone the stream to simulate splitting Tx & Rx with `try_clone()`
        let writer = io::LineWriter::new(stream.try_clone()?);
        let reader = io::BufReader::new(stream);
        Ok(Self { reader, writer })
    }
}
```

### Reading and Writing
With the `LinesCodec` struct and its buffered reader/writers, we can continue our implementation to borrow the reading and writing code from the previous demo:

```rust
...
impl LinesCodec {
    /// Write the given message (appending a newline) to the TcpStream
    pub fn send_message(&mut self, message: &str) -> io::Result<()> {
        self.writer.write(&message.as_bytes())?;
        // This will also signal a `writer.flush()` for us; thanks LineWriter!
        self.writer.write(&['\n' as u8])?;
        Ok(())
    }

    /// Read a received message from the TcpStream
    pub fn read_message(&mut self) -> io::Result<String> {
        let mut line = String::new();
        // Use `BufRead::read_line()` to read a line from the TcpStream
        self.reader.read_line(&mut line)?;
        line.pop(); // Remove the trailing "\n"
        Ok(line)
    }
}
```

# Using LinesCodec in the client
With the `TcpStream` out of the way, let's refactor our client code from the previous demo and see how easy this can be:

```rust
use std::io;
use std::new::TcpStream;

use crate::LinesCodec;


fn main() -> io::Result<()> {
    // Establish a TCP connection with the farend
    let mut stream = TcpStream::connect("127.0.0.1:4000")?;

    // Codec is our interface for reading/writing messages.
    // No need to handle reading/writing directly
    let mut codec = LinesCodec::new(stream)?;

    // Serializing & Sending is now just one line
    codec.send_message("Hello")?;

    // And same with receiving the response!
    println!("{}", codec.read_message()?);
    Ok(())

}
```

# Using LinesCodec in the server
And now we have some very similar work to update our server to use `LinesCodec`. We'll define a `handle_connection` function that:
- Takes ownership of a `TcpStream` and wraps it in `LinesCodec`
- Receives a message (`String`) from the client and reverses it
- Sends the message back to the client, again using `LinesCodec`

```rust
use std::io;
use std::net::TcpStream;

use crate:LinesCodec;

/// Given a TcpStream:
/// - Deserialize the message
/// - Serialize and write the echo message to the stream
fn handle_connection(stream: TcpStream) -> io::Result<()> {
    let mut codec = LinesCodec::new(stream)?;

    // Read & reverse the received message
    let message: String = codec
        .read_message()
        // Reverse message
        .map(|m| m.chars().rev().collect())?;

    // And use the codec to return it
    codec.send_message(&message)?;
    Ok(())
}
```

Using the codec makes the business logic code much clearer and there are fewer opportunities to mis-manage newlines or forget `flush()`. Check out the [client](src/bin/client.rs) and [server](src/bin/server.rs) code for a full runnable example.

In the [next demo](../protocol) we dive even deeper to build a custom protocol with our own serialization/deserialization!

# Running the demo
From within this `./lines` directory we can start the server, and then in another terminal (tmux pane, ssh session, etc), run the client with a message of your choice

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


**(Inspired by the now removed [tokio example](https://github.com/tokio-rs/tokio/blob/9d4d076189822e32574f8123efe21c732103f4d4/examples/chat.rs))**