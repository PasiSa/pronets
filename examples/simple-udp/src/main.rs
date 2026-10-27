/* Demonstrates the use of UDP socket, by sending user input to given destination address.
 * Server echoes the datagram content back to client that prints it.
 *
 * Server usage: cargo run -- -s port
 * Client usage: cargo run -- -c <address:port>
 */

use clap::Parser;
use std::{error::Error, io::stdin, net::UdpSocket};

/// Command line arguments parser.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Start as client, give address to connect.
    #[arg(short, long, default_value = "239.0.0.1:20000")]
    connect_addr: String,

    /// Start as server, give address to listen for connections.
    #[arg(short, long, default_value_t = 0)]
    server_port: u16,
}

fn client(args: &Args) -> Result<(), Box<dyn Error>> {
    // Create UDP socket, bind to any address, pick a free UDP port
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input)?;

        // Send the line given by user to UDP socket, to given destination.
        // Because UDP is not connection-oriented, we use send_to function that
        // provides destination address in addition to data. Unlike with TCP, we
        // can use the same socket for sending to multiple destinations.
        if let Err(err) = socket.send_to(input.trim().as_bytes(), &args.connect_addr) {
            eprintln!("Failed to send datagram to {}: {}", args.connect_addr, err);
            continue;
        }

        let mut buf = [0; 1024];
        // Read response back from UDP socket. Blocks execution until datagram
        // arrives. Recv_from returns number of bytes read, and the address/port
        // of the sender.
        let (size, src) = match socket.recv_from(&mut buf) {
            Ok(result) => result,
            Err(err) => {
                eprintln!("Failed to receive datagram: {}", err);
                continue;
            }
        };
        let msg = String::from_utf8_lossy(&buf[..size]);
        println!("Received from {}: {}", src, msg);
    }
}

fn server(args: &Args) -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", args.server_port))?;

    loop {
        let mut buf = [0; 1024];
        // Block until datagram comes to the socket. Returns number of bytes read
        // and the address/port of the sender.
        // Convert the u8 array into UTF8 string.
        let (size, src) = match socket.recv_from(&mut buf) {
            Ok(result) => result,
            Err(err) => {
                eprintln!("Failed to receive datagram: {}", err);
                continue;
            }
        };
        let msg = String::from_utf8_lossy(&buf[..size]);
        println!("Received from {}: {}", src, msg);

        if let Err(err) = socket.send_to(&buf[..size], src) {
            eprintln!("Failed to send datagram to {}: {}", src, err);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments.
    let args = Args::parse();

    if args.server_port > 0 {
        server(&args)?;
    } else {
        client(&args)?;
    }

    Ok(()) // Everything successful!
}
