---
---

# List of examples

This folder contains Rust network programming examples referred to in the course
material, each in separate Rust project folder. They contain Cargo.toml, etc.
required files so that you can build and test them using `cargo build` and
`cargo run`.

One easy way to try the examples is that you clone the course repository to your
machine and locate this directory. The examples contain Cargo.toml, etc.
required files so that you can build and test them using `cargo build` and
`cargo run`. Feel free to modify the examples as you want, to try different
things.

The examples are as follows:

- **[simple-client](https://github.com/PasiSa/pronets/tree/main/examples/simple-client/src/main.rs)**:
  Opens connection, then writes and reads a bit of data.

- **[tcpheader](https://github.com/PasiSa/pronets/tree/main/examples/tcpheader/src/main.rs)**
  Example of converting a struct consisting TCP header fields into byte stream
  that can be written to a socket, and conversely, filling the struct from data
  read from byte stream.

- **[send-much](https://github.com/PasiSa/pronets/tree/main/examples/send-much/src/main.rs)**:
  Contains both simple server and client implementation: server accepts
  connections, then waits for user input and reads all data someone sends to the
  socket, until the socket is closed. Client just sends the requested number of
  bytes. Used to demonstrate the effect of socket buffering on socket API
  behavior.

- **[simple-server](https://github.com/PasiSa/pronets/tree/main/examples/simple-server/src/main.rs)**:
  Accepts a connection, then reads data from socket and writes some data back,
  then closes the connection. Handles only one connection at a time in a loop.
  Can be tested together with
  [simple-client](https://github.com/PasiSa/pronets/tree/main/examples/simple-client/src/main.rs).

- **[iterative-server](https://github.com/PasiSa/pronets/tree/main/examples/iterative-server/src/main.rs)**:
  Accepts incoming connections, then waits for incoming data and echoes it back
  in all active connections. Keeps connections open until other end closes them.
  Can handle multiple connections in parallel. Demonstrates non-blocking sockets
  in an iterative single-threaded server using Rust's **[mio
  crate](https://crates.io/crates/mio)**.
