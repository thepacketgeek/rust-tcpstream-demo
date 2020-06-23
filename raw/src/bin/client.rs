use std::io::{self, Write};
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

/// Send data (bytes) to the server
pub fn write_data(stream: &mut TcpStream, data: &[u8]) -> io::Result<()> {
    // Here, `write_all()` attempts to write the entire slice, raising an error if it cannot do so
    stream.write_all(data)?;

    // An alternative is `write()` which will return the number of bytes that *could*
    // be sent. This can be used if your app has a mechanism to handle this scenario.
    // E.g. TCP backpressure for high-bandwidth data
    //
    // This is an example of what `write_all()` does:
    // let bytes_to_write = data.len();
    // let bytes_written = stream.write(data)?;
    // if bytes_written < bytes_written {
    //     return Err(Error::new(ErrorKind::Interrupted, "Could not write all data"));
    // }

    // Signal that we're done writing and the data should be sent (with TCP PSH bit)
    stream.flush()
}

fn main() -> io::Result<()> {
    let args = Args::from_args();

    let mut stream = TcpStream::connect(args.addr)?;
    write_data(&mut stream, &args.message.as_bytes())?;

    // Now read & print the response
    // (this will block until all data has been received)
    extract_string_unbuffered(&mut stream).map(|resp| println!("{}", resp))
}
