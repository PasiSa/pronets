---
title: Rust basics and client sockets
---

In this material, all examples and reference code are written in Rust. Rust is a
relatively new language that has become popular, for example, among developers
who write distributed networking code. Like C and C++, Rust is compiled into
machine code, so Rust programs can be expected to be about as efficient as
programs written in C or C++. However, Rust's data ownership model aims to
provide better memory safety (which is important for security), and Rust also
has modern package management and testing support, among other useful features.

**Objectives for this module:**

- You will get an **initial understanding and some practice of Rust basics** and
  the mechanisms specific to it, for example related to memory ownership.

- You will **learn to implement a basic TCP client** that connects to existing
  server, and sends and receives data to it.

- You will **understand how to transfer binary data between computer memory and
  protocol messages** over the Internet. Particularly, it is useful to understand
  that binary numbers can be encoded in different ways in different computer
  architectures that need to interact between one another.

- You will **learn to make a basic HTTP request and receive a response to it**
  from a compiled (Rust) program.

- The work on this course is maintained in a Git repository. In this module you
  will **learn the basic functions of Git**, and how to set up a Git repository.

## Rust basics

The [Rust book](https://doc.rust-lang.org/stable/book/) gives a comprehensive
learning material and overview of the Rust language. It is recommended that you
get familiar with it. This material does not cover the Rust specifics in detail,
but points to the respective chapters in the Rust book in the appropriate
places. We focus on those features of Rust that help you get started with
network programming.

### Installing Rust and other tools

The Rust book starts with installation instructions for Linux, Mac OS and
Windows in [section
1.1](https://doc.rust-lang.org/stable/book/ch01-01-installation.html). Rust uses
the **rustup** installation tool that should make installation easy in most
environments. The tool install all the Rust tools needed for package management,
testing, code formatting, etc.

For editing the programs, [VS Code](https://code.visualstudio.com/) is a popular
and useful tool. If you use VS Code, it is recommended to install the Rust
extensions, such as **rust-analyzer** and **Rust Syntax** highlighting module
for better experience. You can find these in the VS Code extension catalogue
after installing the tool.

A little bit later you will also need **git** and **Docker**. Code submissions
should be pushed to git repository created for this course, and Docker is used
to run the server programs in a commonly accessible server machine.

After installations, try the [Hello
Cargo!](https://doc.rust-lang.org/stable/book/ch01-03-hello-cargo.html) example
in Rust book section 1.3. Cargo is the build and package management tool for
Rust, and will be used heavily from this point on.

To get some hands-on feeling about Rust programming, you can also try the
"Programming Guessing Game" example in [Chapter
2](https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html).

### Variables, data types and functions

Like practically all programming languages, Rust organizes the program code
inside functions, and stores data values in variables. The **main** function
starts the program execution, and any non-trivial program has many other
functions to implement the program logic. In Rust, variables are declare either
as **non-mutable**, in which case their value cannot be changed afterwards, or
**mutable**, making it possible to change the variable value over time. [Section
3.1](https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html)
in the Rust book gives some examples of this.

Like C or C++, Rust assignes fixed **data type** to all values and variables.
There are types for unsigned and signed integers of different sizes, and
separately for floating point numbers of different sizes. Rust also has
**boolean** type, and a type for primitive characters, similarly to C. See more
about data types in [Section
3.2](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html) of the Rust
book.

[Section
3.3](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html)
gives an overview of how to use functions, with arguments and return values in
Rust, and [Section
3.5](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html) gives an
overview of the basic control flow expressions. These are mostly similar to
other common programming languages, perhaps apart from some syntactical
specifics.

_TODO: Discuss function return values, particularly the Result type_

### Ownership and references

The concept of **ownership** is central to Rust programming that makes it
different from many of the earlier systems programming languages such as C or
C++. Every value in Rust has an **owner**, and there can be only one owner at a
time. and the Rust compiler aims to carefully control that the ownership rules
are followed, and refuses the compile the program if the program violates these
rules. As a result, for beginner the Rust programs may be difficult get compiled
before the ownership rules are understood. In return we avoid the run-time
memory management problems that make the C programs so difficult to debug.

Also the concept of **scope** in Rust, as it affects the memory allocation and
release. In Rust programs developer does not need to manually release allocated
memory, but it is automatically released when the variable or value owner goes
out of scope. [Rust book section
4.1](https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html) has
more about this.

_TODO: example_

Especially when functions are used, we talk about borrowing, i.e., passing a
value by reference to be used inside the function. In this case the ownership is
not transferred inside the function, but stays with the caller, and the function
just **borrows** the variable using the reference. The reference can be passed
as non-mutable or mutable. If the values need to be modified, i.e., they need to
be passed as mutable references, some special attention is needed: to ensure
data consistency, Rust compiler allows only one mutable reference inside its
scope. This can sometimes cause headache to beginning Rust programs, especially
during program loops that would like to call the same function repeatedly. [Rust
book section
4.2](https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html)
has more about this.

_TODO: example_

### Structures and methods

Similarly to other programming languages, also Rust allows using **structures
(or structs)** to collect together related data. The [Rust book section
5.1](https://doc.rust-lang.org/stable/book/ch05-01-defining-structs.html) shows
how this is done, and how structures are instantiated, used, and modified in a
program.

Functions related to operating the structure can be added as methods inside the
structure's namespace. This is a good way to structure the program into logical
modules, similarly to how classes are used in object-oriented programming. [Rust
book section
5.3](https://doc.rust-lang.org/stable/book/ch05-03-method-syntax.html) discusses
methods.

_TODO: Enums and Tuples_

### Collections

_TODO: Allocated from heap._

See text about vectors in [Rust book section
8.1](https://doc.rust-lang.org/stable/book/ch08-01-vectors.html).

See text about Hash Maps in [Rust book section
8.3](https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html).

### Setting up a Rust project

_TODO: Cargo.toml, common directory structure_

## Stream socket basics for client applications

Applications send and receive data using the **socket** abstraction between
application and operating system kernel, that encapsulates and decapsulates the
data in protocol packets and passes the data to the network. Commonly, socket
encapsulates one communication session between the local application and a
remote peer, for example in most common case, one TCP connection. There are
various types of sockets, but on this course we focus on **stream sockets** that
are commonly used for reliable stream-oriented communication using TCP protocol,
and **datagram sockets** that are used for unreliable datagram-oriented
communication using UDP. However, also other protocols could be used with these
types of sockets.

![Interaction between application and OS with sockets](/images/basics-socket.svg){: width="90%" .center-img }

Socket API is natively defined in the Posix API in C language. The Rust standard
library has wrapped the socket operations in Rust functions, providing
convenient (and easier) socket API for Rust programmers.

We will start with stream sockets and return to datagram sockets a bit later in
the course. **Stream socket** provides a reliable byte pipe between two
communicating end points. One of the end points takes the role of a **server**,
i.e., it listens passively for incoming connection requests at a known address
and port, and then creates an active socket to start actual communication. Other
end point is the **client** that opens a connection to the known address where
server should be listening for incoming connections. We start with clients, and
return to server sockets in the next module.

### Establishing a connection

In Rust, a TCP-based client stream socket is typically created using the
`connect()` function:

```rust
    TcpStream::connect("some.address.fi:5000");
```

Using the string argument is most convenient in most cases, but there are also
other ways for passing the destination address, for example in the binary form.
The argument can include a DNS name, in which case a DNS name resolution is done
first to find the actual destination IP address, or it could be directly an IP
address. A full documentation of the **TcpStream** struct is given in [Rust
documentation](https://doc.rust-lang.org/stable/std/net/struct.TcpStream.html),
including some examples.

Connect function returns a **Result** enum, i.e. it can fail for different
reasons, for example if the name cannot be resolved, or the address is otherwise
invalid, or if the destination address and port cannot be reached. If the
function is successful, it returns a stream object that can be used for different
network operations from this point on.

Like with many network functions, the connect function call can take a long time
to complete, because it involves network communication. In fact, quite much
happens:

- If DNS name is given in argument, the client system and its name resolver
  library starts DNS name resolution. If the name is not in local or nearby
  cache, this may require multi-stage name resolution with Internet name
  servers, as discussed earlier.

- Once the IP address is known, the function initiates TCP's three-way
  connection establishment handshake with the server host. Because we are
  communicating over the Internet, there are delays, and there may be packet
  losses. If packet losses happen, the operating system TCP implementation makes
  needed number of timer-based retransmission attempts until it gives up. The
  timeout value is doubled after every failed attempt, until the connection
  succeeds or the TCP implementation gives up after some number of attempts.

In practice, when a program calls `connect()`, in an unlucky case the execution
may stop for tens of seconds, after which the call either succeeds or fails.
This needs to be taken into account if the client program has interactive or
time-sensitive components.

_TODO: add a time-sequence picture about all this_

### Writing data to socket

Stream sockets transfer unstructured byte stream between two endpoints. This is
a common communication model especially in the traditional network applications,
such as file transfer (which classical world-wide web also is to large extent).
Stream socket does not pay attention to message boundaries, or does not care in
what sequence of `write()` calls the data was written. Everything is handled as
a continuous stream of ordered bytes. Stream socket aims to guarantee that when
data is successfully delivered, the bytes are delivered to receiver in the
**original order** and bytes are **not corrupted**. To guarantee this, the TCP
protocol applies checksums, retransmissions and buffering at both ends of the
connection. To the application possible problems therefore come out as variable
delays in data transfer.

The operating system kernel maintains **socket buffers** both for data **to be
written** to network, and for data that has been **received from network
packets**, but not yet read by the application. When application calls the
`write()` call, the data passed with the call is **copied to operating system
socket buffers**. After that the call completes. In other words, when `write()`
call has completed, in common case the **data has not actually left the local
computer system** yet, but waits in the operating system buffers. There are
multiple things that the operating system must do for actually sending the data:

- The data has to be split into TCP segments, i.e. IP packets to be transmitted
  over the Internet.

- Before sending the packets, the TCP sender needs to ensure that receiver has
  enough buffer space to receive the packets. For this, the TCP receiver tells
  in the protocol acknowledgments how much free space it has available in its
  receive buffer. This is called **flow control**.

- TCP sender applies **congestion control**, i.e. it adjusts the rate of
  outgoing data based on its measured estimate of network capacity. In the
  traditional form of congestion control, TCP maintains a **congestion window**
  that tells how much unacknowledged data there can be in outstanding in the
  network. If the congestion window gets full, the TCP sender must delay sending
  new data for a while.

The data is stored in the socket send buffers until a TCP acknowledgment
confirms that the receiver has the data. After this the TCP sender can release
the data and free some space in the buffer. But even after this, the sending
application does not know if the receiving application has actually read the
data from its socket buffer.

Also the `write()` call can block execution for indefinite amount of time, if
the socket send buffer gets full. The `write()` call does not necessarily write
all of the data it has been given: if the buffer has space for some of the data
in `write()` call, that number of data is copied and write call completes. The
return value tells how many bytes were actually copied. If this was less than
originally given, the application logic has to be designed appropriately, for
example to make another `write()` call to send the rest of the data (iteratively).

To make this easier, there is another version of write, the `write_all()`
function that blocks until all of the data is copied.

### Reading data from socket

The `read()` call from the socket works conversely to the `write()` call: at the
TCP receiver it copies data from the socket receive buffers to the application
buffer given as call attribute. If there is no data to be read in the socket
buffer, the call may block indefinitely. If there was less data in the socket
buffer than the size of the allocated application buffer, the `read()` call
returns, but indicates the actual number of bytes copied as its return value.

### Example

Below is a short example that uses the above-mentioned function or the mentioned
variations.

There is also a similar example,
"**[simple-client](https://github.com/PasiSa/pronets/tree/main/examples/simple-client/src/main.rs)**"
in the course material [git repository](https://github.com/PasiSa/pronets).

```rust
// io is used for the io::Result return type. Read and Write bring the
// read_to_string() and write_all() methods into scope for the TCP stream.
use std::io::{self, Read, Write};

// TcpStream is the standard library type for a TCP socket connection.
use std::net::TcpStream;

// Main function returns a Result enum type, i.e. either "Ok" or "Err", depending
// of its outcome. Because this is the first (and only) function called,
// the return value determines the process result code and nothing else.
fn main() -> io::Result<()> {
    // Set address and port as Rust string
    let address = "localhost:5000";

    // Open a TCP socket and connect it to above-given address.
    // The question mark in the end tells to propagate the possible error as
    // the result of calling function: if there is an error, the function execution
    // is finished here, and Err value is returned.
    // stream variable needs to be mutable, because the read and write operations
    // change the stream TcpStream structure state.
    let mut stream = TcpStream::connect(address)?;

    println!("Connected to {address}");

    // Specify message string and write it to stream socket.
    // write_all() function tells that the function should terminate only after the
    // full message is written.
    // The function wants to write u8 type byte array, therefore the string needs
    // to be converted using as_bytes() function.
    // Also this call can end in error (that will be propagated outside the function),
    // like all other socket functions.
    let message = "Hello there!";
    stream.write_all(message.as_bytes())?;

    // Allocate u8 buffer of 160 bytes and read data to it from socket.
    // Print the output to screen.
    // Interpret as UTF-8, replacing invalid bytes instead of returning an error.
    // If read causes error, exits the main function
    let mut buf: [u8; 160] = [0; 160];
    let n = socket.read(&mut buf)?;
    println!("Read {} bytes: {}", n, String::from_utf8_lossy(&buf));

    // Return Ok return value from function, everything was successful.
    Ok(())
}
```

The `Read` and `Write` names imported from `std::io` are **traits**. A trait in
Rust defines behavior that different types can provide. In this example,
`TcpStream` implements both of these traits: it can be read from, because bytes
can arrive from the network, and it can be written to, because bytes can be
sent through the connection.

The `Read` trait provides methods for receiving bytes from a stream. For
example, `read_to_string()` reads bytes from the TCP connection and appends
them to the `String` variable. The `Write` trait provides methods for sending
bytes to a stream. For example, `write_all()` writes the whole byte slice to the
connection before returning successfully.

These traits are needed because `TcpStream` only represents the TCP connection
itself. The common operations for reading and writing streams are defined by
the `Read` and `Write` traits, so the same method names can be used with many
different stream-like types, such as files, standard input and output, and TCP
connections.

The traits must also be imported into scope. Although `TcpStream` implements
`Read` and `Write`, Rust only allows calling trait methods such as
`read_to_string()` and `write_all()` when the corresponding traits are visible
to the compiler. Without `use std::io::{Read, Write};`, the compiler would know
that `stream` is a `TcpStream`, but it would not find these trait methods for
it.

## Encoding binary data

Byte order, packing and structures

## HTTP basics

Needed in the first assignment

## Using Git

How to set up Git repository in version.aalto.fi

Setting up ssh keys to the repository

Cloning repository to local machine

Committing changes

Pushing local commits to the server

## Assignment

_TBD: Set up Git repository for your course work_

Implement a program that opens a TCP socket and connects it to
"pronets.dice.aalto.fi" port 10000, and writes message `TST (some string)`.
Then the program should read bytes from the socket.

Implement another function to your program that opens a TCP connections to
"pronets.dice.aalto.fi", port 80, and makes a GET request for `index.html`.

_TBD HTTP POST request_

Before you run the program, open Wireshark to analyze the communication that
happens.

- Identify DNS query and DNS response. What fields and what values does the
  query and response have?

- Identify TCP connection establishment for the TST exchange. What is the source
  TCP port that is used? What TCP options are in use in the first packet with
  SYN flag?

Finally, answer the following questions:

- How much time did you use for this assignment?

- What was easy or difficult in the assignment?

- What tools did you use? In particular, if you used AI assistants, tell how did
  you use then and if they were helpful.

_TBD..._
