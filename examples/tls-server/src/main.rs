/* Open a TLS server using Tokio and echo data from each client.
 *
 * Usage: cargo run -- <IP>:<port> <certificate.pem> <private-key.pem>
 * 
 * You can use files cert/server.crt and cert/server.key for these. They are
 * created with the ca.crt available at tls-client example.
 */

use std::{
    env,
    error::Error,
    fs::File,
    io::{self, BufReader},
    net::SocketAddr,
    sync::Arc,
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use tokio_rustls::{rustls::ServerConfig, server::TlsStream, TlsAcceptor};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Read the listening address, certificate, and private-key paths.
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("arguments: <host>:<port> <certificate.pem> <private-key.pem>");
        return Err("Invalid command".into());
    }

    // A certificate is public and identifies this server to connecting clients.
    // A PEM file may contain the server certificate followed by intermediate ones.
    let certificates = rustls_pemfile::certs(&mut BufReader::new(File::open(&args[2])?))
        .collect::<Result<Vec<_>, _>>()?;

    // The private key proves that the server owns the certificate. It must be
    // kept secret and must match the public key in the server certificate.
    let private_key = rustls_pemfile::private_key(&mut BufReader::new(File::open(&args[3])?))?
        .ok_or("private key not found")?;

    // Configure rustls with the server identity. This example authenticates
    // only the server, so clients do not need to provide certificates.
    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certificates, private_key)?;

    // TlsAcceptor turns accepted TCP connections into encrypted TLS streams.
    let tls_acceptor = TlsAcceptor::from(Arc::new(config));

    // TLS runs on top of TCP, so the server still starts with a TCP listener.
    let server = TcpListener::bind(&args[1]).await?;
    println!("Listening on {}", args[1]);

    loop {
        // Accepting here establishes TCP only; the TLS handshake happens below.
        let (socket, address) = server.accept().await?;

        // Each task needs its own handle to the shared TLS configuration.
        let tls_acceptor = tls_acceptor.clone();

        // Handle clients concurrently so one slow client does not block others.
        tokio::spawn(async move {
            let result = async {
                // Perform the TLS handshake and verify that the client and
                // server agree on supported TLS parameters.
                let socket = tls_acceptor.accept(socket).await?;
                println!("Accepted TLS connection from {address}");
                process_client(socket, address).await
            }
            .await;

            if let Err(error) = result {
                eprintln!("Connection to {address} failed: {error}");
            }
        });
    }
}

async fn process_client(mut socket: TlsStream<TcpStream>, address: SocketAddr) -> io::Result<()> {
    let mut buf = [0; 10000];

    loop {
        // TlsStream implements the normal async read/write interfaces. rustls
        // transparently decrypts incoming bytes and authenticates each record.
        let n = socket.read(&mut buf).await?;
        if n == 0 {
            println!("Client {address} closed connection");
            return Ok(());
        }

        println!("Read {n} bytes from client {address}");

        // write_all gives plaintext to rustls, which encrypts it before sending
        // it through the underlying TCP connection.
        socket.write_all(&buf[..n]).await?;
    }
}
