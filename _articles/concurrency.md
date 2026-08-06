---
title: Concurrency and asynchronous programming
---

## Smart pointers for memory management

### Allocating data from heap

See text about allocating data from heap and Box type in [Rust book section
15.1](https://doc.rust-lang.org/stable/book/ch15-01-box.html).

### Reference counted variables: Rc and Arc

See text about reference counted variables in single-threaded cases using Rc
type in [Rust book section
15.4](https://doc.rust-lang.org/stable/book/ch15-04-rc.html).

Mention RefCell here.

Mention atomic reference counting already here.

## Threads

Modern operating systems organize the different applications and services as
**processes** that are allocated an isolated (virtual) memory and other
resources as arranged by the operating system, and are executed in parallel as
scheduled by the operating system. Because of the strong isolation arranged by
the operating system, programs can be designed and implemented safely without
interfering each other, unless some form of inter-process communication is used
(such as sockets).

Within the process the execution can be distributed into **threads**. Threads
within the same process share many of the same resources, such as heap memory,
but are executed and scheduled by the operating system, leveraging the number of
CPU cores available at the underlying computer system. Each thread manages its
own call stack and registers, however, e.g. the local variables and execution
context. Threads are a convenient way of implementing parallelism, for example
at a network server that handles multiple clients at the same time, or an
interactive application that needs to react to user input and network events at
the same time. For example, calling socket operations (such as `read()`) that
may block in some situations, distributing the programming logic to separate
threads can keep other connections still operational, because the blocking
applies only on the current thread.

The parallelism of threads also causes programming challenges, when there is
state and variables that need to be accessed across operations in different
threads. Because operating system takes care of the scheduling, the order of
execution of different operations and functions cannot be known at the
programming time, and careless access of shared resources can cause unwanted
consequences and corrupted state, if not designed carefully. Rust aims to help
the programmer in avoiding mistakes through its ownership model and operations
intended for accessing shared resources securely.

The following server uses a separate thread for every accepted connection. The
main thread can therefore continue waiting for new clients even while an
existing client is blocked waiting for data. The `move` keyword on line 31
transfers ownership of the connected socket (`stream` variable) to the new
thread. The `spawn()` function also uses **closure**, a method that allows
passing functions as arguments to other functions. There is more information
about closures in Rust book [section
13.1](https://doc.rust-lang.org/stable/book/ch13-01-closures.html).

```rust
use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

// Read data from one client and echo it back until the client disconnects.
fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            return Ok(());
        }

        stream.write_all(&buffer[..bytes_read])?;
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:2000")?;
    println!("Listening on 0.0.0.0:2000");

    for connection in listener.incoming() {
        match connection {
            Ok(stream) => {
                let address = stream.peer_addr()?;
                println!("Accepted connection from {address}");

                thread::spawn(move || {
                    if let Err(error) = handle_client(stream) {
                        eprintln!("Connection to {address} failed: {error}");
                    }
                });
            }
            Err(error) => eprintln!("Failed to accept connection: {error}"),
        }
    }

    Ok(())
}
```

The Rust book [chapter
16](https://doc.rust-lang.org/stable/book/ch16-00-concurrency.html) discusses
concurrency and threads in more detail, with additional examples.

Also the examples folder has an example of **[threaded
server](https://github.com/PasiSa/pronets/blob/main/examples/threaded-server/src/main.rs)**
you want to try out in practice.

### Mutexes for concurrent access

We saw earlier how ownership of a variable can be shared using reference counted
heap-allocated variables. The variable continues to exist as long as some one is
using the variable, and when the last instance runs out of scope, the memory for
the data is then released.

When a multithreaded programs uses shared state, we need to be careful that the
modifications to the data must be done atomically and only by single thread at a
time, to avoid confusion with possible parallel write operations.

A common way to ensure that only one thread modifies the share state at a time
is to use **Mutexes**, locks that block the thread if someone is currently
accessing the critical region. When the earlier owner of the lock exits the
locked scope, the next thread in line can access the data. Mutex type is defined
with a data item that needs to mutable access in the program (if mutability is
not needed, we do not need Mutexes either). The Mutex object itself needs to be
assigned as an **atomic reference counted** variable, "**Arc**".

Below is a simple example of how this happens.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Shared state: a counter protected by a mutex.
    let counter = Arc::new(Mutex::new(0));

    // Vector for thread handles.
    let mut handles = Vec::new();

    for _ in 0..4 {
        // Clone the Arc to share ownership between threads.
        // The Arc type maintains reference count of the number of copies of the data.
        let clonedcounter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            // Lock the mutex before accessing the shared data.
            let mut value = clonedcounter.lock().unwrap();
            *value += 1;
            // Mutex is automatically unlocked when `value` goes out of scope
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", *counter.lock().unwrap());
}
```

See text about Mutexes and Atomic Reference Counting in [Rust book section
16.3](https://doc.rust-lang.org/stable/book/ch16-03-shared-state.html).

## Asynchronous programming

When operating with threads, the programmer can forget about scheduling and
parallelism, as long as the write access to shared data is taken care of.
However, excessive use of threads causes overhead to operating system that may
become expensive with large number of concurrent actions, for example on a
server with large number of client sessions. Benefit of threads is, however,
that they can make good use of the multiple CPU cores typically available in the
system.

Another approach is to apply asynchronous programming model, where thread-like
parallelism can be applied within a single thread using a asynchronous runtime
that runs aside the application program to schedule tasks in the same thread.
Because this does not involve operating system scheduling, it is more
light-weight and efficient. In practice, the best approach is often a combination
of both: leverage pool of multiple threads to the extent that one can make good
use of the available CPU cores, but within each thread, apply also asynchronous
programming model.

Asynchronous programming is based on a **Future** data type that encapsulates
variable whose value may not be determined immediately when executing a program
block, but there may be delay because of some blocking operation, as we commonly
have in network programming. Such program block or function is marked with
`async` key word which tells the Rust compiler that the operation returns a
Future that may not be determined immediately, but after some delay.

In asynchronous programming blocking operations, such as socket I/O are suffixed
with a `await` keyword that tells that the function call returns a Future that
may be incomplete at return, and the operation should return here at a later
time when something interesting is available.

The Rust book [Rust book section
17.1.](https://doc.rust-lang.org/stable/book/ch17-01-futures-and-syntax.html)
has more details about how this works.

### Asynchronous runtime and non-blocking I/O using Tokio

To implement asynchronous programs, in addition to programming language
primitives the program needs to have a scheduler for asynchronous tasks. There
are a few scheduler libraries available, but here we discuss
**[Tokio](https://crates.io/crates/tokio)**, that is one of the most used
libraries for this purpose. Tokio supports multithreading with asynchronous
programming and provides asynchronous versions of the common standard library
functions.

An example of a simple server written using Tokio is given in
[examples](https://github.com/PasiSa/pronets/blob/main/examples/async-server/src/main.rs).
With Tokio, the `main()` function is defines as asynchronous and there is an
**attribute** `#[tokio::main]` in front of the function definition. Attributes
are processed by Rust compiler and cause some code to be generated around the
function. In this case this embeds the main function with the Tokio scheduler.

In the example you can see that many of the network function calls that may
block, are supplied by the `await` keyword. This makes it possible for them to
temporarily return with unfinished Future, to give turn to other tasks, before
the current call can actually complete with return value.

## Spawning multiple processes

_TBD: maybe skip this section?_

## Assignment

Test multiple concurrent clients on a message channel

Implement `OBJ channel name data`, where _data_ can be any binary data. Length
is determined by the size in the common frame header.

Write a test program that starts 5 clients in parallel that connect to server as
separate users. Each client should send one megabyte of data at the same time.
Measure and report completion times for these flows. What if these clients write
chat messages at the same time to a communication channel.
