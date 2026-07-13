/* Open TCP server socket, bind it to given address, and wait incoming connections.
 * For each incoming connection, a new async task is spawned to handle the connection.
 * For each connection, read incoming data and echo it back, until connection closes.
 * Bind to "0.0.0.0:<port>" if connections are allowed from any interface.
 * 
 * Usage: cargo run -- <IP>:<port>
 */

use std::{
    env,
    error::Error,
    net::SocketAddr,
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    // different from the std versions. These support async use
    net::{TcpListener, TcpStream},
    task,
};


// The below annotation sets up the tokio runtime for task scheduling with async/await.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Collect command-line arguments into a vector
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("arguments: <host>:<port>");
        return Err("Invalid command".into());
    }

    // Create a passive server socket and bind to address given as command line argument.
    // If there is an error in bind, exit the main function with error.
    // Note that tokio version of bind uses await to allow scheduling of other tasks.
    let server = TcpListener::bind(&args[1]).await?;

    loop {
        // Wait until new connection request comes in.
        // accept returns active socket and address of the connecting host as tuple.
        // tokio await prevents blocking here
        let (socket, address) = server.accept().await?;
        println!("Accepting connection from {}", address.to_string());

        // Spawn a new tokio task to handle communication with the client.
        task::spawn(async move {
            process_client(socket, address).await;
        });
    }
}


async fn process_client(mut socket: TcpStream, address: SocketAddr) {
    loop {
        let mut buf: [u8; 160] = [0; 160];
        // Test whether read call returns Ok or Err result variant
        match socket.read(&mut buf).await {
            Ok(n) => {
                // return value of 0 bytes means that socket is closed by
                // the other end.
                if n == 0 {
                    println!("Client {} closed connection", address.to_string());
                    break;  // exit the function and terminate the thread
                }
                println!("read {} bytes from client {}", n, address.to_string());

                // Echo the same bytes back to client. 
                if let Err(e) = socket.write(&buf[..n]).await {
                    // If there is an error, terminate task
                    println!("Error writing to client: {}", e);
                    break;
                }
            },
            Err(e) => {
                println!("Error reading from {}: {}",
                    address.to_string(), e);
                break;  // terminating task
            }
        }
    }
}