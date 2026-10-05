---
title: Concurrency and asynchronous programming
lang: en
translation_key: concurrency
---

This section dives into concurrent processing using **threads**. At server end,
multiple threads allow better parallelism in modern computer system, as the
operating system can schedule the different threads to be executed in parallel
on different CPU cores. At client threads can be used to implement network
communication without interrupting real-time user interaction on a graphical
user interface.

To support parallel execution under Rust's ownership rules, **shared reference
counted pointers** are introduced, along with **Mutexes**, that protect the data
integrity from parallel access.

Finally, we look into **asynchronous programming** with **Tokio**, where the concurrent
operations, e.g. with multiple clients, are distributed into lightweight tasks
that are operated and scheduled by the user-level Tokio runtime, instead of
operating system - scheduled threads.

<div class="objectives-frame" markdown="1">

**Objectives for this module:**

- You will **understand the concept of smart pointers** for dynamic data types or
  shared ownership, and know **how to apply them** in Rust.

- You will know how to **use threads to manage concurrent application logic**
  such that the operation of server and client progresses smoothly and
  efficiently.

- You will **understand how Mutexes work and how to use them together with smart
  pointers** to guard access to shared data when there are concurrent threads
  operating on data.

- You will learn to **apply asynchronous programming model**, particularly with the
  Tokio library as the asynchronous runtime.

</div>

## Smart pointers for memory management

For now we have used locally operated with variables and values that are
typically allocated in stack as program is executed, with the exception of the
`String` type or the container types that have used heap. Sometimes the local
allocation causes difficulties: if the program contains asynchronous or
concurrent logic with threads, managing the ownership would be difficult, if not
impossible with the basic mechanisms. On the other, in some situations we do not
know the actual size needed at compile time, for example, if we want to operate
objects with dynamic type. In such cases we need one of Rust's smart pointer
data types to place the value in heap, and with the help of the build-in smart
pointer mechanisms manage its lifetime.

### Allocating data from heap

Perhaps the most straight forward way of heap allocation is to use the `Box<T>`
type, that places its content into heap. The memory is released normally when
the Box type goes out of scope. The below simple example of how Box is assigned
and used, is copied from the [Rust book section
15.1](https://doc.rust-lang.org/stable/book/ch15-01-box.html) that discusses Box
in more detail.

```rust
fn main() {
    let b = Box::new(5);  // type: i32
    println!("b = {b}");
}
```

![integer stored to heap using Box](/images/concurrency-boxbasic.svg){: width="90%" .center-img }

The Box type is helpful when we don't know the exact size of the allocated data
at the compile time. Box can be used to implement object-oriented-like
polymorphism in Rust with the help of **Traits** that specify common interface
that different data types can implement. For example the `TcpStream` type
implements `Read` and `Write` traits that define common operations for reading
from and writing to a stream interface, that also some other stream-like types
could implement.

If we were implementing a whiteboard application, there could be trait
`WhiteboardObject` that specifies `draw()` method. Then we could have types
`Text`, `Rectangle`, and `Line` that implement the trait and the `draw()`
method. Using the `Box` type we can define a datatype for a variable to be
dynamic, based on a given trait. For example, below we have a vector that can
store any of the boxed `WhiteboardObject` elements:

```rust
trait WhiteboardObject {
    fn draw(&self);
}

struct Text(String);
struct Rectangle { width: u32, height: u32 }
struct Line { length: u32 }

impl WhiteboardObject for Text {
    fn draw(&self) { println!("Text: {}", self.0); }
}

impl WhiteboardObject for Rectangle {
    fn draw(&self) { println!("Rectangle: {} x {}", self.width, self.height); }
}

impl WhiteboardObject for Line {
    fn draw(&self) { println!("Line: length {}", self.length); }
}

fn main() {
    let objects: Vec<Box<dyn WhiteboardObject>> = vec![
        Box::new(Text(String::from("Hello!"))),
        Box::new(Rectangle { width: 100, height: 50 }),
        Box::new(Line { length: 80 }),
    ];

    for object in &objects {
        object.draw();
    }
}
```

Even though the vector is specified to hold `Box<dyn WhiteboardObject>` objects,
the actual objects stored in heap are Text, Rectangle or Line types, each
with different size in memory. The `dyn` keyword enables dynamic dispatch:
calling `draw()` selects the implementation for the concrete object at runtime.
For simplicity, the example just prints something, instead of doing the actual
graphics.

![Memory layout of vector with boxed dynamic elements](/images/concurrency-box.svg){: width="100%" .center-img }

[Chapter 18 of the Rust
book](https://doc.rust-lang.org/stable/book/ch18-00-oop.html) discusses Rust's
Object-oriented programming features in more detail.

### Reference counted pointer: Rc

In some cases Rust's ownership rules may be limiting, and the ownership needs to
be shared between multiple users. In particular, this is often needed in
asynchronous programming, or when working with concurrency and threads where
timing of events and access to common data is not always deterministic. If we
are sharing data in a multithreaded program, how can we know when its memory can
be released? Even if one thread is at the end of execution scope and would be
ready to release the data, there may be other threads still needing it.

The `Rc<T>` type stores its data in heap, but differing to `Box`, the heap
reference can be shared between multiple owners. Access to the data can be
granted to a new owner, while retaining the old ownership(s) by using the
`clone()` function. In addition to data reference, the `Rc` type maintains
**reference count** of the owners having access to the data (which explains the
type name). The reference count is increased by each call to `clone()`, and
decreased as the different `Rc` variables run out of scope. When the last
instance does this, the reference count falls to zero, and the data can be
actually freed from memory.

For example, two variables can share ownership of the same string:

```rust
use std::rc::Rc;

fn main() {
    let message = Rc::new(String::from("Hello!"));
    println!("Owners: {}", Rc::strong_count(&message)); // 1

    {
        let shared_message = Rc::clone(&message);
        println!("message: {}   shared: {}", message, shared_message);
        println!("Owners: {}", Rc::strong_count(&message)); // 2
    } // shared_message is dropped here

    println!("Owners: {}", Rc::strong_count(&message)); // 1
} // The last owner is dropped, freeing the string
```

`Rc::clone(&message)` creates another owner of the same heap allocation; it does
not copy the string itself. The original `message` remains usable after
`shared_message` goes out of scope.

![Shared string](/images/concurrency-rc.svg){: width="100%" .center-img }

You can find more information about the `Rc` type in [section 15.4 of the Rust
book](https://doc.rust-lang.org/stable/book/ch15-04-rc.html).

### Atomic reference counting: Arc

The `Rc` type is not safe against concurrent access, so it is only usable in
single-threaded use cases. When ownership needs to be shared across multiple
threads, we need to use **atomic reference counting**, the `Arc<T>` type. Apart
from being an atomic variant of reference counting, `Arc` works in the same way
as the `Rc` type.

## Threads

Modern operating systems organize the different applications and services as
**processes** that are allocated an isolated (virtual) memory and other
resources as arranged by the operating system, and are executed in parallel as
scheduled by the operating system. Because of the strong isolation arranged by
the operating system, programs can be designed and implemented safely without
interfering each other (at least as long as there are resources available in the
system), unless some form of inter-process communication is used (such as
sockets).

Within the process the execution can be distributed into separate execution
units, **threads**. Threads within the same process share the same resources,
such as heap memory and program code, but are executed and scheduled separately
by the operating system. Different threads can be scheduled to be executed by
different CPU cores available in the system, and therefore with threads we can
achieve a true parallel execution, and increased efficiency, e.g. on busy
servers. Each thread manages its own call stack and registers, for example the
local variables and execution context.

Threads are one way of achieving timely access at a network server that needs to
process multiple clients at the same time without delays. If a separate thread
was assigned for serving each client, blocking socket I/O operations (e.g.,
`read()`) only block the current thread without affecting others. In some cases
the server implementation logic becomes simpler compared to the iteration loop
of poll-based server with Multiplexed I/O -- at least until we need to share
data between the threads.

The following server starts a separate thread for every accepted connection. The
main thread can therefore continue waiting for new clients even while the
existing sockets are blocked waiting for data.

A new thread is started using the `spawn()` function. Its parameter is unusual
to what we have seen before. The parameter is another function that is run
inside the new thread. Instead of named function, a common practice, as done
here, is to use **closure**, an unnamed function specified inline with the
`spawn()` call. A closure can capture variables from its surrounding scope, like
here we are doing for the `stream` variable that contains the just-accepted
client socket. In the context of threads, `move` keyword is used here. It tells
that the ownership of captured variables (i.e., `stream`) moves inside the
closure, and this case inside the thread. The main thread cannot use it any
more, and when spawned thread completes, the stream is released. Closures are
explained more throughly in [Section 13.1 of the Rust
book](https://doc.rust-lang.org/stable/book/ch13-01-closures.html).

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

[Chapter 16 of the Rust
book](https://doc.rust-lang.org/stable/book/ch16-00-concurrency.html) discusses
concurrency and threads in more detail, with additional examples. The examples
folder has also example code
**[threaded-server](https://github.com/PasiSa/pronets/blob/main/examples/threaded-server/src/main.rs)**
that demonstrates separate thread per incoming client.

Because operating system schedules the threads, and they may be run concurrently
by multiple CPUs, the mutual order of events between threads is undeterministic.
It is common that different threads also need to operate on a shared state, or
need to pass information between them. Access to the shared information needs to
be atomic, so that the data structures remain at consistent state even when
potentially large number of threads are operating on them. Rust aims to protect
data against concurrent access, and has a few tools managing this, as discussed
next.

## Mutexes for concurrent data access

As discussed above, ownership of data can be shared using reference counted
heap-allocated variables. The data continues to exist in heap as long as there
is at least one reference to the variable, and when the last instance runs out
of scope, and the reference count falls to zero, the data and the memory it used
is released. With this method, we can share common data also across multiple
threads, using the `Arc<T>` type.

The `Arc` type alone is not usually sufficient, however, when operating with
multiple threads. We must ensure that only one thread is accessing the data at a
time (and the Rust compiler tries to help programmer by requiring this).
Therefore the `Arc` - pointed data needs to be wrapped inside a `Mutex` type
(for **mutual exclusion**) to guard the data against concurrent access. Mutex
maintains a lock that tracks who is currently accessing data, does not allow
other access until the previous user is finished.

`Mutex` uses the `lock()` method to guard the access to the data it holds. If
the data was free, the method returns a mutable reference to data encapsulated
in `MutexGuard` type, that can be used to read and modify the data. If someone
had already acquired the lock, the `lock()` call blocks until the lock is
released. Following the typical style in Rust, the lock is released when the
MutexGuard variable runs out of scope. The following picture illustrates how the
memory layout might look like when shared state _MyState_ is shared by two
threads using `Arc<Mutex<MyState>>` variables. The green region illustrates
**Arc**, the grey region illustrates **Mutex**, and the purple area is the
actual shared data. Note that the implementation details of these types may
actually differ, the names on the diagram are illustrative and simplified.

![Reference counted shared data protected by mutex](/images/concurrency-mutex.svg){: width="100%" .center-img }

The below example is slightly modified from the example in [Section 16.3 of the
Rust book](https://doc.rust-lang.org/stable/book/ch16-03-shared-state.html).

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
            // lock().unwrap() returns a MutexGuard, so `value` holds the guard.
            // The * operator (dereference) is needed to access the protected counter.
            let mut value = clonedcounter.lock().unwrap();
            *value += 1;
            // Mutex is automatically unlocked when `value` goes out of scope
        });

        handles.push(handle);
    }

    // Wait for all threads to finish: join() blocks the current thread until
    // the thread represented by the handle finishes, returning Err if the
    // thread panicked.
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", *counter.lock().unwrap());
}
```

Working with mutexes and locks introduces the risk of deadlocks, which can be
difficult to debug as programs grow in complexity. For example, one thread may
hold a lock while waiting for an event that another thread can produce only
after acquiring the same lock. Neither thread can proceed, so both wait
indefinitely. To reduce this risk, one should keep critical sections short and
avoid blocking operations while holding a lock. When multiple locks are needed,
the program should acquire them in a consistent order across threads.

## Message channels for communicating between threads

Another method of sharing data between threads is to use **message passing**.
Rust's standard library offers a type called `mpsc` (for "multiple providers,
single consumer") channels. When channel is created it provides separate handles
for receiving messages and sending messages. When then a new thread is spawned,
one of the handles can be moved to the new thread, while other end stays with
the main thread. The channel can be used to pass any values of any type. When
the the sender sends the value (using `send()` call), also its ownership is
transferred to the receiver (who takes it using `recv()` call). Therefore the
value cannot be used anymore after sending it. [Rust book's Section
16.2](https://doc.rust-lang.org/stable/book/ch16-02-message-passing.html)
discusses message passing in more detail.

One use case for message passing could be, for example, when implementing an
interactive client. The main thread could be responsible for providing user
interface and smooth interaction with the user, maybe with real-time animations
in case of a simple game. Then, a second thread could be spawned to manage the
communication with the server. If communication blocks to wait for new data in
the communication thread, it would not harm the main thread and its real-time
interaction. Then, a message channel can be used to pass the status updates from
the socket to the main thread that does the needed adjustments in the user
interface.

![Connecting threads using channel](/images/concurrency-channel.svg){: width="90%" .center-img }

The simplified example below simulates waiting for server updates using
`sleep()`. In a real client, the communication thread would read from a socket.
The main thread uses `try_recv()` to check for updates without blocking, so it
can continue updating the user interface while no messages are available.

```rust
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

fn main() {
    let (sender, receiver) = mpsc::channel();

    // moving the sending end of channel to the new thread
    let communication = thread::spawn(move || {
        for status in ["Connected", "New message", "Server disconnected"] {
            // Simulate a blocking socket read in the communication thread.
            thread::sleep(Duration::from_millis(500));
            sender.send(String::from(status)).unwrap();
        }
        // Dropping the sender disconnects the channel.
    });

    loop {
        match receiver.try_recv() {
            Ok(status) => println!("Update interface: {status}"),
            Err(TryRecvError::Empty) => {} // No update yet; keep running.
            Err(TryRecvError::Disconnected) => break,
        }

        // Handle user input and draw the next animation frame here.
        // Simulate the pause between frames to avoid a busy loop.
        thread::sleep(Duration::from_millis(16));
    }

    // Wait for the communication thread to finish.
    communication.join().unwrap();
}
```

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
light-weight and efficient. In practice, it is often best to apply a combination
of both: leverage pool of multiple threads to the extent that one can make good
use of the available CPU cores, but within each thread, apply also asynchronous
programming model.

Asynchronous programming in Rust is based on futures: values implementing the
**Future** trait that represent operations which may complete later, for
example, when an operation needs to wait for data to arrive over a network.
Calling a function declared with `async`, or evaluating an `async` block,
creates a future. Its body does not execute until the future is polled, meaning
that it is asked to make progress. An asynchronous runtime is responsible for
this polling process.

Inside an `async` function or block, `.await` waits for a future to complete and
produces its output value. If the future is not ready, the current task can
pause, allowing the runtime to run other tasks on the same thread. If the future
is ready immediately, no pause is needed. Operations such as asynchronous socket
reads and asynchronous mutex locking use this mechanism to wait without blocking
the thread. Adding `.await` does not turn an ordinary blocking operation into an
asynchronous one, but the operation must provide an asynchronous interface.

The Rust book [Section
17.1.](https://doc.rust-lang.org/stable/book/ch17-01-futures-and-syntax.html)
discusses more about this.

### Asynchronous runtime and non-blocking I/O using Tokio

To implement asynchronous programs, in addition to programming language
primitives the program needs to have a runtime for asynchronous tasks. There
are a few scheduler libraries available, but here we discuss
**[Tokio](https://crates.io/crates/tokio)**, that is one of the most used
libraries for this purpose. Tokio supports multithreading with asynchronous
programming and provides asynchronous versions of the common standard library
functions.

An example of a simple server written using Tokio is given in
[examples](https://github.com/PasiSa/pronets/blob/main/examples/async-server/src/main.rs).
With Tokio, the `main()` function is defined as asynchronous and there is a
`#[tokio::main]` attribute in front of the function definition. Attributes are
processed by Rust compiler and cause some code to be generated around the
function. In this case the attribute creates a Tokio runtime that runs an async
version of the main function. The runtime schedules the asynchronous tasks and
monitors the status of I/O operations and timers, to resume tasks when progress
is possible.

A good **[tutorial of Tokio](https://tokio.rs/tokio/tutorial)** is given on the
official web site. Borrowing the tutorial, it is good to point out what actually
happens when the `main()` function is wrapped with `#[tokio::main]` attribute,
to show how its magic works. The following _main_ implementation:

```rust
#[tokio::main]
async fn main() {
    println!("hello");
}
```

gets transformed to the following by the compiler:

```rust
fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        println!("hello");
    })
}
```

The `block_on()` function takes another function as a parameter that gets
executed until the first instance of `.await` comes across, after which the
runtime finds another task to continue execution from. Internally, Tokio uses
e.g. the **Mio** library to poll for available I/O sources when scheduling the
tasks.

When an operation such as `socket.read(&mut buf).await` cannot complete
immediately, the client task pauses and Tokio can run other tasks. When the
socket becomes ready for reading, Tokio schedules the task to continue. This
allows many connections to share a small (but sufficient in terms of optimal
parallelism) number of worker threads.

The interface for spawning a task looks very similar to spawning a thread:

```rust
task::spawn(process_client(socket, address));
```

The behavior is different, though. There is no thread created, which would
require OS interaction, but Tokio just creates a lightweight task, and the
accept loop can continue to wait for the next connection.

It is good to note that when using Tokio, the program needs to use
**different versions of the I/O data types** such as `TcpListener` or
`TcpStream` that work asynchronously and co-operate with the Tokio runtime.
Therefore, in the beginning of the example we see that the actual types used are
`tokio::net::TcpListener` and `tokio::net::TcpStream`, instead of Rust's
standard library versions.

Finally, **ownership still matters**, also with asynchronous tasks. The example
moves the ownership of the socket to the new task, after which the main loop
cannot use it anymore. For data that needs to be accessed by different tasks, we
may still need reference counted smart pointers. Because the Tokio runtime can
distribute tasks to multiple threads, we'd need to use `Arc` similarly to
threads, and mutexes to guard against simultaneous access. Also for `Mutex`
there is a Tokio-specific version `tokio::sync::Mutex` that supports `.await`
with the `lock()` function to yield if lock is not available, then to be later
scheduled by the Tokio runtime when the lock can be acquired.

### Writing client/server tests using Tokio

When the implementation uses Tokio for managing concurrency, it is good to use
Tokio also in tests that involve client-server interaction. This allows nice way
of creating e.g. integration tests.

The [`#[tokio::test]`](https://docs.rs/tokio/latest/tokio/attr.test.html)
attribute creates a Tokio runtime for an asynchronous test. This allows us to
run a server in a separate task while the test itself acts as a client. For
example, the following test checks that a TCP server echoes the client's data.
The following could be an integration tests in imaginary program.

```rust
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{timeout, Duration};

#[tokio::test]
async fn server_echoes_client_data() {
    timeout(Duration::from_secs(5), async {
        // Bind before connecting; port 0 lets the OS choose an available port.
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buffer = [0; 1024];
            loop {
                let count = socket.read(&mut buffer).await.unwrap();
                if count == 0 {
                    break; // The client has finished sending.
                }
                socket.write_all(&buffer[..count]).await.unwrap();
            }
        });

        let mut client = TcpStream::connect(address).await.unwrap();
        let message = b"Hello, Tokio!";
        client.write_all(message).await.unwrap();
        // Close only the sending side so we can still receive the echo.
        client.shutdown().await.unwrap();

        let mut response = Vec::new();
        client.read_to_end(&mut response).await.unwrap();
        server.await.unwrap();
        assert_eq!(response, message);
    })
    .await
    .expect("client/server test timed out");
}
```

The `async move` block transfers ownership of the listener to the server task.
Both tasks can progress on the default single-threaded test runtime because
waiting for network I/O yields to Tokio. Note that in tests it is good to bind
to port 0 at server, meaning that system chooses port, and then query for the
chosen port, using the `local_addr()` method. If we'd choose a fixed port, in
unlikely case someone might have reserved it, and test would fail for that
reason, which is not the desired outcome.

TCP is a byte stream, so one `read()` may receive only part of the message.
The server therefore loops and echoes each received chunk using `write_all()`.

This example uses a feature of TCP sockets we haven't discussed yet: connection
can be "half-closed", i.e. one of its directions can be closed early using the
`shutdown()` method, while other direction is still available. It is not a
commonly used feature, but used in this test: after the client shuts down its
sending side, the server reads EOF and exits, dropping its socket. The client's
[`read_to_end()`](https://docs.rs/tokio/latest/tokio/io/trait.AsyncReadExt.html#method.read_to_end)
collects the response until that socket closes. Awaiting the server's task
handle also checks that it completed without panicking, and the final assertion
verifies that the echoed bytes match the original message. The timeout makes the
test fail if the exchange stops making progress.

<div class="assignment-frame" markdown="1">

## Assignment #6

**Part 1:** Redesign your server to use **Tokio** and the async/await model. If
you have progressed with your project implementation, and don't feel comfortable
rewriting the code, or don't want to proceed with Tokio towards the end of the
project, you can also choose make an alternative Tokio-based server binary
specifically for this assignment that implements the handling of the base
protocol messages. After your implementation works, **discuss in the report**
which of the different approaches seems best server design for your project, and
why.

**Part 2:** Enhance the test client from last assignment so that it opens (at
least) five concurrent connections to the selected server to test the different
use cases you have implemented. Implement this by spawning a separate thread for
each connection. Measure and output also the time each thread uses from start to
finish, using e.g. the time keeping functions presented in the [previous
module](../testing/#different-kinds-of-metrics). Test the updated test client
against two server listed at the [course
server](https://pronets1.dice.aalto.fi/). Report your findings.

Include also the following information:

- How much time did you use for this assignment?
- What was easy or difficult in the assignment?
- What tools or other information sources did you use? In particular, if you
  used AI assistants, tell how did you use them and if they were helpful.

</div>
