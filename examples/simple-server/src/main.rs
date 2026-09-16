/* Open TCP server socket, bind it to given address, and wait incoming connections.
 * Handle incoming connections one at the time: read some data from socket,
 * and echo it back. Bind to "0.0.0.0:<port>" if connections are allowed from any
 * interface.
 *
 * Usage: cargo run -- <IP>:<port>
 */

use std::{
    env,
    io::{self, ErrorKind, Read, Write},
    net::TcpListener,
};

fn main() -> io::Result<()> {
    // Collect command-line arguments into a vector
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("arguments: <host>:<port>");
        return Err(io::Error::new(ErrorKind::InvalidInput, "Invalid command"));
    }

    // Create a passive server socket and bind to address given as command line argument.
    // If there is an error in bind, exit the main function with error.
    let server = TcpListener::bind(&args[1])?;

    loop {
        // Wait until new connection request comes in.
        // accept returns active socket and address of the connecting host as tuple.
        // Accept may fail for different reasons: if client aborted connection,
        // we can continue.
        // Other kinds of errors may be more persistent (out of file descriptors, etc.),
        // so then we exit the main function with error.
        let (mut socket, address) = match server.accept() {
            Ok((socket, address)) => (socket, address),
            Err(e) if e.kind() == ErrorKind::ConnectionAborted => {
                eprintln!("Client aborted connection: {e}");
                continue;
            }
            Err(e) => return Err(e),
        };
        println!("Accepting connection from {}", address.to_string());

        // Read at most 160 bytes from the established connection
        // 'readn' will contain the number of bytes actually read.
        // If the client closes or aborts the connection, accept the next client.
        // Other read errors cause main to exit.
        let mut buf: [u8; 160] = [0; 160];
        let readn = match socket.read(&mut buf) {
            Ok(readn) => {
                if readn == 0 {
                    println!("Client closed connection");
                    continue;
                }
                readn
            }
            Err(e) => {
                eprintln!("Error reading socket: {e}");
                continue;
            }
        };

        // Write the bytes that were read back to the client.
        // 'writen' will contain the number of bytes actually written.
        // If function fails, the error causes main function to exit.
        // Write only as many bytes as were read, because the buffer may not be full.
        // For output, we convert the bytes to a string using from_utf8_lossy,
        // which will replace invalid UTF-8 sequences with the replacement character.
        println!(
            "Read {} bytes: {}",
            readn,
            String::from_utf8_lossy(&buf[..readn])
        );
        let writen = match socket.write(&buf[..readn]) {
            Ok(writen) => writen,
            Err(e) => {
                eprintln!("Write error: {e}");
                continue;
            }
        };
        println!("Wrote {} bytes", writen);

        // Client socket is implicitly closed as 'socket' goes out of scope
        // at the end of loop. We are ready to accept the next connection.
    }
}
