# Demo client/server project using Slint UI

This example shows how client and server implementations can be included in the
same project as two different packages. Similar structure is recommended also in
the course projects. The example also shows simple example how to use the Slint UI
framework for client. The example intentionally does not do anything useful, but
feel free to take it as a basis for building your own application.

## Building and running the Rust code

Server:

    cargo run -p server

Running client that connects to an address:

    cargo run -p client -- -a pronets.dice.aalto.fi
