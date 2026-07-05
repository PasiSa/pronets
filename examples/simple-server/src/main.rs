/* Open TCP server socket, bind it to given address, and wait incoming connections.
 * Handle incoming connections one at the time: read some data from socket,
 * and echo it back. Bind to "0.0.0.0:<port>" if connections are allowed from any
 * interface.
 * 
 * Usage: cargo run -- <IP>:<port>
 */

use std::{
    env,
    error::Error,
    io::{Read, Write},
    net::TcpListener,
};


fn main() -> Result<(), Box<dyn Error>> {
    // Collect command-line arguments into a vector
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("arguments: <host>:<port>");
        return Err("Invalid command".into());
    }

    // Create a passive server socket and bind to address given as command line argument.
    // If there is an error in bind, exit the main function with error.
    let server = TcpListener::bind(&args[1])?;

    loop {
        // Wait until new connection request comes in.
        // accept returns active socket and address of the connecting host as tuple.
        let (mut socket, address) = server.accept()?;
        println!("Accepting connection from {}", address.to_string());

        // Read at most 160 bytes from the established connection
        // 'readn' will contain the number of bytes actually read.
        // If read fails, the error causes main function to exit. 
        let mut buf: [u8; 160] = [0; 160];
        let readn = socket.read(&mut buf)?;

        // Write the bytes that were read back to the client.
        // 'writen' will contain the number of bytes actually written.
        // If function fails, the error causes main function to exit.
        let writen = socket.write(&buf[..readn])?;
        println!("Wrote {} bytes", writen);

        // Client socket is implicitly closed as 'socket' goes out of scope
        // at the end of loop. We are ready to accept the next connection.
    }
}
