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

### Mutexes for concurrent access

See text about Mutexes and Atomic Reference Counting in [Rust book section
16.3](https://doc.rust-lang.org/stable/book/ch16-03-shared-state.html).

## Asynchronous programming

### Asynchronous programming model

See text about async/await and Futures in [Rust book section
17.1.](https://doc.rust-lang.org/stable/book/ch17-01-futures-and-syntax.html)

### Asynchronous runtime and non-blocking I/O using Tokio

_TBD: Tokio, async, await, etc_

## Spawning multiple processes

_TBD: maybe skip this section?_

## Assignment

Test multiple concurrent clients on a message channel

Implement `OBJ channel name data`, where _data_ can be any binary data of size bytes.
Size is a 32 bit unsigned integer in network byte order, immediately followed by
that many bytes.

Write a test program that starts 5 clients in parallel that connect to server as
separate users. Each client should send one megabyte of data at the same time.
Measure and report completion times for these flows. What if these clients write
chat messages at the same time to a communication channel.
