use std::io;
use std::thread;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

/// Start a separate thread to handle socket communication with the server.
/// The closure function passed to start_socket_io (in main.rs) will be called
/// whenever a new message is received from the server.
pub(crate) fn start_socket_io<F>(address: String, on_message: F) -> UnboundedSender<Vec<u8>>
where
    F: Fn(String) + Send + 'static,
{
    let (outbound, receiver) = mpsc::unbounded_channel();
    thread::spawn(move || {
        // This background OS thread owns a single-threaded Tokio runtime. `block_on`
        // drives `run_socket_io` and its asynchronous TCP/channel operations until
        // the connection ends or an error occurs, without blocking the UI thread.
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .build()
            .expect("failed to create socket runtime");
        if let Err(err) = runtime.block_on(run_socket_io(&address, receiver, &on_message)) {
            on_message(format!("Socket error: {err}"));
        }
    });
    outbound
}

async fn run_socket_io<F>(
    address: &str,
    mut outbound: UnboundedReceiver<Vec<u8>>,
    on_message: &F,
) -> io::Result<()>
where
    F: Fn(String),
{
    let stream = TcpStream::connect(address).await?;

    // Split the TCP stream into independent read and write halves so `select!`
    // can wait for incoming data while also sending messages from the UI channel,
    // without either operation blocking the other.
    let (mut reader, _writer) = tokio::io::split(stream);
    let mut incoming = Vec::new();
    let mut read_buffer = [0_u8; 8192];

    loop {
        tokio::select! {
            message_out = outbound.recv() => {
                // New message to send to the server, delivered via mpsc channel
                // between the UI thread and this socket thread.
                // If the channel is closed, the UI thread has exited and we can exit too
                let Some(_message_out) = message_out else { return Ok(()); };

                // TODO: write_message(...message_out, some other params...)
            }
            result = reader.read(&mut read_buffer) => {
                // New data received from the server, read from the TCP stream
                let read = result?;
                if read == 0 {
                    on_message("Connection closed".to_string());
                    return Ok(());
                }
                incoming.extend_from_slice(&read_buffer[..read]);
                // TODO: parse incoming data into messages
                // just a placeholder here
                on_message(format!("Received {} bytes from socket", read));
            }
        }
    }
}
