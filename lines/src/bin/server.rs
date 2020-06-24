use std::io;
use std::net::{SocketAddr, TcpListener, TcpStream};

use structopt::StructOpt;

use tcp_demo_lines::{LinesCodec, DEFAULT_SERVER_ADDR};

#[derive(Debug, StructOpt)]
#[structopt(name = "server")]
struct Args {
    /// Service listening address
    #[structopt(long, default_value = DEFAULT_SERVER_ADDR, global = true)]
    addr: SocketAddr,
}

/// Given a TcpStream:
/// - Deserialize the message
/// - Serialize and write the echo message to the stream
fn handle_connection(stream: TcpStream) -> io::Result<()> {
    let peer_addr = stream.peer_addr().expect("Stream has peer_addr");
    eprintln!("Incoming from {}", peer_addr);
    let mut codec = LinesCodec::new(stream)?;

    let message: String = codec
        .read_message()
        // Reverse message
        .map(|m| m.chars().rev().collect())?;
    codec.send_message(&message)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let args = Args::from_args();
    eprintln!("Starting server on '{}'", args.addr);

    let listener = TcpListener::bind(args.addr)?;
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            std::thread::spawn(move || {
                handle_connection(stream).map_err(|e| eprintln!("Error: {}", e))
            });
        }
    }
    Ok(())
}
