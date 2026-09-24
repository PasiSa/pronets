use std::{
    io::{self, Read},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

// Integration tests import the library through its public API.
use integration_test::{
    protocol::{read_message, write_message},
    server::handle_connection,
};

fn run_server(listener: TcpListener, timeout: Duration) -> io::Result<()> {
    let (stream, _) = listener.accept()?;
    stream.set_read_timeout(Some(timeout))?;
    stream.set_write_timeout(Some(timeout))?;
    handle_connection(stream)
}

fn run_client(mut client: TcpStream) -> io::Result<(Vec<u8>, usize)> {
    write_message(&mut client, b"hello")?;
    let response = read_message(&mut client)?;

    // The server handles just one message and then closes its socket.
    let mut byte = [0];
    let remaining = client.read(&mut byte)?;
    Ok((response, remaining))
}

#[test]
fn echoes_message_over_tcp() -> io::Result<()> {
    let timeout = Duration::from_secs(5);

    // When binding to port 0, the system will assign a local port.
    // Therefore we need to ask it separately.
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let address = listener.local_addr()?;

    // TCP can queue a connection before accept is called. Connect first so a
    // failed connection attempt cannot leave a server thread waiting in accept.
    // connect_timeout() is a variant of connect that associates a timeout with
    // the connection attempt, used to avoid blocking forever if the server
    // thread crashes.
    let client = TcpStream::connect_timeout(&address, timeout)?;

    // Also read and write operations can be equipped with a timeout,
    // to avoid blocking the test indefinitely.
    client.set_read_timeout(Some(timeout))?;
    client.set_write_timeout(Some(timeout))?;

    // Start a separate thread for server that runs the run_server function
    // in its closure.
    // "move" transfers the ownership of listener and timeout into the new thread.
    let server = thread::spawn(move || run_server(listener, timeout));

    // Client side is run in the main thread.
    // Save the client result so that we join the server even if an exchange fails.
    let client_result = run_client(client);

    // Wait for the server thread to finish and if there was error,
    // propagate it to the main thread.
    server.join().expect("server thread panicked")?;
    let (response, remaining) = client_result?;
    assert_eq!(response, b"hello");

    // The server should close the connection after sending its response.
    assert_eq!(remaining, 0, "server should close after its response");
    Ok(())
}
