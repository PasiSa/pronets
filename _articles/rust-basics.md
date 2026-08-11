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

<div class="objectives-frame" markdown="1">

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

</div>

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

Like most other programming languages, Rust organizes the program code
inside functions, and stores data values in variables. The **main** function
starts the program execution, and any non-trivial program has many other
functions to implement the program logic. In Rust, variables are declare either
as **non-mutable**, in which case their value cannot be changed afterwards, or
**mutable**, making it possible to change the variable value over time. [Section
3.1](https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html)
in the Rust book gives some examples of this.

Like C or C++, Rust assigns fixed **data type** to all values and variables.
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

The following example defines an immutable string variable and a mutable integer
variable. The integer is changed before both values are passed to a function:

```rust
fn make_score_message(name: &str, points: i32) -> String {
    let doubled_points = points * 2;
    format!("{name} has {doubled_points} points")
}

fn main() {
    let player_name = "Ada";
    let mut score: i32 = 10;

    score += 5;

    let message = make_score_message(player_name, score);
    println!("{message}");
}
```

The `player_name` variable is non-mutable, which is the default for variables
declared with `let`. Assigning another value to it later would cause a compiler
error. The `score` variable is declared with `let mut`, so its value can be
changed. Its `i32` (32-bit signed integer) type is written explicitly, while
Rust infers the type of `player_name` from its value. In Rust programming, it is
common not to explicitly indicate the variable type with the definition, if the
compiler can infer it. Editor tools, such as the Rust extensions in VScode can
use visual hints to indicate the programmer what the actual type is. The
`make_score_message` function receives both values, doubles the score, and
returns a new `String` containing the result. The program prints `Ada has 30
points`. The last line in the function does not end in semicolon, because it
returns the String value that is passed as the return value of the function.
This is common style in Rust, although the _return_ keyword is also available,
similarly to C.

You can try the above program by copying it to a source file (e.g. "_main.rs_"),
and using the `rustc` compiler to build the binary:

    rustc main.rs

This produces executable file "_main_" that you can run on command line to see
the output:

    ./main

Rust has two commonly used string types. A `String` owns its text, stored in a
dynamically allocated buffer, and can grow or be modified when the variable is
mutable. The `str` type represents a sequence text that is usually accessed
through a borrowed reference written as `&str` (called "string **slice**").
String literals such as `"Ada"` have the type `&str`. In the example, the
function borrows this string data through its `name: &str` parameter, but
creates and returns an owned `String` with the `format!` macro. In Rust, strings
are UTF-8-encoded.

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

The following function borrows a `String` through a mutable reference and
modifies it without taking ownership:

```rust
fn add_greeting(name: &mut String) {
    name.insert_str(0, "Hello, ");
    name.push('!');
}

fn main() {
    let mut name = String::from("Ada");

    add_greeting(&mut name);
    println!("{name}"); // Prints: Hello, Ada!
}
```

The caller declares `name` as mutable and creates a mutable reference with
`&mut name`. The parameter type `&mut String` allows `add_greeting` to change
the borrowed value. Ownership remains in `main`, so `name` can still be used
after the function call and contains the changes made by the function.

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
methods. The methods are defined in a separate `impl` block, that usually
follows the actual `struct` definition that just defines the data that is
included in the struct, as shown in example below. Functions usually use `self`
reference to refer to an instance of a struct that is being operated. If the
function modifies the content of the structure, the self reference needs to be
defined mutable: `&mut self`. Ownership rules apply as with any other variables
also for the "self variable.

**Enumeration (_enum_)** is a a type that can have multiple variants. Below is
an example (related to our course scope), of a structure that can handle both
IPv4 and IPv6 addresses. The example also shows a simple struct. This example is
taken and modified from [Rust book section
6.1](https://doc.rust-lang.org/stable/book/ch06-01-defining-an-enum.html) that
discusses more about enums.

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

impl IpAddr {
    fn print_address(&self) {
        println!("{}", self.address);
    }
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    home.print_address();
    loopback.print_address();
}
```

Two particularly useful enums defined in Rust standard library are **Option**
and **Result**.

**Option** can have two values: **None** or **Some(T)**, where T can be any
generic type. This allows specifying variables that do not always contain valid
value. Classically in C, _null_ value has been used for this situation. The Rust
book discusses why this has been a mistake, that has led to different kinds of
vulnerabilities and system crashes over the years. Rust does not use _null_, but
the _Option_ enum, that is a cleaner way of expressing the same meaning.

**Result** type can also have two values: **Ok(T)** or **Err(E)**. Both variants
contain generic data type. For example, if executing a function us success, it
can return some value of type _T_, wrapped inside _Ok_ enum. If there is an
error, it returns _Err_, with E telling something about the reason of error. It
could be a string or an error code, for example. This is particularly useful in
network programming, because or operations interacting with external network can
fail for various reasons not in the control of the programmer or the user. In C,
classically special return values, such as -1 as integer are used to indicate
error, e.g. in Posix API. In my opinion, the _Result_ enum is nicer way for
doing that.

The following example uses `Option` when an address might not be found and
`Result` when parsing a port number might fail:

```rust
fn find_address(host: &str) -> Option<String> {
    if host == "localhost" {
        Some(String::from("127.0.0.1"))
    } else {
        None
    }
}

fn parse_port(text: &str) -> Result<u16, String> {
    match text.parse::<u16>() {
        Ok(port) => Ok(port),
        Err(_) => Err(format!("'{text}' is not a valid port number")),
    }
}

fn main() {
    match find_address("example.com") {
        Some(address) => println!("Address: {address}"),
        None => println!("Address was not found"),
    }

    match parse_port("not-a-number") {
        Ok(port) => println!("Port: {port}"),
        Err(error) => println!("Could not parse port: {error}"),
    }
}
```

The example also shows the use of `match` expression, that is particularly
useful in Rust to handle the different possible cases. It requires the program
to handle both possible variants. In the first call, `find_address` returns
`None`, and in the second call, `parse_port` returns `Err`. Using `"localhost"`
and `"8080"` instead would exercise the `Some` and `Ok` branches.

### Collections

Collections are dynamic data types to store multiple data items of varying
numbers. The memory used by collection is dynamically allocated from the heap
memory of the computer system, and released automatically when collection runs
out of scope (i.e., code block or function it is used in).

**Vector (Vec)** is an ordered list of values of specific type (see [Rust book
section 8.1](https://doc.rust-lang.org/stable/book/ch08-01-vectors.html)). The
different items in the vector can be pointed by its index. The _Vec_ type has
various functions for managing, such as `push()` to add values at the end of the
vector or `pop()` to take the last value from the vector. The [Vec reference
documentation](https://doc.rust-lang.org/std/vec/struct.Vec.html) has a complete
list of these operations.

The following example creates a vector of port numbers and modifies its
contents:

```rust
fn main() {
    let mut ports = vec![80, 443];

    ports.push(8080);
    println!("First port: {}", ports[0]);

    for port in &ports {
        println!("Port: {port}");
    }

    match ports.pop() {
        Some(port) => println!("Removed port: {port}"),
        None => println!("The vector was empty"),
    }
}
```

The `vec!` macro initializes the vector, and Rust infers its type as `Vec<i32>`.
The loop borrows the vector so it remains usable afterwards. The `pop()` method
returns an `Option`: it contains `Some(port)` when an item was removed or
`None` when the vector was empty.

A vector should not be confused with an **array**, that has a fixed length that
is part of its type, for example as in following:

```rust
fn main() {
    let mut numbers = [5, 6, 7, 8];  // Inferred as [i32; 4]

    numbers[0] = 4;

    for number in numbers {
        println!("number: {number}");
    }
}
```

The above array contains four i32-type elements stored inline (not typically in
a dynamically allocated buffer). The length of the array is part of its type,
and cannot be changed. An array can be mutable, in which case its elements can
be changed, but its length is still fixed.

**HashMap** maps keys (of chosen type) to values (of chosen type), and allows
efficient lookup of values based on the key (more details in [Rust book section
8.3](https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html)). The entries
are placed in memory using a hashing function, and the order of items in
_HashMap_ is not therefore deterministic. New key-value pairs can be added to
HashMap using the `insert()` function. Each key can be in _HashMap_ only once:
if the inserted key already existed in HashMap, the value will be replaced by
the new one.

The following example maps service names to their default port numbers:

```rust
use std::collections::HashMap;

fn main() {
    let mut service_ports: HashMap<&str, u16> = HashMap::new();

    service_ports.insert("http", 80);
    service_ports.insert("https", 443);

    match service_ports.get("https") {
        Some(port) => println!("HTTPS uses port {port}"),
        None => println!("HTTPS was not found"),
    }

    // Print all entries in the service_ports HashMap.
    for (service, port) in &service_ports {
        println!("{service}: {port}");
    }
}
```

The type `HashMap<&str, u16>` specifies string slices as keys and unsigned
16-bit port numbers as values. The `get()` method returns an `Option`, because
the requested key might not exist. Iterating over a `HashMap` produces its
key-value pairs, but their order is not guaranteed.

### Setting up a Rust project

For any larger project, it is better to use the **cargo** tool for various
tasks. Cargo can do many things needed during development: building and running
the project, manage library dependencies, cleaning up code style, running tests,
and so on.

A new cargo project is created in the following way:

    cargo new my_project

This creates a `Cargo.toml` file that defines the project attributes, and
particularly, library dependencies. It also creates a `src` directory where the
source files are customarily placed, and a placeholder `main.rs`. Also a new
local git repository is created, and a default `.gitignore` file that determines
which files should not be pushed to git repository (for example the target
binaries). After this the repository could then be pushed to remote server for
sharing the work.

`cargo build` compiles the project into binary executable. `cargo run` then
executes the binary (that can also be located under `target` directory tree). If
sources have been changed when executing `cargo run` also re-compiles the
project.

There are many more features in cargo, of which some we will discuss a bit
later, e.g. related to testing. There is a separate **[online Cargo
book](https://doc.rust-lang.org/cargo/index.html)** discussing different
features in detail.

When your program grows, you will want to split it into multiple logical modules
and source files with specifically defined public interfaces that hide the
implementation details. [Rust book chapter
7](https://doc.rust-lang.org/stable/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
discusses thoroughly the concepts of **packages** and **crates** and how the
project is organized into **modules** of specific file hierarchy.

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
The argument specifies the host to connect, and destination port at that host.
The host can be a DNS name, in which case a DNS name resolution is made
first to find the actual destination IP address, or it could be directly an IP
address. A full documentation of the **TcpStream** is given in [Rust
documentation](https://doc.rust-lang.org/stable/std/net/struct.TcpStream.html),
including some examples.

The `connect()` function returns a **Result** enum, i.e. it can fail for different
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

The below diagram shows how this message exchange progresses over time: the DNS
request is made to a separate DNS server configured in the client system. After
(if) the request is successful, TCP's three-way handshake is started, and if it
completes successfully, the socket can be used for reading and writing. The
client system, chooses an available local TCP port automatically (in this
example, port 51782). The diagram also shows the server-side functions called,
which we will discuss in more detail in the next module.

![TCP connection establishment](/images/basics-tcpconnect.svg){: width="90%" .center-img }

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
in the "[examples folder](https://github.com/PasiSa/pronets/tree/main/examples)"
course material git repository. The example contains all needed Rust project
files to build and try it. One way to test this and other examples is to clone
the course material repository to your own machine and try it out using normal
Rust build tools.

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
Rust defines behavior that different types can provide. This is similar to
_interface_ in Java or _pure abstract class_ in C++. In this example,
`TcpStream` implements both of these traits: it can be read from, because bytes
can arrive from the network, and it can be written to, because bytes can be sent
through the connection.

The `Read` trait provides functions for receiving bytes from a stream. For
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

Numbers and characters are stored in computer memory as binary data. One byte
(or octet) consists of 8 bits. It can represent unsigned integer values from 0
to 255, or signed integer values from -128 to 127.

### Numbers and byte order

Larger integer values, such as 16-bit, 32-bit and 64-bit values, consist of
multiple bytes. These bytes can be ordered in memory in different ways. In
**big endian** byte order, the most significant byte is stored first, before the
less significant bytes. In **little endian** byte order, the least significant
byte is stored first. Most current desktop and server systems, including x86
processors and current Apple chips, use little endian byte order internally.

Network protocols normally use big endian byte order for binary integer values.
For this reason, big endian is also called **network byte order**. The byte
order used by the local computer is called **host byte order**. Depending on the
processor architecture, host byte order may be the same as network byte order,
but on most current systems it is little endian.

When binary integer values are sent over the network, they should therefore be
converted to network byte order before writing them to the socket. In Rust, the
integer types provide helper methods for this. For example, `to_be_bytes()`
converts an integer into a byte array in big endian byte order.

```rust
use std::io::{self, Write};
use std::net::TcpStream;

fn send_number(stream: &mut TcpStream, value: u32) -> io::Result<()> {
    // Convert the 32-bit integer to network byte order, i.e. big endian.
    let bytes = value.to_be_bytes();

    // Write all four bytes to the TCP connection.
    stream.write_all(&bytes)?;

    Ok(())
}
```

Conversely, when bytes are read from the network, they need to be interpreted
using the byte order defined by the protocol. If the protocol uses network byte
order, the received big endian bytes can be converted back into an integer with
`from_be_bytes()`.

```rust
use std::io::{self, Read};
use std::net::TcpStream;

fn receive_number(stream: &mut TcpStream) -> io::Result<u32> {
    // Read exactly four bytes, because a u32 value consists of four bytes.
    let mut bytes = [0u8; 4];
    stream.read_exact(&mut bytes)?;

    // Convert the received network byte order bytes into a u32 value.
    let value = u32::from_be_bytes(bytes);

    Ok(value)
}
```

### Handling data structures

Another issue is padding. When Rust compiles a normal struct, it may insert
unused bytes between fields so that the CPU can access the fields efficiently.
These padding bytes are part of the in-memory representation of the struct, but
they are normally not part of the network protocol message. In addition, the
structure fields that represent larger than 8-bit numbers need to be converted
into network byte order.

Our examples folder includes
"**[tcpheader](https://github.com/PasiSa/pronets/tree/main/examples/tcpheader/src/main.rs)**",
that converts a TCP header from a structure into standards-compliant byte array
that can then be written to network, and the opposite function of composing a
structure based in incoming byte array.

### Text and strings

When strings and text are sent over the network, they must first be converted
into bytes. For this reason, a protocol specification should define which
character encoding is used.

Traditionally, many text-based protocols used 7-bit ASCII. ASCII is simple
because each character fits into one byte, but it only supports a limited set of
characters. Today, international characters inside the text are common, and
UTF-8 is often used instead. UTF-8 represents Unicode text as a sequence of
8-bit bytes, while still keeping ordinary ASCII characters unchanged.

Rust strings are UTF-8 encoded, and Rust provides helper methods for converting
between strings and byte arrays when data is written to or read from the
network.

```rust
use std::io::{self, Write};
use std::net::TcpStream;

fn send_text(stream: &mut TcpStream, text: &str) -> io::Result<()> {
    // Rust strings are UTF-8, and as_bytes() returns the encoded bytes.
    let bytes = text.as_bytes();

    stream.write_all(bytes)?;

    Ok(())
}
```

## HTTP basics

Probably the most common application protocol in today's Internet is the
**Hypertext Transfer Protocol (HTTP)** that is used for transmitting web content
of top the TCP protocol, and as a general communication protocol, for example,
for various applications that use RESTful APIs. HTTP server can be located at
server port 80 for insecure connection with plaintext messages (readable e.g.
with Wireshark), or at port 443 (for TLS-encrypted connection, also known as
HTTPS). In practice these days implementations use practically always the secure
connection.

HTTP is a request-response protocol, where the HTTP client makes a request
containing one of the HTTP methods (GET, POST, PUT, etc.), some HTTP headers and
optionally a body. HTTP server replies with a response that contains a numeric
status code, and also has a header and a body.

HTTP was developed in 1991 by Tim Berners-Lee and few other researchers at CERN
(see [their publication](https://dl.acm.org/doi/abs/10.1145/179606.179671)), and
for majority of that time we have used its text-encoded versions 1.0 and 1.1. In
early 2010s, [HTTP/2](https://datatracker.ietf.org/doc/html/rfc7540) was
developed. It introduced few significant changes in how protocol is used,
paricularly, encoding the HTTP headers in binary format, thus taking less space.
Later became [HTTP/3](https://datatracker.ietf.org/doc/html/rfc9114) which is
based on the **QUIC protocol** that runs on top of UDP.

For the purpose of the assignments in this course, we will focus on the
text-based HTTP version 1, even though it is being phased out as the
newer versions are deployed, as it is easier to get started with.

A minimal HTTP/1.1 GET request asks the server to return a resource. The empty
line after the headers marks the end of the request headers.

In HTTP/1.1, each line in the request or response should end with the two-byte
sequence carriage return and line feed, written as `\r\n` in Rust strings. The
empty line after the headers is therefore also written as `\r\n`. In program
code, the request below would end with `\r\n\r\n`: one line ending for the last
header line, and another line ending for the empty line that terminates the
header section.

```http
GET /index.html HTTP/1.1
Host: example.com

```

A simple HTTP/1.1 response starts with the protocol version, status code and
status text. The headers are followed by an empty line and then the response
body. Content-Length tells the length of the body in the response. This is
needed, so that we know where this response ends and a new one starts. TCP is
stream-oriented protocol, so the `read` call to the socket does not necessarily
return a full response.

```http
HTTP/1.1 200 OK
Content-Type: text/plain
Content-Length: 13

Hello, world!
```

A POST request sends data to the server. In this case we apply JSON encoding in
the request body to pass different named attributes to the server.

```http
POST /new-user HTTP/1.1
Host: example.com
Content-Type: application/json
Connection: close

{
    "name": "Alice",
    "age": 30,
    "email": "alice@example.com"
}
```

<div class="assignment-frame" markdown="1">

## Assignment

This assignment consists of multiple parts. First you will implement a simple
TCP client that connects to "`www.aalto.fi`", port 80 and makes a GET request
for "`/index.html`". I.e., for this assignment we use plaintext HTTP, even though
it is strongly discouraged in reality. For this assigment, start a project in
your Git repository under folder "http-client".

1. Open Wireshark and capture packets that are destined to UDP port 53 or TCP
   port 80.

2. After your program has successfully established connection to the server,
   make it stop for user input before sending the actual HTTP request. You can,
   for example use the `read_line()` function (without processing the input) to
   do this.

Here is a simple example:

```rust
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
```

{:start="3"}

3. Can you locate the DNS request and response. Which type of DNS record does
   the response have and what does it mean?

4. Identify the TCP connection in Wireshark. What is the source TCP port used
   for the communication? What TCP options are visible in the initial SYN packet

After you have checked the DNS query and answer, press return and let the
program run forward, so that it makes the HTTP query and receives the
response that is printed to the standard output.

{:start="5"}

5. What is the status code in the response and what does it mean? What
   additional information are the headers telling?

Extend your program with another HTTP request. This time it is a POST request
that sends some information to the server, which causes it to fetch a Git
repository. The request should be sent to `pronets.dice.aalto.fi` and then
endpoint is "`/fetch-git`". We still use port 80 at server. You don't need to
analyze Wireshark from this point on, but keeping it open does not harm.

The POST request body should be JSON formatted (use Content-Type
`application/json`, as in example above), and it should have the following
keys:

- **"username"**: The username you want to use with the server. This should be
  the username you declared earlier.
- **"git-repo"**: The URL of the Git repository you are using for your course
  assignments and project.

{:start="6"}

6. Print the response that the server returns to the request. Note that the
   processing the request may take a short while, because the server actually
   tries to clone your repo.
   Was request successful according to response? If not, tell also that in your
   report, but try to fix the situation. Note that you should have at least one
   commit in your Git repo, so that the server can fetch it.

Finally, answer the following questions:

- How much time did you use for this assignment?

- What was easy or difficult in the assignment?

- What tools did you use? In particular, if you used AI assistants, tell how did
  you use then and if they were helpful.

</div>
