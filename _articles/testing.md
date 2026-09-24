---
title: Testing and observability
---

This module looks into analysis of software in various ways. In small programs
it is sufficient to use `println!` debugging or debugger tools, but when
software gets larger, it needs more sophisticated tools for analysis. In
particular, when a server is deployed, and intended to run continuously for long
periods of time, it is important to be able to monitor its performance, and
possible deviations from expected behavior, so that the system operator can
react and take possibly needed actions to correct problems.

<div class="objectives-frame" markdown="1">

**Objectives for this module:**

- You will **learn to divide an implementation into modules** for better
  maintainability, which also helps in organizing testing.

- You will **understand different forms of testing**, particularly **unit
  tests**, **integration tests** and **end-to-end tests**, and how Rust supports
  these.

- You will learn to **use logging and tracing tools** as part of your Rust
  program, to support diagnosing the behavior of the implementation.

- You will **understand different kinds of metrics** and how they can be used to
  analyze the software performance over time, while it has been deployed for
  use.

- You will get a **overview of monitoring tools** and how the leverage the metrics
  collected by the software.

</div>

## Organizing the code to modules

Good software design principle is that the `main.rs` that adds the executable
crate or program should be kept quite short, and most of the functionality is
split into modules that are separate files under the `src/` directory. In
addition the modules should be listed in the beginning or end of the `main.rs`
file, or a `lib.rs` file that is used by library crates, in the following way,
for example:

```rust
mod channel;
mod client;
mod metrics;
```

This could be a structure for a network server, that has **client** module for
encapsulating logic specific to particular client; **channel** module for
encapsulating logic related to chat channel; and **metrics** module for managing
the server metrics. In larger software, modules can be organized in a hierarchy,
as subdirectories under `src/`. Rust book [section
7.5](https://doc.rust-lang.org/stable/book/ch07-05-separating-modules-into-different-files.html)
talks about this in more detail.

In addition, a good modular design could be to start the client module by
specifying a `Client` structure, and implementing its operations as methods
operating on the structure (as briefly discussed in Module 2), for example:

```rust
/// Represents one client connection to the server.
pub struct Client {
    stream: Option<TcpStream>,
    handle: ClientHandle,
    username: Option<String>,
    // ...and probably some other fields...
}

impl Client {
    pub fn new(stream: TcpStream) {
        // constructor for a client object here
    }

    pub fn process_msg(&mut self, message_id: u32, length: u32) -> std::io::Result<()> {
        // implementation here
    }
    // ...other functions...
}
```

The structure and methods can then be accessed from other modules by including
it in the beginning of the module (`crate::` prefix can be left out in
`main.rs` that is at the root of the namespace):

```rust
use crate::client::Client;
```

Note that the structure must be specified as **public** to be accessed from
other modules, as do the methods.

## Testing

Tests check software at different levels. **Unit tests** verify individual
functions or small pieces of logic in isolation. **Integration tests** check
that several components work correctly together. **End-to-end tests** run the
complete application from a user's or client's perspective, for example by
connecting to a running server, sending a request, and checking the response.
Adding tests and improving existing ones should be seen as part of software
development process as new functionality is being developed. Whenever a new
feature is added to the software, it is a good habit to write tests for it.

Rust includes support for automated unit tests and integration tests. After some
tests have been implemented, running `cargo test` goes through the tests in the
project and runs them. Before each git commit, it is a good practice to try that
all tests pass, and if not, do the needed corrections. See [Rust book section
11.1](https://doc.rust-lang.org/stable/book/ch11-01-writing-tests.html) for more
complete description about how to write tests.

Common Git services such as Gitlab and Github provide support for automated
workflows that can be run, e.g., for each push event or merge request (a way for
developers to propose code changes to main branch). These can be used to
implement **Continuous Integration (CI)**, i.e., automatically execute the tests
and code format checking, for example as a precondition to accept the push
event.

### Assert macro

Assertions check that a condition or result matches what we expect. If an
assertion fails, it causes a panic that causes the test fail. The exclamation
mark (`!`) indicates that these are macros. The most common forms of assertion
are:

- `assert!(condition)`: checks that a boolean condition is true, for example
  `assert!(!encoded.is_empty())`.
- `assert_eq!(actual, expected)`: checks that two values are equal, for example
  `assert_eq!(encoded.len(), 9)`.
- `assert_ne!(actual, unexpected)`: checks that two values are different, for
  example `assert_ne!(encoded.len(), 0)`.

The equality and inequality macros show both values when the assertion fails,
which helps identify the problem. All three forms also accept an optional
formatted message to explain a failure, for example
`assert_eq!(encoded.len(), 9, "unexpected length: {}", encoded.len())`.

### Unit tests

**Unit tests** typically test that a single function or functionality works as
expected. In Rust, unit tests are placed at the end of the same program module
(i.e., `*.rs` file) as the actual implementation it is testing. When your
program grows, it is good to start splitting it into separate logical modules,
to make designing tests also easier.

Here is an example of a simple function that adds a length field in the
beginning of message and a unit tests that follows it:

```rust
/// Function that adds a length field in front of the message passed as
/// an argument
pub fn encode_message(payload: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(4 + payload.len());
    result.extend_from_slice(&(payload.len() as u32).to_be_bytes());
    result.extend_from_slice(payload);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Unit test that checks the functions adds a correct length header
    #[test]
    fn encodes_length_before_payload() {
        let encoded = encode_message(b"hello");

        assert_eq!(&encoded[..4], &[0, 0, 0, 5]);
        assert_eq!(&encoded[4..], b"hello");
    }
}
```

The test section of the program is started with `#[cfg(test)]` that tells the
Rust compiler that the following code is compiled only when building tests. The
tests are included in submodule `tests` under the current program module.
`#[test]` is a built-in Rust attribute that tells Rust's test runner that this
function is part of the tests that are run when running executing `cargo test`.

### Integration tests

**Integration test** checks that multiple software components and functions, and
their public interfaces work correctly to implement a particular feature.
Therefore, integration tests do not logically belong as part of some software
module, like unit tests, but are placed separately in source code. In Rust,
integration tests are placed in a separate `tests` directory under the top-level
project directory, aside the `src` directory where the actual code is.

Example
**[intergration-test](https://github.com/PasiSa/pronets/tree/main/examples/integration-test/)**
shows a simple server implementation split into two modules. The "**server**"
module contains function `handle_connection` that receives (ownership of) a TCP
connection, reads a message and writes its back, and then closes the connection
(because the TCP socket runs out of scope at the end of the function). The read
and write operations are done in a separate "**protocol**" module that the
server calls. Integration tests also require that the project has `lib.rs` that
links the modules to the same crate (even if there was also `main.rs`). Note
that this example works like a library crate: it does not have the `main()`
function or **main.rs**.

There is one integration test in `tests/roundtrip.rs` that opens a server socket
in a separate thread, and the connects the client socket to it, to test the
`handle_connection` function, and the interaction between the two modules.
Splitting the server and client implementations in separate threads is a fairly
common design in integration tests, that involve testing parts of the network
interaction.

You can try the test in the example by running `cargo test`. Note that you
cannot use `cargo run` because there is no `main()` function.

When a project consists of a workspace with multiple packages, like in our
projects for client and server, there are separate set of integration tests, and
the `tests` directory needs to be placed separately in both packages.

### End-to-end tests

**End-to-end tests** focus on the full application to tests its external
behavior. If the client is based graphical UI, this client behavior may be
difficult to analyze automatically some specific tools, but it is possible
possible to develop a test client that simulates the different client scenarios
to communicate with the server, and outputs the results on the command line.

In our project setup, we could first start the actual server binary, or the
Docker container, and implement a test client that operates on command line and
sends different message patterns with valid and invalid messages. The client
verifies that the responses from server are as expected and reports the outcome
on the command line.

By default, a binary Rust crate runs the `main()` function from the **main.rs**
source file under the `src` directory. It is possible to have alternative
binaries in a `bin` subdirectory under the `src` directory, that have their own
`main()` function. This way it is possible to have, for example, a test client
to run automated test sequence, when the main interactive (and perhaps
graphical) binary is in the primary **main.rs** of the **client** package.

When there are multiple binaries, the name of the binary needs to be specified
when executing it using `cargo`. If we had `bin/testclient.rs` in addition to
main.rs in client package, the options could be either

    cargo run -p client --bin client

to run the actual client from **main.rs** (the `-p` option tells we are
operating on the client package, and not the server), or

    cargo run -p client --bin testclient

to run our end-to-end tests.

## Logging and tracing

The usual straight-forward way to follow the program execution is to use
`println!` to output program events to console. As the server grows, it becomes
useful to control how much detail is recorded and to identify which connection
or request each message concerns. **Logging** records events during execution,
such as server startup, a client connecting, or a request failing.

### Logging levels

During development we often want detailed information about program execution.
When the program is in production, recording every step for every client creates
too much output. Logging levels allow us to select the correct level of detail.
In the following we will use the **tracing** crate, which provides the following
macros for different logging severity levels, from highest to lowest severity:

- **ERROR**, `error!`: A failure that needs attention, such as the server being
  unable to bind its listening socket or access required storage.
- **WARN**, `warn!`: An unexpected situation that the server can recover from,
  such as rejecting an invalid protocol message while continuing to serve
  other clients.
- **INFO**, `info!`: Important normal events, such as server startup or shutdown.
  Connection lifecycle events may also be useful at this level for our project.
- **DEBUG**, `debug!`: Details useful during development, such as the type and ID
  of a message being processed.
- **TRACE**, `trace!`: Fine-grained execution details, such as progress through
  individual parsing steps.

The severity level should be chosen according to the impact on the application.
Invalid client input does not necessarily indicate a server failure, and an
ordinary client disconnection is usually expected. Frequent events, such as
every successfully processed message, normally belong at DEBUG or TRACE level.

### Setting up tracing

The **tracing** library we show here consists of two parts. The `tracing` crate
produces the events and spans (explained shortly). A tracing **subscriber**
receives these, filters them, and determines how they are recorded. The
`tracing-subscriber` crate provides a subscriber for formatted text output. To
add tracing support, add the following dependencies to the server package's
`Cargo.toml`, under the `[dependencies]` section:

```toml
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

Tracing is set up in the beginning of `main()`, before most other program logic.
Here is a short example:

```rust
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;

fn main() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .init();

    info!("server starting");
    debug!("configuration loaded");
}
```

This setup reads the `RUST_LOG` environment variable and uses INFO level, if the
environment variable is now given. INFO includes INFO, WARN, and ERROR events in
the log, and is usually a good choice when running a server in production (or on
our course server). The `env-filter` feature and `.with_env_filter(...)`
configuration set up the filtering based on environment variable. If you are
interested in more details, you can check the [formatting subscriber
documentation](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/).

When running the server on the command line, you could use, for example, the
following to set the environment variable and logging level. At each level all
the messages with severity above the level are also shown, so the DEBUG logging
may contain a lot of output, if you have used that extensively.

```sh
RUST_LOG=info cargo run -p server
RUST_LOG=debug cargo run -p server
```

You can set the environment variable for the Docker container by adding the
following in the `Dockerfile`, before the CMD line that starts the server,
choosing the logging level as you wish:

```
ENV RUST_LOG=info
```

**Note:** The course server has an additional optional attribute for setting
environment variables with the `run-docker` POST request, if you want to
temporarily adjust, e.g., the logging level for some debugging purposes. The
attribute name to be added in JSON payload is `env`, and it takes an array of
strings as parameter, for example: `["RUST_LOG=debug", "SECRET_VALUE=xyz"]`.

### Events and structured fields

An **event** records something that happened at a particular point in execution.
Alongside a readable message, it can contain named **fields**. For example, this
function shows information about an already decoded message:

```rust
fn record_message(connection_id: u64, message_id: u32, message_type: &str) {
    tracing::debug!(
        connection_id,
        message_id,
        message_type,
        "message received"
    );
}
```

Even though the macros can be used like `println!` or `format!` for formatting
different parameters given to the calls, the **tracing** output methods allow
including selected variables in the output, shown as `name_of_variable = value`,
as shown above for `connection_id`, `message_id` and `message_type`. This
example assume that each accepted connection is assigned a numberic identifier
for logging purposes.

Note that you can skip the `tracing::` prefix from different log calls by adding
the following in the beginning of the file:

```rust
use tracing::*;
```

More details on formatting, and other specifics are explained in the [tracing
macro documentation](https://docs.rs/tracing/latest/tracing/#using-the-macros).

### Structured tracing and spans

When several clients communicate with a server, their events are interleaved.
**Structured tracing** adds context using **spans**: named operations with a
beginning and an end. A connection span can contain the events for that
connection, with a nested span for each request. Fields on the span identify the
operation without requiring every logging event to repeat them.

The `#[tracing::instrument]` attribute creates a span around a function. For
example, a connection handler might call this function once for each decoded
message:

```rust
#[tracing::instrument(
    name = "message",
    level = "debug",
    skip(payload),
    fields(payload_bytes = payload.len())
)]
fn process_message(message_id: u32, message_type: &str, payload: &[u8]) {
    tracing::debug!("processing message");
    // Validate and handle the message here.
    tracing::debug!("message processing complete");
}
```

By default, the attribute records all function arguments as fields.
`skip(payload)` excludes the message body, while `payload_bytes` records its
size. The other arguments identify the message. The [instrument attribute
documentation](https://docs.rs/tracing/latest/tracing/attr.instrument.html)
describes additional options.

### Choosing what to record

Here are some guidelines to consider for logging:

- Use consistent field names in different log messages.
- Record enough context to understand a failure, including the operation that
  failed and the error value. Log the error at the point where it is handled with
  that context, rather than repeatedly at every function through which it passes.
- **Important:** Do not record passwords, authentication tokens, or other
  private information (such as message content). Remember that automatically
  recorded function arguments may also contain such values; use `skip(...)` or
  `skip_all` where appropriate. Remember that the logs are visible on our course
  server to everyone.

## Metrics

While well-designed logs and traces give detailed information of the software
behavior, it is hard to get an overall understanding of the server load and
behavior based on them alone. **Metrics** are quantitative measurements that
summarize what is happening in the application. For example, they can tell us
how many clients are connected, how many messages the server processes over
given period of time, and how often processing fails.

Collecting these measurements over time helps us see changes in server behavior.
A growing number of active connections together with increasing response times
may indicate that the server is approaching its capacity. Metrics help to
identify when a problem occurs, while logs and traces help investigate the
individual events behind it.

### Different kinds of metrics

There are different kinds of metrics that can be measured at a server:

- **Counter** measures the total number of events since measurement started
  (usually when server was started). For example, typically a server might
  count the total number of accepted connections, number of received messages,
  or number of errors in processing.

- **Gauge** measures the current state of the system. This could be, for example
  number of currently active connections, or number of unprocessed messages
  waiting on a queue, or metrics related to memory usage.

- **Histogram** measures how values are spread across a specified range.
  Histogram could present the distribution of processing time of a request, or
  the size of incoming and outgoing messages. This tells more about the dynamics
  of the server behavior than just plain average: If there are large number of
  well-performing connections and minority of badly performing connections,
  histogram can show this behavior, while average may hide the badly behaving
  tail.

  To collect a histogram, one typically specifies the categories or buckets of
  different measurement ranges, and count the number of observation falling into
  each bucket. Below is an example of a histogram collecting HTTP request
  durations, from blog article "[Prometheus Metrics Explained: Counters, Gauges,
  Histograms &
  Summaries](https://victoriametrics.com/blog/prometheus-monitoring-metrics-counters-gauges-histogram-summaries/)".

  ![Message format](https://victoriametrics.com/blog/prometheus-monitoring-metrics-counters-gauges-histogram-summaries/histogram-request-duration.webp){: width="90%" .center-img }

To measure processing time in Rust, you can use `std::time::Instant::now()` to
take the current time at the beginning of the operation, and later call
`.elapsed()` when the operation is finished. This returns a `Duration` type
value; `.as_secs_f64()` converts it to seconds, including the fractional part:

```rust
let started = std::time::Instant::now();
// Process the request here.
let processing_seconds = started.elapsed().as_secs_f64();
```

### Measuring server behavior

_TODO: edit this_

A counter value alone does not tell us how busy the server is now. To calculate
**throughput**, we compare two readings over a known time interval. If the
received message counter increases from 1200 to 1500 in 10 seconds, the average
throughput during that interval is `(1500 - 1200) / 10 = 30` messages per second.
The calculation must account for counter resets, rather than interpret a server
restart as negative throughput.

An **error rate** can similarly describe error events per second. To calculate
the fraction of requests that fail, divide the number of failed requests by the
total number of requests in the same interval. A general error event counter
cannot necessarily be used for this fraction: one request may generate several
errors, and some errors may occur outside request handling.

For **latency**, define the operation being timed. Measuring from the start of
request handling to the completion of the response write gives a server-side
processing duration. Measuring from sending a request to receiving its complete
response at the client also includes network delays and other waiting. These
measurements answer different questions. In Rust, `std::time::Instant` can be
used to measure elapsed time without relying on wall-clock timestamps.

## Monitoring

To support the analysis of a busy server, there are **monitoring** and
visualization tools that use the metrics, logs, and traces described above to
follow the health and performance of a running server. Collecting observations
continuously helps to recognize changes in load and detect problems that might
not occur during testing. Below we shortly refer to **Prometheus** and
**Grafana** which are not used on the assignments, but you are free to try them
if you are interested and have spare time.

To make metrics available outside the application, the server needs to
**export** them. For example,
[Prometheus](https://prometheus.io/docs/introduction/overview/) is a widely used
tool that collects metrics from a specific HTTP endpoint, commonly `/metrics`,
where the application exposes them in format understood by Prometheus.
Prometheus stores the readings with timestamps, allowing us to examine their
history and calculate rates such as messages processed per second.

A **dashboard** presents these measurements as graphs and current values.
[Grafana](https://grafana.com/docs/grafana/latest/fundamentals/dashboards-overview/)
is another commonly used tool that can query a data source such as Prometheus and
display related measurements together. For our server, a useful dashboard could
show active connections, message throughput, error rate, and request processing
times. Comparing these graphs over the same time interval helps reveal how
server load affects performance.

**Alerts** notify the server operator when a condition requires attention,
without requiring someone to watch the dashboard continuously. For example, an
alert could indicate that the server is unreachable or that the error rate has
remained above a chosen threshold for several minutes. Requiring a condition to
persist can prevent brief spikes from causing unnecessary notifications. Alerts
should describe a problem that the operator can investigate or act on.

Metrics help locate the time and extent of a problem; logs and traces provide
the details needed to investigate it. If a dashboard shows an increase in
processing times, examine logs from the same period and use connection or
message IDs to follow affected operations. Together, these observations help to
notice, for example, increased traffic from failures or delays in particular
operations.

<div class="assignment-frame" markdown="1">

## Assignment

**Part 1**: If you haven't done so yet, add proper error handling to your
implementation in places where execution can be expected to fail sometimes, in
communication operations. User `error!` and `warn!` macros in appropriate places
to log these events. Add also an `info!` event when a new connection is
accepted.

**Part 2**: Implement at least one unit test for a selected functionality in your
implementation. If you haven't done so yet, rearrange some suitable
functionality (e.g. message header parsing) in your implementation as a separate
function that can be tested. Describe in your report shortly what functionality
you tested, and what kind of cases were asserted.

**Part 3**: To implement end-to-end tests, write a test client that connects
server, and tests TST ping, user registration using USR, and message sending
using MSG, with different valid and perhaps invalid sequences. Try also sending
an unknown message type. Test that your own server survives the test, and if
not, do the necessary fixes. Then test **two other servers** listed on the
course server (that claim to support "base-2" version of the protocol). In your
report describe your test cases, and what were your findings in the tests.

To make testing different servers easier, it is good to use command line
arguments to specify address and port to connect. You can place the test client
as a separate binary under `bin` directory, for example naming it as
"_testclient.rs_".

**Part 4**: Collect metrics at least for number of opened connections, number of
received messages and number of error events. Implement **MET** message that
returns the current state of the metrics collected at the server. The server
should respond with MET message having the same ID as the request. Following the
common header, the metrics should be included in JSON formatted string (remember
from last module that there is the **serde** crate that serializes structures
into JSON format). At least following metrics should be included:

- `"connections_total"`: Total number of connections opened to the server.
- `"messages_received"`: Total number of messages received
- `"errors"`: Total number of error events in the server (there should be more
  information about the errors in the trace log)

You can also have more metrics in addition to these if you want.

Finally, if you have updated your protocols plans and implementation, update
`doc/features.md` as needed. After you have done and tested the above parts,
commit and push your work to git normally, and update the server instance using
the `/run-docker` endpoint. Use "**base-3**" as the protocol identifier, or if
you have started implementing your project-specific protocol, use your own label
(this implies that you also implement "**base-3**" specifications).

Include also the following information:

- How much time did you use for this assignment?
- What was easy or difficult in the assignment?
- What tools or other information sources did you use? In particular, if you
  used AI assistants, tell how did you use them and if they were helpful.

</div>
