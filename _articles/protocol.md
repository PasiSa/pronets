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

## Common protocol specifications

## Collaborative document protocol

## Assignment

_TBD: write decent description_

Make a basic client implementation that connects to server and can send the
**TST** request implemented last time, along with the following new messages.

Implement `AUT user` message that should be sent as a first message when client
connects to server. Just plaintext username is sufficient at this time (later we
will implement more secure token for this). Server should respond by echoing the
message back.

Implement `ADD channel` message that adds an user to the given message channel

Implement `MSG channel message` message that sends the given message all users
at given channel. For now you can just assume that there is only single channel,
and the message is broadcast to all users connected to server. Support for
multiple channels can be implemented later.
