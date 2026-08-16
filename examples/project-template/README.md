# Demo client/server project using Slint UI

This example shows how client and server implementations can be included in the
common project workspace as two packages. The example also shows simple example
how to use the Slint UI framework for client. The example does not do anything
useful, but you can use it as a basis for building your own application with a
graphical client implementation.

## Building and running the Rust code

Server:

    cargo run -p server

Running client that connects to an address:

    cargo run -p client -- -a pronets.dice.aalto.fi
