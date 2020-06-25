//! Shared code between client & server

use std::io::{self, BufRead, Write};
use std::net::TcpStream;

pub const DEFAULT_SERVER_ADDR: &str = "127.0.0.1:4000";

///A smarter implementation of `extract_line` that supports writing messages also
pub struct LinesCodec {
    reader: io::BufReader<TcpStream>,
    writer: io::LineWriter<TcpStream>,
}

impl LinesCodec {
    /// Encapsulate a TcpStream with reader/writer functionality
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        let writer = io::LineWriter::new(stream.try_clone()?);
        let reader = io::BufReader::new(stream);
        Ok(Self { reader, writer })
    }

    /// Write this line (with a '\n' suffix) to the TcpStream
    pub fn send_message(&mut self, message: &str) -> io::Result<()> {
        self.writer.write_all(&message.as_bytes())?;
        // This will also signal a `writer.flush()` for us!
        self.writer.write(&['\n' as u8])?;
        Ok(())
    }

    /// Read a received message from the TcpStream
    pub fn read_message(&mut self) -> io::Result<String> {
        let mut line = String::new();
        // Use `BufRead::read_line()` to read a line from the TcpStream
        self.reader.read_line(&mut line)?;
        line.pop(); // Drop the trailing "\n"
        Ok(line)
    }
}
