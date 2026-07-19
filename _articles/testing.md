---
title: Testing and observability
---

## Testing

_TODO: Different types of tests in Rust and how they are used in Cargo_

How unit and integration tests are a build-in feature in Rust, how cargo is used
to run the tests.

See [Rust book section
11.1](https://doc.rust-lang.org/stable/book/ch11-01-writing-tests.html) about
how to write tests.

### Unit tests

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

### Integration tests

_TODO: Tell what integration test is_

We need to add `lib.rs` so that the program logic can be executed as a
"library". Tests do not use our normal main function. This basically just
includes our other program modules

Interation tests should be in a separate top-level "tests" directory.

Example in pronets-server.

## Continuous Integration

_TODO: GitHub example in pronets-server_

Unfortunately version.aalto.fi does not support CI runners.

### End-to-end tests

_TODO: do we need this for now?_

### Testing network protocols

_TODO: think abot if this is needed_

## Debug output trait

_TODO_: How to do it

## Logging and tracing

_TODO_

In early versions of our programs we have used the most basic (and most common)
analysis method, `println!` to printout the program process on console. In
server applications there normally is no console available, but if we want
output, we want to log it to a file (or database).

There are different needs for logging. During development we want more detailed
information about the program progress, but when the system is in production and
used my many clients, this usually is too much information. In production
environment we still want to get information about unexpected events or error
situations, and maybe some relevant normal events (e.g., new client arriving).

Rust offers logging support for different levels of logging using specific
macros, instead of `println!`. The logs generated this way can be structured in
useful way, and directed to chosen output destination. Rust applies the
following logging levels (from highest to lowest):

- **ERROR**, `error!(message)`: Something failed (e.g. server could not bind to
  chosen port)
- **WARN**, `warn!(message)`: Something unexpected happened, but the application
  can continue (e.g., user message had something incomprehensible that was
  ignored)
- **INFO**, `info!(message)`: Important normal events (e.g. new user logged in)
- **DEBUG**, `debug!(message)`: Information useful for debugging (e.g. message XXX was
  received and succesfully processed)
- **TRACE**, `trace!(message)`: Detailed execution information (e.g. message field values
  of incoming message, or state variables at different phases of processing)

Usually the log output can be filtered based on levels. Many loggers use system
environment variables that can be specified when starting a program (often
"`RUST_LOG`"), that can be specified before starting the server on command line,
or in Dockerfile specification. For example, to show outputs only for level INFO
and above, one might execute the program in the following way:

    RUST_LOG=info cargo run

_TODO: example of logging in a function_

### Structured tracing

Especially for asynchronous network application it is useful to apply
**structured tracing** for analysis. Instead of plain flat-level logging with
output lines of varying severity, tracing adds some structure to analysis that
is useful for analysis when there are several sessions and events, as in a
production server.

Rust provides **tracing** crate that makes tracing easy, being able to offer
various output formats from plain text lines to structured JSON output and
delivering the logging data to analysis tools through a dedicated API.

_TODO: structured tracing and example_

## Metrics

_TODO_

- Counters
- Gauges
- Histograms
- Measuring request latency, throughput, error rate, active connections

## Monitoring

_TODO_

- Exporting metrics (e.g., Prometheus endpoint)
- Dashboards (Grafana)
- Basic alerting concepts
- Using logs and metrics together to diagnose problems

## Assignment

<div class="assignment-frame" markdown="1">

- Add tests for the functionality developed so far, at least three tests

- Add trace logging to your project

- Collect metrics at least for number of opened connections, number of received
  messages and number of error events. Implement **MET** request that returns
  the current state of the metrics. The server should respond with MET message
  having same ID as the request. Following the common fields, the metrics should
  be included in JSON format. At least following metrics should be included:
  - `"connections_total"`: Total number of connections opened to the server.
  - `"messages_received"`: Total number of messages received
  - `"errors"`: Total number of error events in the server (there should be more
    information about the errors in the trace log)

  You can also have more metrics in addition to these if you want.

</div>
