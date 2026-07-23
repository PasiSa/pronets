/* Open a TLS connection, send some data and receive data back.
 *
 * Usage: cargo run -- <name/address>:<port> <string> [root-certificate.pem]
 *
 * For public service with public CA - authored cert try, for example:
 * cargo run -- www.aalto.fi:443 $'GET /fi HTTP/1.1\r\nHost: www.aalto.fi\r\nConnection: close\r\n\r\n'
 * 
 * For local test (server name "localhost") you need to use the local CA, e.g.:
 * cargo run -- localhost:6000 HelloHello cert/ca.crt
 * (this works with tls-server)
 */

use rustls::{pki_types::ServerName, ClientConfig, ClientConnection, RootCertStore, StreamOwned};
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufReader, Read, Write},
    net::TcpStream,
    sync::Arc,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Collect command-line arguments into a vector
    let args: Vec<String> = env::args().collect();

    if !(3..=4).contains(&args.len()) {
        eprintln!("arguments: <host>:<port> <message> [root-certificate.pem]");
        return Err("Invalid command".into());
    }

    // TLS verifies that the certificate is valid for this server name.
    // Separate the server name from command argument.
    let host = args[1]
        .rsplit_once(':')
        .ok_or("address must be in the form <host>:<port>")?
        .0
        .trim_matches(['[', ']']);
    let server_name = ServerName::try_from(host.to_owned())?;

    // If a root certificate was given, trust certificates signed by that root.
    // This is useful with a private certificate authority or a local test server.
    // Otherwise, trust the normal public roots bundled by webpki-roots.
    let roots = if let Some(path) = args.get(3) {
        let mut roots = RootCertStore::empty();
        let mut certificate_file = BufReader::new(File::open(path)?);
        let certificates = rustls_pemfile::certs(&mut certificate_file);

        for certificate in certificates {
            roots.add(certificate?)?;
        }

        if roots.is_empty() {
            return Err("root certificate file contained no certificates".into());
        }

        roots
    } else {
        RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned())
    };

    let config = ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();

    // Wrap the TCP socket in a rustls stream. Reads and writes now use TLS.
    let tcp = TcpStream::connect(&args[1])?;
    let tls = ClientConnection::new(Arc::new(config), server_name)?;
    let mut socket = StreamOwned::new(tls, tcp);

    socket.write_all(args[2].as_bytes())?;

    let mut buf: [u8; 10000] = [0; 10000];
    let n = socket.read(&mut buf)?;
    println!("Read {} bytes: {}", n, String::from_utf8_lossy(&buf[..n]));

    // Gracefully close TLS before the underlying TCP socket is dropped.
    socket.conn.send_close_notify();
    socket.flush()?;

    Ok(())
}
