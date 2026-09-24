//! Messages consist of a four-byte, big-endian length followed by the payload.

use std::io::{self, Read, Write};

// Keep the other end from requesting an arbitrarily large allocation.
const MAX_MESSAGE_SIZE: usize = 64 * 1024;

/// Read one complete message.
pub fn read_message(reader: &mut impl Read) -> io::Result<Vec<u8>> {
    let mut header = [0; 4];
    reader.read_exact(&mut header)?;
    let length = u32::from_be_bytes(header) as usize;
    if length > MAX_MESSAGE_SIZE {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "message exceeds 64 KiB",
        ));
    }

    let mut payload = vec![0; length];
    reader.read_exact(&mut payload)?;
    Ok(payload)
}

/// Write a complete message, including its length header.
pub fn write_message(writer: &mut impl Write, payload: &[u8]) -> io::Result<()> {
    if payload.len() > MAX_MESSAGE_SIZE {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "message exceeds 64 KiB",
        ));
    }

    writer.write_all(&(payload.len() as u32).to_be_bytes())?;
    writer.write_all(payload)
}
