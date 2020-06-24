use std::io::{self, BufReader, BufWriter};
use std::net::{SocketAddr, TcpListener, TcpStream};

use structopt::StructOpt;

use tcp_demo_raw::{extract_string_buffered, write_data, DEFAULT_SERVER_ADDR};

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
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = BufWriter::new(stream);

    let message = extract_string_buffered(&mut reader)?;
    write_data(&mut writer, &message.as_bytes())
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
