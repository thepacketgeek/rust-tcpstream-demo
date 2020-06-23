use std::io::{self, BufReader, BufWriter, Write};
use std::net::{SocketAddr, TcpStream};

use structopt::StructOpt;

use tcp_demo_raw::{extract_string_unbuffered, DEFAULT_SERVER_ADDR};

#[derive(Debug, StructOpt)]
#[structopt(name = "client")]
struct Args {
    message: String,
    /// Server destination address
    #[structopt(long, default_value = DEFAULT_SERVER_ADDR, global = true)]
    addr: SocketAddr,
}

/// Client establishes a connection and has Buffered Reader/Writers
struct TcpClient {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl TcpClient {
    /// Establish a connection, wrap stream in BufReader/Writer
    pub fn connect(dest: SocketAddr) -> io::Result<Self> {
        let stream = TcpStream::connect(dest)?;
        eprintln!("Connecting to {}", dest);
        Ok(Self {
            reader: BufReader::new(stream.try_clone()?),
            writer: BufWriter::new(stream),
        })
    }

    /// Serialize a request to the server and deserialize the response
    pub fn send_request(&mut self, message: &str) -> io::Result<String> {
        self.writer.write_all(&message.as_bytes())?;
        self.writer.flush()?;
        extract_string_unbuffered(&mut self.reader)
    }
}

fn main() -> Result<(), String> {
    let args = Args::from_args();

    let resp = TcpClient::connect(args.addr)
        .and_then(|mut client| client.send_request(&args.message))
        .map_err(|e| format!("Error sending request: {}", e))?;

    println!("{}", resp);
    Ok(())
}
