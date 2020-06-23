use std::io::{self, BufReader, BufWriter, Write};
use std::net::{SocketAddr, TcpStream};

use structopt::StructOpt;

use tcp_demo_lines::{LinesCodec, DEFAULT_SERVER_ADDR};

#[derive(Debug, StructOpt)]
#[structopt(name = "client")]
struct Args {
    message: String,
    /// Server destination address
    #[structopt(long, default_value = DEFAULT_SERVER_ADDR, global = true)]
    addr: SocketAddr,
}

fn main() -> io::Result<()> {
    let args = Args::from_args();

    let stream = TcpStream::connect(args.addr)?;

    // Codec is our interface for reading/writing messages.
    // No need to handle reading/writing directly
    let mut codec = LinesCodec::new(stream)?;

    codec.send_message(&args.message)?;
    println!("{}", codec.read_message()?);
    Ok(())
}
