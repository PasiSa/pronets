mod comms;
mod ui;

use clap::Parser;
use slint::ComponentHandle;
use std::io::{self};

use comms::start_socket_io;
use ui::{append_to_conversation, AppWindow};

#[derive(Parser, Debug)]
#[command(author, version, about = "Demo TCP Chat Client", long_about = None)]
struct Args {
    /// Server host to connect to
    #[arg(short, long, default_value = "127.0.0.1")]
    addr: String,

    /// Server port to connect to
    #[arg(short, long, default_value_t = 1234)]
    port: u16,

    /// Username to use in the chat
    #[arg(short, long, default_value = "User")]
    user: String,
}

fn main() -> io::Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    let address = format!("{}:{}", args.addr, args.port);

    println!("Connecting to {}...", address);

    // Create the main application window, based on the Slint UI definition in ui.slint
    let window =
        AppWindow::new().map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))?;
    let weak_window = window.as_weak();

    // Start separate thread to handle socket communication.
    // The closure passed to start_socket_io will be called whenever a
    // new message is received from the server.
    //
    // Messages cross the UI/socket thread boundary through channels: UI callbacks
    // send bytes through the returned `outbound` sender to the socket thread, while
    // received messages enter this callback and are queued onto Slint's event loop
    // so that conversation state is updated only from the UI thread.
    // message variable contains the received message inside the closure,
    // and is passed to append_to_conversation to update the UI.
    let socket_weak_window = weak_window.clone();
    let outbound = start_socket_io(address, move |message| {
        let weak_window = socket_weak_window.clone();
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(window) = weak_window.upgrade() {
                append_to_conversation(&window, &message);
            }
        });
    });
    // outbound is the channel sender to send messages from the UI thread to the socket thread.
    // Need to clone it for use in the UI callbacks below, since the closure will be called multiple times.
    let message_outbound = outbound.clone();

    // Handle the send message event when user has typed message
    window.on_send_message(move |message| {
        let message = message.trim();
        if message.is_empty() {
            return;
        }

        let outmsg = "placeholder message".as_bytes().to_vec(); // TODO: replace with actual message serialization
        if message_outbound.send(outmsg).is_err() {
            eprintln!("Failed to send message");
            return;
        }

        // Append the sent message to the conversation area in the UI
        if let Some(window) = weak_window.upgrade() {
            append_to_conversation(&window, message);
        }
    });

    // Handle the send test event when user clicks the test button
    window.on_send_test(move || {
        if outbound.send(b"TST button clicked".to_vec()).is_err() {
            eprintln!("Failed to send test message");
        }
    });

    // Run the UI event loop until the application window is closed.
    window
        .run()
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))?;

    Ok(())
}
