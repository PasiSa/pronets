use std::{io, net::TcpStream};

use crate::protocol::{read_message, write_message};

/// Echo one message and close the connection when the stream goes out of scope.
pub fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let payload = read_message(&mut stream)?;
    write_message(&mut stream, &payload)
}
