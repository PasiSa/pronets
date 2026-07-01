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

- _TODO: more examples to follow_
