use std::sync::mpsc::{self, Receiver, RecvTimeoutError, SyncSender};
use std::thread;
use std::time::Duration;

// Only plain game data crosses threads; textures and drawing stay on the main thread.
#[derive(Clone, Copy)]
pub struct CarState {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
}

pub struct RemoteCarUpdate {
    pub player_id: u64,
    pub state: CarState,
}

pub fn start() -> (SyncSender<CarState>, Receiver<RemoteCarUpdate>) {
    // sync_channel has limited capacity for the message queue, to keep a slow
    // network from accumulating unlimited frame updates. Therefore it works for
    // real-time game state updates, where only the most recent state is
    // relevant.
    let (local_tx, local_rx) = mpsc::sync_channel::<CarState>(1);
    let (remote_tx, remote_rx) = mpsc::sync_channel::<RemoteCarUpdate>(32);

    thread::spawn(move || {
        // TODO: Open a socket and connect to the server here.
        loop {
            // read the latest local car state from the game thread, if available.
            match local_rx.recv_timeout(Duration::from_millis(50)) {
                Ok(_state) => {
                    // TODO: Serialize the local car state and send it through the socket.
                }
                Err(RecvTimeoutError::Timeout) => {}
                // The game dropped its sender: stop the communication thread.
                Err(RecvTimeoutError::Disconnected) => break,
            }

            // TODO: Read available socket messages and decode remote car updates.
            // Ensure that the worker thread never blocks on network I/O.
            // For example, you can use set_read_timeout on the socket, mio,
            // or async I/O with tokio.
            let received_update: Option<RemoteCarUpdate> = None;
            if let Some(update) = received_update {
                // Send remote car data to the game thread.
                // Skip updates when the queue is full instead of blocking the worker.
                match remote_tx.try_send(update) {
                    Ok(()) | Err(mpsc::TrySendError::Full(_)) => {}
                    Err(mpsc::TrySendError::Disconnected(_)) => break,
                }
            }
        }
        // TODO: Close the socket / notify the server on shutdown if needed.
    });

    (local_tx, remote_rx)
}
