---
title: Protocol design and project grouping
---

## Protocol design

_TBD_

In this course you are provided a common protocol structure and basic protocol
messages that all implementations must support. On top of that, you can design
extensions to support additional features. These may be:

- Collaborative editing of a text document or a simple drawing tool
- Simple game of some sort, for example a open world simple strategy game (or
  Nethack-style dungeon game), online chess, or something else.

If you design custom extensions, you should find a group of at least three
persons that agrees to use the common protocol. This is to enable
interoperability testing between implementations later in the course. Each
member will do own implementations of both client and server, but can
collaborate in doing so, to verify that protocols are interoperable.

The protocol extensions must be specified in a common document in such detail
that each group participant is able to implement a interoperable implementation.

There are common extensions for collaborative text document editing, where there
is a reference implementation from the course personnel. If you don't have a
group to design protocol extensions, you can follow this implementation, as
described below.

Each protocol design should have a feature that requires transmission of large
files (more than 100 KB), and a feature that requires real-time communication
using UDP.

_TODO: How to separate messages in streamed communication: framing,
Content-Header, delimiters._

## Common protocol specifications

All protocol messages follow the following pattern:

- Message starts with message length: a 32-bit unsigned integer in network byte
  order. The length indicates the number of bytes (octets) that follow the
  integer
- Three characters followed by a space, that indicate the message type. The
  characters MUST be from the 7-bit ASCII character range, i.e., they can be
  parsed in compatible way following the UTF-8 encoding. The characters SHOULD
  consist of letters A-Z in upper case letters.
- What follows after this is message dependent, as described below.

### Protocol messages

- **ADD** (Add to channel). Client sends this message to add the current user to
  a discussion channel, to receive messages and other content posted to the
  channel.
  - **Parameters:**
    - **Channel**: Name of the channel to join. The channel name MUST consist
      of alphanumeric characters, i.e., either upper or lower case letters or
      numbers in UTF-8 format. Specifically, the space character MUST NOT be
      included in the channel name.
  - **Response:** _TBD_
  - **Example:** _TBD_

- **ERR** (Error). _TBD_

- **MSG** (Message). Sends chat message to given channel.
  - **Parameters:**
    - **Channel**: Name of channel to send chat message to. The message should
      be delivered to all users registered as members of the channel. See the
      restrictions applied to channel name from the **ADD** protocol message
      descriptions.
    - Space character
    - **Message**: Message as UTF-8 encoded text format. Message can contain any
      UTF-8 characters, for example line feeds. The length of the message is
      determined by the length field given at the start of the protocol message,
      taking the other protocol message fields into account.
  - **Response:** _TBD_
  - **Example:** _TBD_

- **OBJ** (Data Object). Posts a named data object to the given channel.
  - **Parameters:**
    - **Channel**: Name of channel to send message to. See the restrictions
      applied to channel name from the **ADD** message descriptions.
    - Space character
    - **Name**: Name of the object transmitted. The channel name MUST consist
      of alphanumeric characters, i.e., either upper or lower case letters or
      numbers in UTF-8 format. Specifically, the space character MUST NOT be
      included in the name.
    - Space character
    - **Data object**: Sequence of bytes representing the object. The format of
      bytes can be anything.
  - **Response:** _TBD_
  - **Example:** _TBD_

- **TST** (Test). Either client or server can send this to test that the
  connection works and the other end is responsive, or for example to measure
  connection latency.
  - **Parameters:**
    - **Test sequence**: Sequence of bytes that must be echoed by the other end.
      The format of these bytes can be anything, they can be an UTF-8 encoded
      string or any binary sequence of bytes. The test sequence may be long (up
      to 4 GB), the length is only limited by the total message length.
  - **Response:** The same message echoed back to the other end.
  - **Example:** [length: 14]`TST 0123456789`

- **USR** (User registration). Client sends this message to associate user with
  the current TCP connection. Either this, or more secure **AUT** message MUST
  be sent as the first message after connection is established, before any other
  messages are sent in the connection.
  - **Parameters:**
    - **Username**: UTF-8 encoded string of length as specified by the message
      field.
  - **Response:** Server replies by echoing the same message back, or with an
    **ERR** message, if registration was not successful, for example the name
    was already taken.
  - **Example:** [length: 10]`USR Jaakko` -- Registers user "Jaakko" with this TCP
    connection.

## Collaborative document protocol

## Assignment

_TBD: write decent description_

Make a basic client implementation that connects to server and can send the
**TST** request implemented last time, along with the following new messages.

Implement `USR user` message that should be sent as a first message when client
connects to server. Just plaintext username is sufficient at this time (later we
will implement more secure authentication for this). Server should respond by echoing the
message back.

Implement `ADD channel` message that adds an user to the given message channel

Implement `MSG channel message` message that sends the given message all users
at given channel. For now you can just assume that there is only single channel,
and the message is broadcast to all users connected to server. Support for
multiple channels can be implemented later.
