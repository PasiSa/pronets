---
title: Protocol design and projects
lang: en
translation_key: protocol
---

This part of the course starts a project that you will start working on until
the end of the course. In the following weeks you will enhance your project
according to the themes discussed each week (testing and monitoring, improved
concurrency, security, etc.). This module does not have particular technical
learning objectives, but the purpose is to set the stage for the project work.

## Common protocol specification

We will specify some basic properties and features for a communication protocol
used in the projects, but on top of these, you can specify your own protocol and
implementation design according to the topic of your choice. Each protocol has a
label that takes the following form: "`name-NN`" where "_name_" is the protocol
name, and "_NN_" is a positive integer number standing for the protocol version.
The protocol versions should be backwards compatible: "_name-2_" should be
compatible with all messages in "_name-1_", but has some additional features.
This label will be included in the _protocol_ field of the `/run-docker` request
introduced in the last module, that builds and runs the server container
implementing the protocol on the course server.

There will be a common base protocol that all projects should implement. The
base protocol has common frame format that the implementations should follow,
and some features divided into three versions as follows, following the
above-mentioned naming scheme:

- `base-1`: includes the **TST** message to test the connectivity between client
  and server, already implemented in Assignment 3.
- `base-2` (implemented in Assignment 4, described at the end of this page):
  adds the **USR** message that registers a named user using a client, and the
  **MSG** message that sends a message to other users connected to server.
- `base-3` (implemented in Assignment 5): adds the **MET** message that queries
  some usage and performance metrics from server.

All extended protocol implementations should implement "base-3" features and
format, in addition to the additional extensions you have designed.

### Message frame format

Because we are using a stream-oriented TCP protocol that does not preserve
message boundaries, we need to choose an approach that allows separating
messages on application layer. One common approach is to decide on a common
header to message, that tells the payload length (e.g. in bytes). Other
approach, as used e.g. by the old versions of HTTP and other classical
text-based protocols, is to specify a special character to indicate the end of
the message (e.g., two consecutive newline characters). In our common protocol
specification, we choose the former.

All protocol messages should start with the following header:

- Message starts with **message length** in bytes: a 32-bit unsigned integer in
  network byte order. The length indicates the full message length in bytes
  (octets), **including the length and ID fields**.
- Second field is the **message ID**: a 32-bit unsigned integer. The purpose of
  message ID is to match request messages with their responses. Easiest way to
  implement this is to assign increasing unsigned integer numbers, but other
  allocation method is possible, too, as long as the same ID is not used twice
  in the same connection (unless all 32-bit identifiers are already used, after
  which the ID number allocation can start from the beginning).
- Three characters followed by a space, that indicate the **message type**. The
  characters should consist of letters A-Z in upper case letters, or number
  characters, if needed.
- What follows after this is message dependent, as described below.

See below the picture shown in previous module, that shows the above described
structure:

![Message format](/images/protocol-format.svg){: width="90%" .center-img }

### Common messages, "base-1"

- **TST** (Test). Either client or server can send this to test that the
  connection works and the other end is responsive, or for example to measure
  connection latency.
  - **Parameters:**
    - **Test sequence**: Sequence of bytes that must be echoed by the other end.
      The format of these bytes can be anything, they can be an UTF-8 encoded
      string or any binary sequence of bytes. The test sequence may be long (up
      to almost 4 GB), the length is only limited by the total message length.
  - **Response:** The same message echoed back to the other end. Therefore, also
    the message length and message ID are same as in the request message.
  - **Example:** [length: 22]`TST 0123456789`

### Common messages, "base-2"

In addition to the TST message:

- **USR** (User registration). Client sends this message to associate username
  with the current TCP connection. This should be sent as the first message
  after connection is made (apart from TST message that is always accepted).
  - **Parameters:**
    - **Username**: UTF-8 encoded string following the USR message type. The
      length of the string can be determined from the message length field in
      the beginning.
  - **Response:** Server replies by echoing the same message back, or by sending
    an **ERR** message, if registration was not successful, for example the name
    was already taken. The response should have the same ID as the request
    message.
  - **Example:** [length: 18]`USR Jaakko` -- Registers user "Jaakko" with this TCP
    connection.

- **MSG** (Message, from client). Sends chat message to given channel. The
  message will be broadcast to all users who are member of the given channel.
  Note that the server needs to assign an unallocated message ID to each of the
  receivers the message is broadcast to, because ID numbering is
  connection-specific. The response to the sender uses the same ID than the
  client used with this request.
  - **Parameters:**
    - **Channel**: Name of channel to send chat message to. The message should
      be delivered to all users registered as members of the channel. Channel
      named "**general**" must always be supported, that contains all users
      connected to server. Supporting other channels is optional.
    - Space character
    - **Message**: Message as UTF-8 encoded text format (the default used by
      Rust strings). Message can contain any UTF-8 characters, for example line
      feeds. The length of the message is determined by the length field given
      at the start of the protocol message, taking the other protocol message
      fields into account.
  - **Response:** No actual response, but also sender will receive the
    broadcasted MSG from the server, that applies separate message ID
    allocation. If there is an error (for example invalid channel name), the
    server should respond with **ERR** message with same ID than the request,
    along with reason of the failure.
  - **Example:** [length: 45]`MSG general Hello, how are you doing?`

- **MSG** (Message, from server). Used by server to broadcast a message from an
  user to everyone on the channel. The payload of the server-originated message
  differs slightly from the client-originated message. Note that the Message ID
  is allocated by server, it is not the same than in the original MSG.
  Server should not reuse same message ID twice with a particular client.
  - **Parameters:**
    - **Sender**: Username of the sender of the message.
    - Space character
    - **Channel**: Name of the channel to where message was intended.
    - Space character
    - **Message**: Message as UTF-8 encoded text format.
  - **Response:** No response.
  - **Example:** [length: 52]`MSG Jaakko general Hello, how are you doing?`

- **ERR** (Error, from server). Send in response to a request message, if there
  is a failure in executing it. The message ID should be the same as in the
  request message that triggered the error.
  - **Parameters:**
    - **Reason**: Reason text in UTF-8 format.
  - **Response:** No response.
  - **Example:** [length: 35]`ERR Username already taken!`

### Common messages, "base-3"

_TODO: details of the protocol will appear a bit later_

## Protocol design principles

The development of the Internet protocols started in the 1970s. Even though the
protocols have evolved over the decades, and new applications and technologies
have been developed, the design principles chosen in the early protocols have
proven to be sustainable in enabling innovative uses of the old protocol
technology.

In 1996 the **[Internet Architecture Board](https://www.iab.org/about/)** in the
**[IETF](https://www.ietf.org/)** that defines the Internet protocol standards
produced a **[RFC 1958, "Architectural Principles of the
Internet"](https://datatracker.ietf.org/doc/html/rfc1958)** that proposes design
principles for developing new Internet protocols, as collected by a group of
experienced Internet engineers. Even though the document is more than 30 years
old, the principles are still good to be thought when developing protocols. Many
of them are also useful as general software design principles.

Here are the principles from the "General Desing Issues" section of the
document, with some additional commentary (by myself, can be argued):

1. **Heterogeneity is inevitable and must be supported by design.** The recent
   history has shown that the communication technologies and use cases have
   changed significantly, and there are very heterogenous environments where we
   need to consider: wireless devices may have unreliable connectivity to the
   Internet, and the communication speeds may vary significantly. Sometimes
   communication happens over satellite link, which causes long delays, and so
   on. This is good to be kept in mind when designing time-dependent logic, or
   robustness of an application.

2. **If there are several ways of doing the same thing, choose one.** Different
   network applications and protocols often need to solve similar challenges
   related robustness and reliability. Instead of developing a fancy new
   approach, it may be useful to study how earlier designs have solved problems,
   and whether same designs could be re-used.

3. **All designs must scale readily to very many nodes per site and to many
   millions of sites.** On this course we will not test the scale to millions of
   sites. Nevertheless, scalability is an important goal for protocol design,
   and for a network server implementation, for example when designing data
   structures or algorithms to process messages.

4. **Performance and cost must be considered as well as functionality.** When
   developing a new application, the functionality and features are naturally
   the gaining attention. One should also consider performance at the same time:
   communication protocol overhead and delays, processing efficiency, and memory
   management economy, for example.

5. **Keep it simple. When in doubt during design, choose the simplest
   solution.** Good principle in all cases: not only in protocol design or
   software engineering, but life in general. It is common mistake, especially
   for a beginner, to come up too complicated design, being blind to a
   simpler and easier approach.

6. **Modularity is good. If you can keep things separate, do so.** A commonly
   good principle in software engineering. In protocol design, the traditional
   layering of protocols is one way of applying this principle. It is good idea
   to apply common protocol design, and reuse functions e.g. related to message
   parsing (like for our common frame header discussed above).

7. **In many cases it is better to adopt an almost complete solution now, rather
   than to wait until a perfect solution can be found.** Our lifetime is
   limited, and so is the duration of this course.

8. **Avoid options and parameters whenever possible. Any options and parameters
   should be configured or negotiated dynamically rather than manually.** This
   is related to the first principle: because different network environments and
   technologies are vastly different, manually choosing values e.g. for timeouts
   may turn out to be a short-lived solution. Even though some values work now
   in the current environment, it may not work after few years. Individual
   implementations can be updated through software updates, but protocols are
   implemented across multiple implementations in different systems, and they
   are more difficult to be universally changed later after deployment.

9. **Be strict when sending and tolerant when receiving.** This is a common
   principle of robustness in protocol implementations. When sending messages
   out to the network, one should be careful to follow protocol specification
   and its rules, to not cause problems at the receiver. When receiving
   messages, one should try to handle robustly all sorts of failures in
   communication (invalid or illogical values, unidentified protocol formats,
   etc.) Some mistakes are unintentional, perhaps misunderstanding a protocol
   specification by another implementation, and some may be intentional, for
   example by some attacker trying to find vulnerabilities.

10. **Be parsimonious with unsolicited packets, especially multicasts and
    broadcasts.** It is always good to be economical in protocol traffic.
    Network communication consumes energy on different scales: network routers
    and datacenters are large energy consumer, but also in smaller scales, for
    example if a wireless battery-powered device needs to be woken up because of
    a message that could have been avoided.

11. **Circular dependencies must be avoided.** A good rule in any software
    design.

12. **Objects should be self describing (include type and size), within
    reasonable limits.** There maybe different evolutions and versions of a
    protocol, and some implementations may not support all features. Therefore
    the protocol should be designed in such way that unknown messages can be
    easily ignored, while finding where to continue in the TCP stream, for
    example.

13. **All specifications should use the same terminology and notation, and the
    same bit- and byte-order convention.** This is more a guideline for IETF
    standardization work, but a good principle also in our local
    implementations. We will use big-endian network byte order, like the IETF
    protocols.

14. **Nothing gets standardized until there are multiple instances of running
    code.** This is a guideline for IETF standardization, but we will also aim
    for few converged protocol specifications with more than one independent
    client and server implementation of each.

## From base protocol to networked application

At this point you can choose a project topic to work on during the rest of the
course. Here are some common requirements for all projects:

- The projects must implement the base protocol as specified in the course
  material. The new project-specific protocol messages must use the same header
  format (length, ID, type name) as the base protocol messages on the TCP
  channel.
- The server implementations must be able to serve multiple clients concurrently
  and timely.
- By the end of the course, the communication should use TLS (apart from UDP
  communication implemented near the end of the course).
- The project should involve real-time communication over UDP. This could be
  some collaborative editing feature, a real-time component in a multiplayer
  game, or for a more challenging option, voice chat between connected clients
  (there are available Rust crates for sampling audio from microphone into data
  chunks using a chosen codec, and playing them back)

The goal is that for each project topic there are at least two independent
client and server implementations by two different groups (or individuals, if
you are working alone). Different implementations may have some different
features, but the goal is that within each topic there should also be some
common protocol messages that are mutually understood between implementations.

## Project topics

Here are project topics for you to choose from. All projects will have a chat
forum, that was implemented as part of the base protocol in the assignments. The
following descriptions propose ideas about possible features and related
communication needs, but you get to specify the actual protocol messages
collaboratively with others working on the same topic.

### Collaborative whiteboard

Different users can connect to 2D whiteboard where they can draw different kinds
of elements. There should be tools to draw at least text, rectangle and a line,
but you can develop also additional features for different shapes, choosing
colors, moving objects, and so on. Optionally, you can manage multiple
named whiteboards (like there can be multiple chat rooms).

Likely protocol actions needed:

- Add element to given coordinates. The message should include also attributes
  depending on the element type (e.g., the text to be added, dimensions of the
  rectangle, etc.). When one client adds an element, other users connected to
  the whiteboard should see it, too.

- Get all current elements on the whiteboard. When a new client joins an
  existing whiteboard, it needs to request its current status. The response to
  this message could be a series of "Add" messages described above, for the
  previously added items.

- Erase white board. Because the server sessions are long lived, the whiteboard
  may get messy over time, and therefore resetting it to empty state may be
  useful.

Idea for a real-time element (using UDP):

- There could be an interactive pointer (e.g. different colors for different
  clients).

### Collaborative document editor

Clients work on a shared text document that can be edited at the same time. Pay
attention to how to manage possible edits to a same location in the document,
and how to keep it consistent between clients. Optionally, there could be
different ways of highlighting the document (colors, effects, or shapes). As one
possible additional feature, it could also be possible to download the document,
or upload some earlier text as basis.

Likely protocol actions needed:

- Add text at given line and row. Doing additions one character at a time may be
  inefficient communication-wise, so you may want to consider doing additions in
  larger chunks.

- Get current status of document. Clients that join late need this. This may
  also be useful to as verification mechanism to ensure that everyone have
  consistent view of the document.

- Erase content from the document

Idea for a real-time element (using UDP):

- Interactive cursor to follow where user is currently working on

### Multiplayer car racing

Simple 2D game where multiple players race on a track, or just in a free
environment with some obstacles. Nice graphics is not a design goal in this
project, but it is easy to find free-to-use sprite art (e.g. for cars) from the
network you can use. For each car the server and clients should track at least
the current position, orientation, and velocity. You may also have additional
features for the cars. In minimal implementation the cars may just race freely,
but you could also add a competitive element (e.g., race 3 laps, take time).
There should also be some form of collision detection (even if not accurate).
Optionally, you could also have automotive NPC cars, that are operated by a
separate client, i.e., there could be clients that interact with user, and other
kind of clients that just operate the cars, that don't necessarily even have a
user interface, but run in the background.

Because network communication has delay, the server should maintain the
authoritative status of each car, given their position and velocity and progress
of time. To make the experience smoother, also clients can also update the
locations locally knowing these attributes, but sync the actual status from
server frequently enough.

Likely protocol actions needed:

- Initialize race, including the track information and possible obstacles or
  other objects.

- Add car at position. Additionally there should be identifier for the player,
  and attributes that help separating the car from others, such as color.

- Update car orientation, position and velocity

- Car left. When server detects a closed connection, it should inform other
  clients about this. The left car could be removed from the race, or left as
  abandoned in the field. The latter option would then allow a player to
  re-enter game (for example, after a short connectivity break).

- End game. At some point the game state needs to be cleaned for a new game, and
  clients informed about it.

Idea for a real-time element (using UDP):

- Real-time car position updates can be done using UDP

### Drone simulator

There would be series of drones on a shared 2D map with obstacles. You can
specify the purpose of the drones freely. They could be delivery drones, or
perform some sort of surveillance tasks. The drones can be human controlled or
autonomous. The client controlling autonomous drones does not necessarily need
user interface. Instead, there could be a separate kind of client that just
offers the user interface to monitor the drone activities. One possible
additional feature could be that of the clients could be a coordinator that
communicates tasks to available drones (using some designed protocol).

This implementation may have some similarity with the car game, and the protocol
messages could be designed in compatible way (e.g., regarding position updates,
and drones entering and leaving the simulated environment).

Likely protocol actions needed:

- Initialize simulation field.

- Add drone at position.

- Update drone velocity (the drones could maintain orientation or not, depending
  on the scenario).

- Drone controller left. You can decide whether drone disappears, or stays in
  the map for a possible reconnection.

- End simulation.

Idea for a real-time element (using UDP):

- Position updates can be done using UDP

## Project goals

At the end of the course, a successful project has the following properties:

- There is a document that describes the project scope, implemented features,
  protocol messages used, and outlines how it was tested. Particularly, there
  should be clear instructions how to build and use the software.

- Documentation about the interoperability tests with other clients and servers.

- The code in git repository should be well-structured and logical to follow.

- The implemented project features work without major bugs.

- There are comprehensive tests, particularly focusing on the communication logic.

- The project works with multiple (at least four) parallel clients robustly and
  responds to actions timely. Tests for load from parallel users have been
  conducted and documented.

- The project applies TLS-secured communication for the TCP streams used.

- The project has a real-time component that uses UDP.

Additional features involving communication are considered a plus, but not a
requirement, for example those proposed with project descriptions.

## Useful library crates

The visual appearance of client is not an evaluation criteria, but some sort of
graphical user interface is needed for the client to do sensible actions. Below
are some crates you may use. All of them should work in different systems
(Windows, Mac, Linux).

### Graphical user interface

- **[Slint](https://crates.io/crates/slint)** ([web page](https://slint.dev/))
  is a GUI toolkit for building user interfaces. It uses a declarative UI
  language to describe layouts, components, animations, and application state
  while keeping application logic separate from the interface. Slint provides
  features such as widgets, images, transformations, animations, and event
  handling. Slint is a good fit especially for the whiteboard client, but it
  supports also animated textured objects, so you can also use it for a simple
  game or drone simulation.

- **[egui](https://crates.io/crates/egui)** is a GUI library,
  where the application describes the interface on each frame rather than
  maintaining a persistent widget hierarchy. It is designed to be simple to
  integrate into Rust applications and works well for tools, editors,
  visualizations, debug interfaces, and interactive applications. egui supports
  common widgets, custom drawing, input handling, layouts, and graphics, and can
  run as a native desktop GUI or in a web browser through WebAssembly.

- **[Macroquad](https://crates.io/crates/macroquad)** ([web
  page](https://macroquad.rs/)) is a game development and graphics library,
  designed for creating 2D games, simulations, and other interactive graphical
  applications. It provides a simple API for opening windows, drawing shapes and
  textures, handling keyboard and mouse input, playing audio, and implementing a
  frame-based game loop, while also supporting features such as sprite rotation,
  collision detection through application logic, and text rendering. It is a
  good option especially for the car game or drone project, but maybe less ideal
  for whiteboard. Macroquad applications can run natively on desktop as well as
  in web browsers through WebAssembly.

  Online book "[Game development in Rust with Macroquad](https://mq.agical.se/)"
  is useful tutorial into various Macroquad features. Our examples directory has
  also a simple [Macroquad
  example](https://github.com/PasiSa/pronets/tree/main/examples/macroquad/src/main.rs)
  that demonstrates rotating and moving an object based on keyboard commands.

- **[Bevy](https://crates.io/crates/bevy)** ([web page](https://bevy.org/)) is a
  game engine written for building 2D and 3D games and interactive applications.
  Its architecture is based on an Entity Component System (ECS), where game
  objects are composed from data components and game logic is implemented as
  systems operating on those components. Bevy provides integrated support for
  rendering, input, audio, animation, physics through plugins. While rich in
  functionality, Bevy may be more difficult to approach for a beginner than
  Macroquad.

### Other tools

- **[Clap](https://crates.io/crates/clap)** (for "Command Line Argument Parser")
  is useful and handy crate for parsing command line arguments into a structure,
  from which various kinds of attributes are easy to access and use.

- **[Serde](https://crates.io/crates/serde)** is a framework for serializing and
  deserializing Rust structures into different formats, such as JSON, that could
  be used to store application state to disk, or for transfer stuructured data
  inside protocol messages.

<div class="assignment-frame" markdown="1">

## Assignment #4

In this assignment we start the work on project and implement the "**base-2**"
part of the common base protocol. We will also check if the **TST** message
works in other implementations available at the course server.

**Part 1:** Create folder "_doc_" in the root of your git repository and add
document "**design.md**" that provides a short description of your project and
the protocol you implement in **markdown format** ([here is one
guide](https://www.markdownguide.org/basic-syntax/)). It is not a good idea to
add unnecessary binary files such as pdf documents to git repository, because
they grow its size rapidly, therefore we use markdown for this.

This will be the first version of the document, and it can be updated during the
coming weeks as your plans get clarified or adjusted. At this point the document
should contain (at least) the following information:

- Name of the project
- Short description (1-2 paragraphs) of the main idea, scope and the main
  features
- High level description of the client(s) (about a paragraph): will it be
  graphical, what libraries/crates will be used? Will there be just a client for
  user interaction, or possible other kinds of clients for different roles (e.g.
  autonomous control of some objects)
- There will be a design description of the server, but we will do that a bit
  later, after we have discussed different server design options more
  specifically.
- Initial idea of protocol messages and their content that you will need in the
  project. At this point it is ok for this to be sketchy and unspecific. The
  protocol details are clarified along the way, and we aim for similar topics to
  synchronize their protocols to be compatible for future interoperability tests.

**Part 2:** Using the client you implemented in last assignment, send **TST**
messages to at least two of the servers listed in the course server's
**[container view](https://pronets1.dice.aalto.fi/)**. If you find something to
fix in your implementation, implement the fixes (the problem could be at the
other end, too, though). Report in the report the server instances you tested,
and your findings.

**Part 3:** Implement the **USR** message that registers a named user to the
server connection. This should be sent as the first message when client connects
to server. Server should respond by either echoing the message back, or with an
**ERR** (error) message, if it cannot register the user (e.g., if there already
was user by same name). Apart from the **TST** message, the server should not
accept any other messages before the **USR** message is successfully received,
and user is registered. Do both client and server implementations.

**Part 4:** Implement the **MSG** message that sends the given message all users
at given channel. You can just assume that there is only single channel
called "**general**", that contains all users connected to server. Support for
multiple channels can be implemented later. Do both client and server
implementations.

Note that for now, just a text client on a command line is sufficient to test
these operations, although it doesn't harm to start working with a graphical
user interface already, if you are motivated.

After you have implemented the above parts, test your implementation locally
using **at least two simultaneous clients** and server. When implementation
seems to work, check that you have committed and pushed your changes to git, and
send the `/run-docker` request to **pronets1.dice.aalto.fi** to update the
container to the current version. Follow the instructions from previous
assignment, but now use "**base-2**" as the protocol label.

As before, shortly answer also the following questions:

- How much time did you use for this assignment?
- What was easy or difficult in the assignment?
- What tools or other information sources did you use? In particular, if you
  used AI assistants, tell how did you use them and if they were helpful.

</div>

## Rust hints

Below are a couple of Rust code samples that may be useful for the assignment.
They are not complete and fully functionally, but possibly useful when working
on your implementation.

### Broadcasting message to all clients

The
**[iterative-server](https://github.com/PasiSa/pronets/tree/main/examples/iterative-server/src/main.rs)**
example demonstrating I/O Multiplexing has a structure for each client that
stores client-specific information such as the socket used for communication,
and currently connected clients are stored in HashMap collection.

You can iterate through all values in the HashMap for example by using the
`values_mut()` function. It returns a mutable iterator, i.e. mutable reference
to one client at a time. Because the value is mutable, the contents of the
structure can be modified, and because it is a reference, the ownership stays in
the HashMap.

Here is an example of code that could be inside the poll loop. A nice
organization of the code could be a separate function that broadcasts a message
to all currently connected clients.

```rust
for client in clients.values_mut() {
    if let Err(e) = client.socket.write_all(&buf) {
        println!("Error writing to client: {}", e);

        // We could have a boolean "active" flag in our hashmap to indicate which
        // clients are still operational, and then at some point, e.g. at the
        // end of the poll processing loop clean up inactive clients
        client.active = false;
    }
}
```

### Using match to parse different message types

Now that we start to have multiple different message types, the match statement
is a nice way of handling different message type. Below is an example of how it
could be done. For each message type, there could be a separate function to
process the particular kind of message.

The processing functions return a Result type, that is checked for errors, in a
common place for all processing functions. Even though using the `?` operator is
a bad idea to be used, e.g., in the `main()` function, here inside the
processing functions it could be helpful, if we know that the error will be
processed soon after the function returns.

The example also works to remind that the length field should be validated so
that it covers the mandatory headers, and has some upper bound so that you don't
accidentally try to allocate a very large chunk of memory (and likely fail).

```rust
// message length and ID have been read earlier
let mut msg_type_bytes = [0_u8; 4];
// TODO: Read message here to above array

// Convert the bytes to string
let msg_str = String::from_utf8_lossy(&msg_type_bytes);

// We assume the socket and length variables are set somewhere earlier.
let result = match msg_str.as_ref() {
    "TST " => process_tst(&mut socket, length),
    "MSG " => process_msg(&mut socket, length),
    // other message types....
    _ => {
        // handle unknown message
    }
};
if let Err(e) = result {
    // Error handling
}

fn process_tst(socket: &mut TcpStream, length: u32) -> std::io::Result<()> {
    if length < 12 || length - 12 > 4096 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid or unsupported message length",
        ));
    }
    let mut buf = [0_u8; 4096];
    let n = (length - 12) as usize; // subtract the common header
    socket.read_exact(&mut buf[..n])?;
    // ...function continues...
    Ok(())
}
```
