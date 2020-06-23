use std::io::{self, BufReader, BufWriter, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

use structopt::StructOpt;

use tcp_demo_protocol::{Request, Response, DEFAULT_SERVER_ADDR};

#[derive(Debug, StructOpt)]
#[structopt(name = "server")]
struct Args {
    /// Service listening address
    #[structopt(long, default_value = DEFAULT_SERVER_ADDR, global = true)]
    addr: SocketAddr,
}

/// Given a TcpStream:
/// - Deserialize the request
/// - Handle the request
/// - Serialize and write the Response to the stream
fn handle_connection(stream: TcpStream) -> io::Result<()> {
    let peer_addr = stream.peer_addr().expect("Stream has peer_addr");
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = BufWriter::new(stream);

    let request = Request::deserialize(&mut reader)?;
    eprintln!("Incoming {:?} [{}]", request, peer_addr,);
    let resp = match request {
        Request::Echo(message) => Response(format!("'{}' from the other side!", message)),
        Request::Jumble { message, amount } => Response(jumble_message(&message, amount)),
    };

    resp.serialize(&mut writer)?;
    writer.flush()
}

/// Shake the characters around a little bit
fn jumble_message(message: &str, amount: u16) -> String {
    let mut chars: Vec<char> = message.chars().collect();
    // Do some jumbling
    for i in 1..=amount as usize {
        let shuffle = i % chars.len();
        chars.swap(0, shuffle);
    }
    chars.into_iter().collect()
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
