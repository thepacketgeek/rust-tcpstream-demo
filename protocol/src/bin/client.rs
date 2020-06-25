use std::io;
use std::net::SocketAddr;

use structopt::StructOpt;

use tcp_demo_protocol::{Protocol, Request, Response, DEFAULT_SERVER_ADDR};

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

fn main() -> io::Result<()> {
    let args = Args::from_args();

    let req = if args.jumble > 0 {
        Request::Jumble {
            message: args.message,
            amount: args.jumble,
        }
    } else {
        Request::Echo(args.message)
    };

    Protocol::connect(args.addr)
        .and_then(|mut client| {
            client.send_message(&req)?;
            Ok(client)
        })
        .and_then(|mut client| client.read_message::<Response>())
        .map(|resp| println!("{}", resp.message()))
}
