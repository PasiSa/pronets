/* Open TCP server socket, bind it to given address, and wait incoming connections.
 * Echo back all data received from the connection. The connections are kept open
 * until the client closes them, i.e., we must handle multiple simultaneous
 * connections at a time. Bind to "0.0.0.0:<port>" if connections are allowed
 * from any interface.
 * 
 * Usage: cargo run -- <IP>:<port>
 */

mod tokenmanager;  // This example also has tokenmanger module in separate file.

use std::{
    collections::HashMap,
    env,
    error::Error,
    io::{Read, Write},
    net::{SocketAddr},
};

use mio::{
    Events,
    Interest,
    Poll,
    net::{TcpListener, TcpStream},
    Token
};

use crate::tokenmanager::TokenManager;


// Small structure to hold information for each client currently being served by
// our server
struct Client {
    socket: TcpStream,
    address: SocketAddr,
}


fn main() -> Result<(), Box<dyn Error>> {
    // Collect command-line arguments into a vector
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("arguments: <host>:<port>");
        return Err("Invalid command".into());
    }

    // Create a passive server socket and bind to address given as command line argument.
    // If there is an error in bind, exit the main function with error.
    let addr = args[1].parse()?;
    let mut server = TcpListener::bind(addr)?;

    // Set up MIO event engine for handling concurrent I/O operations.
    let mut tokenmanager = TokenManager::new();  // our own little token manager
    let mut poll = Poll::new()?;  // MIO's Poll service
    let mut events = Events::with_capacity(128);  // Container for max 128 events at a time

    // Allocate a token for the listening socket and register it to MIO for
    // receiving events
    let listen_token = tokenmanager.allocate_token();
    poll.registry()
        .register(&mut server, listen_token, Interest::READABLE)?;

    // HashMap for currently active clients
    let mut clients: HashMap<Token, Client> = HashMap::new();

    loop {
        // Wait for the next MIO event. There may be multiple events returned,
        // if we are busy. The second parameter is for optional timeout for
        // situations when there are no events for a while.
        poll.poll(&mut events, None)?;

        // Process each event.
        for event in events.iter() {
            // If there is event for listening socket, there is a new connection
            // arriving. Process it using accept.
            if event.token() == listen_token {
                // Accept incoming connection
                let (socket, address) = server.accept()?;
                println!("Accepting connection from {}", address.to_string());

                // Create new client instance for our hashmap.
                let mut c = Client {
                    socket,
                    address,
                };
                let token = tokenmanager.allocate_token();  // create token for client

                // Tell MIO to deliver events whenever there
                // is something to read from the socket.
                poll.registry().register(&mut c.socket, token, Interest::READABLE)?;
                clients.insert(token, c);  // Add new client to hashmap
            }
            // true if the hashmap contains client with received event token
            // 'c' will contain the client structure instance from hashmap
            else if let Some(c) = clients.get_mut(&event.token()) {
                let mut buf: [u8; 160] = [0; 160];

                // Test whether read call returns Ok or Err result variant
                match c.socket.read(&mut buf) {
                    Ok(n) => {
                        // return value of 0 bytes means that socket is closed by
                        // the other end. Remove the token from tokenmanager,
                        // and the client from hashmap.
                        if n == 0 {
                            println!("Client {} closed connection", c.address.to_string());
                            tokenmanager.free_token(event.token());
                            clients.remove(&event.token());
                            continue;  // move to next event
                        }
                        println!("read {} bytes from client {}", n, c.address.to_string());

                        // Echo the same bytes back to client. If we wanted to be
                        // decently prepared for the write call to be blocked, we should
                        // register a writable event and place the data to be written
                        // into client-specific buffer.
                        if let Err(e) = c.socket.write(&buf[..n]) {
                            // if there is a error, remove client and close the socket
                            // without terminating the event loop
                            println!("Error writing to client: {}", e);
                            tokenmanager.free_token(event.token());
                            clients.remove(&event.token());
                        }
                    },
                    Err(e) => {
                        println!("Error reading from {}: {}",
                            c.address.to_string(), e);
                        tokenmanager.free_token(event.token());
                        clients.remove(&event.token());
                    }
                }
            }
        }
    }
}
