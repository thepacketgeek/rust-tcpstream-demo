# Create a Messaging Protocol for TcpStream

In this series so far we've learned how to [read & write bytes](../raw) with our TcpStream and then how to [abstract over that with a `LinesCodec`](../lines) for sending and receiving `String` messages. In this demo, we'll look into what it takes to build a custom protocol for message passing more than a single type of thing (like a `String`).

# Defining our Messages
To give our client and server more options for communicating we'll create:

- A `Request` message that allow the client to request either:
  - Echo a string
  - Jumble a string with a specified amount of jumbling entropy
- A `Response` message for the server to respond with the successfully echo/jumbled `String`

```rust
/// Request object (client -> server)
#[derive(Debug)]
pub enum Request {
    /// Echo a message back
    Echo(String),
    /// Jumble up a message with given amount of entropy before echoing
    Jumble { message: String, amount: u16 },
}

/// Response object from server
///
/// In the real-world, this would likely be an enum as well to signal Success vs. Error
/// But since we're showing that capability with the `Request` struct, we'll keep this one simple
#[derive(Debug)]
pub struct Response(pub String);
```

# Serialization
In the previous demos, we relied on `String::as_bytes()` to serialize the string characters to the byte slice `&[u8]` we passed to `TcpStream::write_all()`. The structs defined above don't have any default serialization capabilities so this example focuses on how to implement that ourselves.


## Serialization/Deserialization Libraries
**Quick detour**: As you'll soon find out, serializing our custom structs is a lot of work. The good news is that there are some really incredible, performant, and battle-tested libraries to make doing this easier! It's fun to see how this might be implemented, but I highly recommend checking out these libraries for your serialization needs:

- [Serde](https://docs.rs/serde/1.0.114/serde/index.html)
- [tokio_util::codec](https://docs.rs/tokio-util/0.3.1/tokio_util/codec/index.html)
- [bincode](https://github.com/servo/bincode)

## Serializing the Request struct
Serialization & Deserialization needs to be **symmetric** (i.e., it round-trips) so that the struct serialized by the client is deserialized by the server back into an identical struct:

Serializing Symmetry (round-trip) pseudo-code
```rust
let message = Request { ... };

// Equivalent to std::str::from_utf8("Hello".as_bytes())
let roundtripped_message = deserialize_message(serialize_message(&message));

// Symmetric serialization means these will match (given the struct has `Eq`)
assert_eq!(message, roundtripped_message);
```

In order to serialize `Request` we need to decide what it looks like on the wire (as a `[u8]`). Since `Request` has two "types" (Echo and Jumble), we'll need to encode that type to help the deserialization know what it's looking for. Here's an approach for the `Request` byte layout:

```
|    u8    |     u16     |     [u8]      | ... u16    |   ... [u8]       |
|   type   |    length   |    bytes      | ... length |   ... bytes      |
           ^---    struct field        --^--   possibly more fields?   --^-- ...
```

- **type**: A codified representation of the `Request` type (E.g. Echo == 0, Jumble == 1)
- **length**/**bytes**: The **L**ength and **V**alue from **T**ype-Length-Value [TLV]
  - Each message struct has specific field types, but TLV is useful when fields are variadic
- **possibly more fields**:
  - Again, each message struct knows it's fields for deserialization, so these length/byte groups can go on for each struct field when needed

We already know how to serialize a `String` with `as_bytes()`, and we can use [byteorder](https://crates.io/crates/byteorder) to serialize our numbers with the correct [Endianness](https://en.wikipedia.org/wiki/Endianness) (spoiler: `BigEndian`, aliased as `NetworkEndian`). Let's walk through the serialization steps:

### Request Type
As we can see above, we need to codify our `Request` types (Echo and Jumble) into a number (`u8`, which allows us up to 255 types!). To help prevent bugs, let's implement `From` for `Request` -> `u8` to have a consistent way of serializing the type:

```rust
use std::convert::From;

/// Encode the Request type as a single byte (as long as we don't exceed 255 types)
///
/// We use `&Request` since we don't actually need to own or mutate the request fields
impl From<&Request> for u8 {
    fn from(req: &Request) -> Self {
        match req {
            Request::Echo(_) => 1,
            Request::Jumble { .. } => 2,
        }
    }
}
```

### Request Fields
Let's finish the building blocks for serialize the `Request` with examples for writing the **length**/**value** for `String` and numbers:

`String` is straightforward and we've used `as_bytes()` in previous demos, and `byteorder` adds extension methods to `Write` for numbers:
```rust
use std::io::{self, Write};
use byteorder::{NetworkEndian, WriteBytesExt};

let mut bytes: Vec<u8> = vec![];  // Just an example buffer that supports `Write`

let message = String::from("Hello");
// byteorder in action
bytes.write_u16::<NetworkEndian>(message.len()).unwrap();
bytes.write_all(message.as_bytes()).unwrap();
```

### Write Request
We now have all the pieces to add a method for `Request` that receives a mutable reference to some buffer that implements `Write` (like our `TcpStream` :)), and serialize a `Request::Echo`:

```rust
/// Starts with a type, and then is an arbitrary length of (length/bytes) tuples
impl Request {
    /// Serialize Request to bytes (to send to server)
    pub fn serialize(&self, buf: &mut impl Write) -> io::Result<()> {
        // Derive the u8 Type field from `impl From<&Request> for u8`
        buf.write_u8(self.into())?;

        // Select the serialization based on our `Request` type
        match self {
            Request::Echo(message) => {
                // Write the variable length message string, preceded by it's length
                let message = message.as_bytes();
                buf.write_u16::<NetworkEndian>(message.len() as u16)?;
                buf.write_all(&message)?;
            }
            Request::Jumble { message, amount } => {
                todo!()
            }
        }
        Ok(())
    }
}
```

Now the `Request::Jumble` serialization is just a slight variation (adding the `amount` field after the `message`):

```rust
// ...
        match self {
            // ... 
            Request::Jumble { message, amount } => {
                // Write the variable length message string, preceded by it's length
                let message_bytes = message.as_bytes();
                buf.write_u16::<NetworkEndian>(message_bytes.len() as u16)?;
                buf.write_all(&message_bytes)?;

                // We know that `amount` is always 2 bytes long
                buf.write_u16::<NetworkEndian>(2)?;
                // followed by the amount value
                buf.write_u16::<NetworkEndian>(*amount)?;
            }
        }
/// ...
```

Tada! A serialized `Request` in the bank! The `Response` struct is even simpler with its single `String` value, so you can see the serialization in the [demo lib.rs](src/lib.rs#L123)


## Deserializing the Request struct
We already have the byte layout figured out, so deserializing should essentially be the reverse of our `Request::serialize()` method above. `byteorder` also gives us `ReadBytesExt` to add extensions to `Read` that `TcpStream` implements. The trickiest part is reading the variable length `String` and this will happen in a few places for `Request::Echo`, `Request::Jumble`, and `Response` so let's break out this logic into a function:

```rust
/// From a given readable buffer (TcpStream), read the next length (u16) and extract the string bytes ([u8])
fn extract_string(buf: &mut impl Read) -> io::Result<String> {
    // byteorder ReadBytesExt
    let length = buf.read_u16::<NetworkEndian>()?;

    // Given the length of our string, only read in that quantity of bytes
    let mut bytes = vec![0u8; length as usize];
    buf.read_exact(&mut bytes)?;

    // And attempt to decode it as UTF8
    String::from_utf8(bytes).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid utf8"))
}
```

Our `deserialize()` method should be straight-forward to read now, especially with `extract_string` at our disposal:

```rust
use std::io::{self, Read};
use byteorder::{NetworkEndian, ReadBytesExt};


impl Request {
    /// Deserialize Request from bytes (to receive from TcpStream)
    /// returning a `Request` struct
    pub fn deserialize(mut buf: &mut impl Read) -> io::Result<Request> {
        /// We'll match the same `u8` that is used to recognize which request type this is
        match buf.read_u8()? {
            // Echo
            1 => Ok(Request::Echo(extract_string(&mut buf)?)),
            // Jumble
            2 => {
                let message = extract_string(&mut buf)?;
                // amount length is not used since we know it's 2 bytes
                let _amount_len = buf.read_u16::<NetworkEndian>()?;
                let amount = buf.read_u16::<NetworkEndian>()?;
                Ok(Request::Jumble { message, amount })
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid Request Type",
            )),
        }
    }

}
```

Wow! We now have the ability to test round-tripping of our structs!

```rust
use crate::*;
use std::io::Cursor;

#[test]
fn test_request_roundtrip() {
    let req = Request::Echo(String::from("Hello"));

    let mut bytes: Vec<u8> = vec![];
    req.serialize(&mut bytes).unwrap();

    let mut reader = Cursor::new(bytes); // Simulating our TcpStream
    let roundtrip_req = Request::deserialize(&mut reader).unwrap();

    assert!(matches!(roundtrip_req, Request::Echo(_)));
    assert_eq!(roundtrip_req.message(), "Hello");
}
```

# Using our new Protocol
Well, if you're still with me here, congrats! That was a lot of work but you're about to see how it all pays off when we use the message structs in our client and server.

I'll leave it up as an exercise to check out the [full protocol implementation](src/lib.rs) where we add `Serialize` and `Deserialize` traits for our methods above and make using our protocol as easy as:

```rust
use std::io;

fn main() -> io::Request<()> {
    let req = Request::Jumble {
        message: "Hello",
        amount: 80,
    };
    
    Protocol::connect("127.0.0.1:4000")
        .and_then(|mut client| {
            client.send_message(&req)?;
            Ok(client)
        })
        .and_then(|mut client| client.read_message::<Response>())
        .map(|resp| println!("{}", resp.message()))
}
```


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