/* Simple example to illustrate effect of buffering on socket API
 * Server accepts connection, then waits for user input before starting to read.
 * Client just writes a large number of bytes to the server.
 * 
 * Server usage: cargo run -- -s <address:port>
 * Client usage: cargo run -- -c <address:port> -b <number of bytes>
 */

use std::{
    cmp::min,
    error::Error,
    io::{stdin, Read, Write},
    net::TcpStream, net::TcpListener,
};

use crate::args::Args;


fn client(args: &Args) -> Result<(), Box<dyn Error>> {
    // Connect TCP socket. If connection fails, exit the main function with error.
    // The connect function does both name resolution and connection establishment.
    let mut socket = TcpStream::connect(args.connect_addr())?;

    let mut total: usize = 0;

    // Just write the given number of bytes in 10000-byte chunks,
    // containing 'A' to the socket.
    // All errors cause the function to exit.
    while args.bytes() - total > 0 {
        let buffer: [u8; 10000] = [b'A'; 10000];
        let to_write = min(args.bytes() - total, 10000);
        let n = socket.write(&buffer[..to_write])?;
        total += n;
        println!("Wrote {} bytes, total written: {}", n, total);
    }

    Ok(())
}


fn server(args: &Args) -> Result<(), Box<dyn Error>> {
    let server = TcpListener::bind(args.server_addr())?;

    let (mut socket, address) = server.accept()?;
    println!("Accepted connection from {}. Press some key to start reading", address);

    let mut keypress = [0; 1];
    stdin().read_exact(&mut keypress)?;

    // Just read data in 10000-byte chunks until client closes the connection.
    // All errors cause function to exit.
    let mut total: usize = 0;
    loop {
        let mut buf: [u8; 10000] = [0; 10000];
        let n = socket.read(&mut buf)?;
        if n == 0 {
            println!("Connection closed, exiting.");
            break;
        }
        total += n;
        println!("Read {} bytes, total read: {}.", n, total);
    }

    Ok(())
}


fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments.
    let args = Args::new();

    if args.server_addr().len() > 0 {
        server(&args)?;
    } else {
        client(&args)?;
    }

    Ok(())  // Everything successful!
}

mod args;
