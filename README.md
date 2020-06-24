# Ready & Writing data with Rust's TcpStream

This repo contains examples of using Rust's [TcpStream](https://doc.rust-lang.org/stable/std/net/struct.TcpStream.html) to send & receive data between a client and server.
This example shows low-level data (raw bytes) reading & writing with Rust's [TcpStream](https://doc.rust-lang.org/stable/std/net/struct.TcpStream.html).  Subsequent examples add abstractions over this, but it's helpful to understand what's happening under the hood and why abstractions make things easier.

## Raw TCP Bytes
See how the [Read](https://doc.rust-lang.org/stable/std/io/trait.Read.html) and [Write](https://doc.rust-lang.org/stable/std/io/trait.Write.html) traits work with low-level TcpStream Tx/Rx

[Get Started Here](./raw)

## Line-based Codec
Step up a level of abstraction using line-based messaging (newline delimited) and how the [BufRead](https://doc.rust-lang.org/stable/std/io/trait.BufRead.html) and [BufWrite](https://doc.rust-lang.org/stable/std/io/trait.BufWrite.html) traits can be more effecient

[Get Started Here](./lines)

## Message Protocol
If we want to send more than just lines, we can abstract even further into a protocol of structs, handling serialization & deserialization in the background.

[Get Started Here](./protocol)
