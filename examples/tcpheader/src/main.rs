
#[derive(Debug)]  // To implement printing of the struct content
struct TcpHeader {
    source_port: u16,       // Source Port
    dest_port: u16,         // Destination Port
    seq_number: u32,        // Sequence Number
    ack_number: u32,        // Acknowledgment Number
    data_offset: u8,        // Data offset (upper 4 bits) + reserved bits
    flags: u8,              // Flags (control bits)
    window_size: u16,       // Window Size
    checksum: u16,          // Checksum
    urgent_pointer: u16,    // Urgent Pointer
}

impl TcpHeader {
    /// Serialize the TCP header into a byte array
    fn to_bytes(&self) -> [u8; 20] {
        let mut bytes = [0u8; 20];
        
        // Convert each field to network byte order and place it into the array
        bytes[0..2].copy_from_slice(&self.source_port.to_be_bytes());
        bytes[2..4].copy_from_slice(&self.dest_port.to_be_bytes());
        bytes[4..8].copy_from_slice(&self.seq_number.to_be_bytes());
        bytes[8..12].copy_from_slice(&self.ack_number.to_be_bytes());
        bytes[12] = (self.data_offset << 4) | (self.flags >> 4); // Combine data offset and part of flags
        bytes[13] = self.flags & 0b00001111;                     // Lower 4 bits of flags
        bytes[14..16].copy_from_slice(&self.window_size.to_be_bytes());
        bytes[16..18].copy_from_slice(&self.checksum.to_be_bytes());
        bytes[18..20].copy_from_slice(&self.urgent_pointer.to_be_bytes());

        bytes
    }

    /// Deserialize a byte slice into a `TcpHeader` with byte order conversion
    fn from_bytes(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), 20, "Incorrect byte slice length");

        TcpHeader {
            source_port: u16::from_be_bytes(bytes[0..2].try_into().unwrap()),
            dest_port: u16::from_be_bytes(bytes[2..4].try_into().unwrap()),
            seq_number: u32::from_be_bytes(bytes[4..8].try_into().unwrap()),
            ack_number: u32::from_be_bytes(bytes[8..12].try_into().unwrap()),
            data_offset: bytes[12] >> 4, // Upper 4 bits for data offset
            flags: bytes[13],           // Flags are a single byte
            window_size: u16::from_be_bytes(bytes[14..16].try_into().unwrap()),
            checksum: u16::from_be_bytes(bytes[16..18].try_into().unwrap()),
            urgent_pointer: u16::from_be_bytes(bytes[18..20].try_into().unwrap()),
        }
    }
}

fn main() -> std::io::Result<()> {
    // Example TCP header
    let tcp_header = TcpHeader {
        source_port: 12345,
        dest_port: 80,
        seq_number: 123456789,
        ack_number: 987654321,
        data_offset: 5, // Minimum size of TCP header (20 bytes)
        flags: 0b00010100, // ACK + PSH flags
        window_size: 4096,
        checksum: 0xFFFF, // Placeholder checksum
        urgent_pointer: 0,
    };

    // Serialize the TCP header into [u8] array that can be written to network
    let serialized = tcp_header.to_bytes();

    // Print serialized header as bytes
    println!("Serialized TCP Header: {:?}", serialized);

    // read [u8] byte stream from network and convert to TcpHeader
    let from_network = TcpHeader::from_bytes(&serialized);

    println!("TCP header: {:?}", from_network);

    Ok(())
}
