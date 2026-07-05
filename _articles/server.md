---
title: Simple server and Docker containers
---

Compared to network clients, server programming presents different properties
and challenges. Typically a server needs to run in the background for long times
without interruption and it needs to respond to connections from multiple
clients efficiently and manage the client sessions in parallel. Handling this
concurrency while taking care of other application logic is one of the main
difficulties in developing an efficient server. Because server need to run long
times (even weeks) without user interaction, and do it efficiently, memory
safety is essential. Careless memory management has caused many security
vulnerabilities some of which have gone unnoticed for long times, and memory
leaks eventually cause the server performance to collapse. This is one reason
why Rust has recently gained popularity among developers of network software.

In the latter part of this module we look into Docker that is a common
technology to package deploy servers in a scalable and portable way. We use
Docker to deploy student server programs into course's server host, so that they
are accessible for testing. From this point on, the following assignments are
designed to contribute to a network project that expands into full network
application by the end of the course.

<div class="objectives-frame" markdown="1">

**Objectives for this module:**

- You will **learn the fundamentals of passive server sockets** and how they are
  operated.

- You will **learn to implement an iterative, single-threaded server** that can
  take care of multiple parallel clients using non-blocking sockets.

- You will **understand the basics of container technology**

- You will **learn to build a Docker image** that runs server software built
  from Rust source code.

</div>

## Passive and active sockets

When a connection-oriented client socket is opened for communication, it is
called an **active socket**. An active socket can be used for both sending and
receiving data, and it is bound to both a local and remote IP address and
transport-layer port.

In contrast, a server application initially opens a socket in _passive_ mode. A
**passive socket** is not yet associated with a remote endpoint. It is bound
only to the local IP address and port on which the server listens for incoming
connection requests. This address needs to be known by the client so that it can
connect the server. A passive socket cannot be used to send or receive data.

In Rust, the `bind` call is used by the server to choose the IP address and
port. In modern systems it is common that a host has multiple IP addresses in
use at the same time for different network interfaces. For example, a laptop has
the loopback address 127.0.0.1 for host-local communication, and it can have
WiFi and wired LAN interfaces, both with different IP address. Commonly the IP
address is bound to "**any**" address, i.e., 0.0.0.0 in the case of IPv4. This
means that incoming connections are taken from any network interface. On the
other hand, if an application wants to limit to a particular interface it
accepts connections from, the address needs to be bound accordingly.

```rust
use std::io;
use std::net::TcpListener;

fn main() -> io::Result<()> {
    // Bind to TCP port 1234 on all local IPv4 network interfaces.
    let listener = TcpListener::bind("0.0.0.0:1234")?;
    println!("Server listening on 0.0.0.0:1234");

    // Wait until one client connects.
    let (stream, address) = listener.accept()?;
    println!("Accepted connection from {address}");

    // 'stream' is now an active socket to the newly connected client.
    // You can start sending and receiving from it normally.

    Ok(())
}
```

When a new connection request comes in at the server, it needs to accept the
connection request using `accept` call. This creates a new active socket for
communication with the incoming client. This socket has both endpoint addresses
defined, and it can be used for sending and receiving data. After this the
operation of the socket becomes symmetric: both ends can send and receive data
as they wish, but typically based on some defined protocol. Over time, there may
be multiple active sockets open as new clients arrive, and the server needs to
apply some strategy how to manage the concurrent clients in timely way,
remembering that by default read and write calls may block program execution
indefinitely, unless concurrency and non-blocking operation is taken care of
appropriately.

## Simple iterative server

We will now take a look at
**[simple-server](https://github.com/PasiSa/pronets/tree/main/examples/simple-server/src/main.rs)**
example in our GitHub repository, probably the simplest server implementation
possible. This program accepts incoming connections one at the time, reads any
data sent by the accepted client, and then echoes the data back. After this the
connection is closed and the server starts to wait for the next client. The
server takes the IP address and transport port to bind to as command line
argument. If you use "0.0.0.0" (assuming IPv4) as the IP address, connections
are accepted from all network interfaces. If you use 0 as transport port, system
will pick an available port for you. In practice this is inconvenient, because
then the client applications would not know which port to connect to.

First you need to start the server by something like:

    cargo run -- 0.0.0.0:2000

and then on another terminal window you can use netcat to test it, and typing
some message:

    nc 127.0.0.1 2000

Or, you can use the simple client on the other terminal window to send the
message (running this on the simple-client directory):

    cargo run -- 127.0.0.1:2000 Hello

The simple server starts by creating a passive server socket and binding it to
the address given as command line argument. `server` is the passive server
socket listening for connections.

```rust
let server = TcpListener::bind(&args[1])?;
```

Then it starts a loop that starts by waiting for the next incoming client. The
`accept` call may block the execution for a long time.

```rust
let (mut socket, address) = server.accept()?;
println!("Accepting connection from {}", address.to_string());
```

When the call completes, we will get the active `socket` representing the
connected client, and the address of the client, that will be printed on the
terminal.

After this, the server will read some data from the active client socket,
assuming that client knows that it is expected to write something. If the client
did not write anything, but would rather wait some input from elsewhere, the
`read` call would block for a long time.

```rust
let mut buf: [u8; 160] = [0; 160];
let readn = socket.read(&mut buf)?;
```

Finally, the server echoes the data that was read back to the client, and closes
the socket, as the lifetime of the local `socket` variable ends at the end of
the loop.

## I/O multiplexing and non-blocking sockets

Sockets can be **can be configured into a non-blocking mode**, in which case the
calls return immediately, but for example, if `read` did not have any data to
read, and it would have blocked in the blocking mode, the call returns a
specific **WouldBlock** error code (that is not actually an error). A naive
implementation would be to build a while loop in the server that reads all
sockets in this way. However, this would create a busy loop that would
unnecessarily load the CPU, even if no data is coming from any of the clients.

To avoid unnecessary CPU load, the Posix C API has functions
**[select](https://man7.org/linux/man-pages/man2/select.2.html)** and
**[poll](https://man7.org/linux/man-pages/man2/poll.2.html)** functions that can
be used to wait simultaneously I/O events from any of the defined sockets, or
other I/O sources. These functions block until any of the give sources can be
called so that the execution would not block. Their return value indicate the
sources with available events, that can then be iterated one by one. In addition
there are system-specific, more efficient variants for these functions, such as
`epoll` in Linux or `kqueue` in BSD-based systems and MacOS.

In Rust, [**mio**](https://docs.rs/crate/mio) is a library (or "crate" in Rust
terminology) that encapsulates the non-blocking socket operation into convenient
set of functions. Our next example is
**[iterative-server](https://github.com/PasiSa/pronets/tree/main/examples/iterative-server/src/main.rs)**
that demonstrates the use of _mio_ (you may want to open the code in a parallel
window while reading this section). The server just reads incoming data from
socket and echoes it back. Different from the earlier implementation, the server
does not close the socket after writing data, but after responding to client, it
continues waiting for more data, until the client closes the connection.
Therefore the server needs to prepare to handle multiple client sockets
simultaneously.

The first lines of the `main` function are similar to previous example, reading
the binding address from command line arguments. Then we set up Mio's poll
service and container for the Mio events. Each possible event source is assigned
an unique "Token" that identifies the event source, basically not much different
from integer. We implement a small "TokenManager" for easier allocation and
release of unique tokens in a separate file, `tokenmanager.rs`.

First we add just the passive listening socket as event source ([line
60](https://github.com/PasiSa/pronets/blob/6e0d2f11eb9fdfd06c07322733acb3b109110bd9/examples/iterative-server/src/main.rs#L60)).
Note that with Mio the `TcpStream` and `TcpListener` implementations are
different than the standard implementations of the same types (see the `use`
statements in the beginning of the program). These are compatible with Mio and
implement non-blocking operation.

The heart of the main event loop is Mio's `poll` function ([line
71](https://github.com/PasiSa/pronets/blob/6e0d2f11eb9fdfd06c07322733acb3b109110bd9/examples/iterative-server/src/main.rs#L71))
that stops until at least one event is available. After poll
completes, there may be multiple events available, so we need to handle all of
them iteratively. If there is an event on the listening socket, we know that we
can call `accept` safely without blocking the program. We have a small `Client`
structure that contains the socket and address of an client. All active clients
are stored in a `HashMap` container. If there was any more complicated
application logic, the `Client` structure could contain also other
client-specific information that is needed. When a new client is accepted, a new
token is allocated for it and registered to Mio as an interesting event source.

Mio has separate event types for situations when socket is readable, and for
situations when socket is writable without blocking the execution. If we wanted
a proper implementation, we should also handle the `write` calls through an
event processing loop, but in this case we skip it for simplicity (and perhaps
laziness). On the other hand, we write a maximum of 160 bytes, so it can be
assumed to take quite many write calls without client reading anything before
the send buffer gets full and blocks writes.

After client connections are opened, also the possible client socket events are
checked in separate if branch. Here one should note handling of the `read` call
return values. In Rust, an often used return type is `Result` that can yield two
return value variants. `Ok` response is returned when read is successful. In the
case of Ok, the return value will indicate the number of bytes read. If the
return value is 0, the client has closed the socket, and therefore we should
clean up: release the Mio event token, and remove the client from the HashMap.
This also causes the lifetime of the socket to end, so it will be cleaned up
also from our end. `Err` response means that error occurred in read. Also in
this case we clean up the client socket, but do not terminate the operation of
the main server loop. Earlier we have mostly used the `?` operator that
propagates the possible error up in the call stack, which would have caused
termination of the program.

The `write` call shows another way of checking for an error outcome, in case we
are not interested in the exact Ok return value. A better alternative, in
addition to handling the write call through the writable event, would be to
check how many bytes were actually written, and prepare for the case when only
part of the data was written. Again, lazy coding.

You can test the program by first starting the server in the same way as before:

    cargo run -- 0.0.0.0:2000

Then, open more than one terminal windows where you start a netcat session in
each, opening multiple connections to server:

    nc 127.0.0.1 2000

Try typing different things to different terminal windows, closing netcat in
some windows by Ctrl-D (Hang-up of connection) or Ctrl-C (Interrupt netcat), and
then restarting netcat.

A benefit of a single-threaded, event-driven server design is that it can scale
efficiently and behave predictably (as long as the operations are not blocking),
as it avoids thread management overhead and synchronization. However, designing
such applications can be complex, particularly with respect to state management
and robust error handling.

## Docker basics

Docker is a way to package an application with its runtime environment so that
it behaves consistently across different machines. On this course we will use
Docker to deploy the server implementations between your local machines to the
course servers hosted by the university. Docker allows you to test the
application first locally and then move it to a server where everyone can access
it.

First, one produces a Docker image that contains a packages filesystem and
needed metadata. `Dockerfile` specifies the recipe for building an image. Docker
images are typically layered: new files and data are built on top of an base
image (for example one that contains the basic Linux tools), so that a single
image does not have to contain a full disk image, like a virtual machine would
have.

Docker container is a running instance of a image. On large services, Docker is
used to dynamically replicate and scale services into multiple, typically
distributed containers.

Often Docker images are deployed in registry (such as `docker.io`), where they
can be found and used by anyone needing them. On this course we are not using a
registry, but the course server rebuilds the image based on student git
repositories.

On most systems, easiest way to get Docker in a local system is to install
[Docker Desktop](https://docs.docker.com/desktop/), that comes also with the
command line tools mentioned below.

### Building a Docker image

Below is an example of a `Dockerfile` that has the **builder part** for building a
binary server application from a Rust source code, and **runtime part** that
runs the binary server application, opening TCP port 1234 for connections from
the outside world.

```dockerfile
# Build the Rust server application.
FROM rust:1.96 AS builder

# Set the working directory inside the container.
WORKDIR /usr/src/my-server-app

# Copy the source code from local machine to the container.
COPY ./ ./

# Build the server in release mode for better performance (inside the container).
# In this case, the server is located in a separate 'server' package under
# the Rust project.
RUN cargo build -p server --release


# Run the compiled server from a smaller runtime image.
FROM debian:bookworm-slim AS runtime

WORKDIR /usr/local/bin

# Copy the compiled server binary from the builder stage to the runtime stage.
COPY --from=builder /usr/src/pronets-demo-chat/target/release/server ./server

# Expose the port that the server will listen on.
EXPOSE 1234

# Set the command to run the server when the container starts.
CMD ["./server", "--port", "1234"]
```

The builder part of the Dockerfile can be run, for example, with the following
command on the command line.

```bash
docker build -t my-server-app .
```

The `-t my-server` option gives the built image a local name. The final `.`
tells Docker to use the current directory as the build context.

After this the container can be executes as follows:

```bash
docker run -d -p 1234:1234 --name my-server-app my-server-app:latest
```

The `-d` option causes the server to be run in the background. To keep it on the
terminal foreground (e.g. for development and debugging), you can leave the `-d`
option out. The `-p 1234:1234` causes TCP port 1234 from the container to be
exposed at the host machine, so that it can be connected from outside
applications.

### Using Docker

_TODO: docker ps, docker logs, stopping container, running commands inside
container_

## Rust project management

_TODO: Packages, crate dependencies_

## Assignment

<div class="assignment-frame" markdown="1">

We now start developing network software project in the git repository created
in the beginning of the course. It is recommended that you include the client
and server implementations in the same repository and same Rust project, as
separate packages as described above.

**Part 1**: Implement a simple server that listens to incoming connections at
the port assigned for you. At this point the server recognizes only one type of
message: Test message "**TST**" that can be used to test that the server is
alive and responsive.

Our message structure, that also the TST message applies is as follows:

- The message starts by an **32-bit unsigned integer** that indicates the
  **message length**. The integer must be in big-endian network byte order (see
  previous module how to do this). The length covers whe length of the whole
  message, including the length field itself and the identifier number.

- Then there should be another **32-bit unsigned integer** that is the **message
  identifier**, in network byte order. Later on, this can be used to identify
  responses to different kinds of request messages.

- After this there is message type "TST" followed by space and arbitrary
  content.

When server receives this message, it must echo the same message back to the
client, using the same length and message identifier.

**Part 2**: Implement also a client application that sends a TST message, so
that you can test the server.

**Part 3**: Add **Dockerfile** to your project that builds the server and starts
it.

After the Dockerfile is created and you have tested that the server works, it is
time to push your work to your git server repository.

**Part 4**: The server should be registered to course server by using a HTTP
POST request to **pronets.dice.aalto.fi**, port 80. The exact HTTP endpoint is
`POST /run-docker`. The body of the POST request must be JSON encoded and have
the following fields:

- **"username"**: The username you want to use with the server. This should be
  the username you declared earlier.
- **"git-repo"**: The URL of the Git repository you are using for your course
  assignments and project.
- **"ports"**: The port number(s) that the server listens for connections. For
  now there is only one port in string format.

Note that the structure is same as with the "`/fetch-git`" endpoint developed in
last assignment, with one additional field.

Implement a program that sends the HTTP request (or it could be part of your
client code, for example) and waits for response. When the server receives the
request, it fetches your code from git, and uses your Dockerfile to build and
run your server. Note that this takes time (could be couple of minutes even),
but if everything goes well and you eventually receive "OK" response, your
server code should be running in `pronets.dice.aalto.fi`, listening to
connections at the port you have given.

The main course "master server" sends the TST message every 10 seconds to all
containers that have been started. You can see all the registered servers, along
with their ports, and the outcome of the latest TST message at
**http://pronets.dice.aalto.fi/containers.**

Write a short report where you document the progress going forward with the
above steps, focusing on the challenges you had with your work. Can you find
your container in the `/containers` view, and does is show "TST OK" for your
container? Again, include also the following information:

- How much time did you use for this assignment?

- What was easy or difficult in the assignment?

- What tools did you use? In particular, if you used AI assistants, tell how did
  you use then and if they were helpful.

</div>
