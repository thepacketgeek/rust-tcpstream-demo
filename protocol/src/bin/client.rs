use std::io::{self, BufReader, BufWriter, Write};
use std::net::{SocketAddr, TcpStream};

use structopt::StructOpt;

use tcp_demo_protocol::{Request, Response, DEFAULT_SERVER_ADDR};

#[derive(Debug, StructOpt)]
#[structopt(name = "client")]
struct Args {
    message: String,
    // Jumble the message by how much (default = will not jumble)
    #[structopt(short, long, default_value = "0")]
    jumble: u16,
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
    pub fn send_request(&mut self, req: &Request) -> io::Result<Response> {
        req.serialize(&mut self.writer)?;
        self.writer.flush()?;
        Response::deserialize(&mut self.reader)
    }
}

fn main() -> Result<(), String> {
    let args = Args::from_args();

    let req = if args.jumble > 0 {
        Request::Jumble {
            message: args.message,
            amount: args.jumble,
        }
    } else {
        Request::Echo(args.message)
    };

    let resp = TcpClient::connect(args.addr)
        .and_then(|mut client| client.send_request(&req))
        .map_err(|e| format!("Error sending request: {}", e))?;

    println!("{}", resp.message());
    Ok(())
}
