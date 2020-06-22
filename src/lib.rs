//! Message Serialization/Deserialization (Protocol) for client <-> server communication
//!
//! Ideally you would use some existing Serialization/Deserialization,
//! but this is here to see what's going on under the hood.
//!
//! ## Libraries for serialization/deserialization:
//! [Serde](https://docs.rs/serde/1.0.114/serde/index.html)
//! [tokio_util::codec](https://docs.rs/tokio-util/0.3.1/tokio_util/codec/index.html)

use std::convert::From;
use std::io::{self, Read, Write};

use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};

pub const DEFAULT_SERVER_ADDR: &str = "127.0.0.1:4000";

/// Request object (client -> server)
#[derive(Debug)]
pub enum Request {
    /// Echo a message back
    Echo(String),
    /// Jumble up a message and return, with given amount of entropy?
    Jumble { message: String, amount: u16 },
}

/// Encode the Request type as a single byte (as long as we don't exceed 255 types)
impl From<&Request> for u8 {
    fn from(req: &Request) -> Self {
        match req {
            Request::Echo(_) => 1,
            Request::Jumble { .. } => 2,
        }
    }
}

/// Message format for Request is:
/// ```ignore
/// |    u8    |     u16     |     [u8]      | ... u16    |   ... [u8]         |
/// |   type   |    length   |  value bytes  | ... length |   ... value bytes  |
/// ```
///
/// Starts with a type, and then is an arbitrary length of (length/bytes) tuples
impl Request {
    /// Serialize Request to bytes (to send to server)
    pub fn serialize(&self, buf: &mut impl Write) -> io::Result<usize> {
        buf.write_u8(self.into())?; // Message Type byte
        let mut bytes_written: usize = 1;
        match self {
            Request::Echo(message) => {
                // Write the variable length message string, preceded by it's length
                let message = message.as_bytes();
                buf.write_u16::<NetworkEndian>(message.len() as u16)?;
                buf.write_all(&message)?;
                bytes_written += 2 + message.len();
            }
            Request::Jumble { message, amount } => {
                // Write the variable length message string, preceded by it's length
                let message_bytes = message.as_bytes();
                buf.write_u16::<NetworkEndian>(message_bytes.len() as u16)?;
                buf.write_all(&message_bytes)?;
                bytes_written += 2 + message.len();

                // We know that `amount` is always 2 bytes long, but are adding
                // the length here to stay consistent
                buf.write_u16::<NetworkEndian>(2)?;
                buf.write_u16::<NetworkEndian>(*amount)?;
                bytes_written += 4;
            }
        }
        Ok(bytes_written)
    }

    /// Deserialize Request to bytes (to receive from client)
    pub fn deserialize(mut buf: &mut impl Read) -> io::Result<Request> {
        match buf.read_u8()? {
            // Echo
            1 => Ok(Request::Echo(extract_string(&mut buf)?)),
            // Jumble
            2 => {
                let message = extract_string(&mut buf)?;
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

/// Response object from server
#[derive(Debug)]
pub struct Response(pub String);

/// Message format for Response is:
/// ```ignore
/// |     u16     |     [u8]      |
/// |    length   |  value bytes  |
/// ```
///
impl Response {
    /// Create a new response with a given message
    pub fn new(message: String) -> Self {
        Self(message)
    }

    /// Get the response message value
    pub fn message(&self) -> &str {
        &self.0
    }

    /// Serialize Response to bytes (to send to client)
    ///
    /// Returns the number of bytes written
    pub fn serialize(&self, buf: &mut impl Write) -> io::Result<usize> {
        let resp_bytes = self.0.as_bytes();
        buf.write_u16::<NetworkEndian>(resp_bytes.len() as u16)?;
        buf.write_all(&resp_bytes)?;
        Ok(3 + resp_bytes.len()) // Type + len + bytes
    }

    /// Deserialize Response to bytes (to receive from server)
    pub fn deserialize(mut buf: &mut impl Read) -> io::Result<Response> {
        let value = extract_string(&mut buf)?;
        Ok(Response(value))
    }
}

/// From a given readable buffer, read the next length (u16) and extract the string bytes
fn extract_string(buf: &mut impl Read) -> io::Result<String> {
    let length = buf.read_u16::<NetworkEndian>()?;
    let mut bytes = vec![0u8; length as usize];
    buf.read_exact(&mut bytes)?;
    String::from_utf8(bytes).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid utf8"))
}
