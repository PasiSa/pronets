/* Open TCP server socket, bind it to given address, and wait incoming connections.
 * For each incoming connection, a new thread is spawned to handle the connection.
 * For each connection, read incoming data and echo it back, until connection closes.
 * Bind to "0.0.0.0:<port>" if connections are allowed from any interface.
 * 
 * Usage: cargo run -- <IP>:<port>
 */

use std::{
    env,
    error::Error,
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    thread,
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
        let (socket, address) = server.accept()?;
        println!("Accepting connection from {}", address.to_string());

        // Spawn a new thread to handle all communication with the client.
        thread::spawn(move || process_client(socket, address));
    }
}


fn process_client(mut socket: TcpStream, address: SocketAddr) {
    loop {
        let mut buf: [u8; 160] = [0; 160];
        // Test whether read call returns Ok or Err result variant
        match socket.read(&mut buf) {
            Ok(n) => {
                // return value of 0 bytes means that socket is closed by
                // the other end.
                if n == 0 {
                    println!("Client {} closed connection", address.to_string());
                    break;  // exit the function and terminate the thread
                }
                println!("read {} bytes from client {}", n, address.to_string());

                // Echo the same bytes back to client. 
                if let Err(e) = socket.write(&buf[..n]) {
                    // If there is an error, terminate thread
                    println!("Error writing to client: {}", e);
                    break;
                }
            },
            Err(e) => {
                println!("Error reading from {}: {}",
                    address.to_string(), e);
                break;  // bye bye, thread!
            }
        }
    }
}