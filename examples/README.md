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

## Basic socket operations

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

## Project template

- **[project-template](https://github.com/PasiSa/pronets/tree/main/examples/project-template)**:
  An example project template of a Rust workspace divided into two Rust packages
  for client and server applications. It also shows how the Slint UI framework
  is used for window-based client GUI.

## Different server variants

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

- **[threaded-server](https://github.com/PasiSa/pronets/tree/main/examples/threaded-server/src/main.rs)**:
  Similar to
  [iterative-server](https://github.com/PasiSa/pronets/tree/main/examples/iterative-server/src/main.rs),
  but spawns a new thread for each active client.

- **[async-server](https://github.com/PasiSa/pronets/tree/main/examples/async-server/src/main.rs)**:
  Similar to
  [threaded-server](https://github.com/PasiSa/pronets/tree/main/examples/threaded-server/src/main.rs)
  but applies asynchronous operations using Rust's **[tokio
  crate](https://crates.io/crates/tokio)**.

## TLS

There are client and server examples for testing Transport Layer Security. They
are similar to above examples, but use TLS with **rustls** crate. They can be
tested together using the local certificates provided.

- **[tls-client](https://github.com/PasiSa/pronets/tree/main/examples/tls-client/src/main.rs)**:
  Opens TLS session and sends message given as command line argument. You can
  test it with local server (at localhost) by giving the CA certificate located
  in `cert` directory, or a public server (e.g. HTTPS to `www.aalto.fi`) by not
  giving the CA. See examples at the beginning of `main.rs`.

- **[tls-server](https://github.com/PasiSa/pronets/tree/main/examples/tls-client/src/main.rs)**:
  Modified from the
  [async-server](https://github.com/PasiSa/pronets/tree/main/examples/async-server/src/main.rs)
  example based on Tokio, but now using TLS with **tokio-rustls** crate. The
  needed certificate and private key are located in the `cert` directory, which
  you need to give as command line argument.
