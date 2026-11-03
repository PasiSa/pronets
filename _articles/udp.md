---
title: UDP and real-time transport
lang: en
translation_key: udp
---

So far we have been using TCP, a **connection-oriented** protocol providing
**reliable stream communication** service between two connection end points. A
TCP connection starts with a **three-way handshake** that initializes the needed
state at both ends of the connection. To ensure reliability, data is buffered at
the both ends of the connection, and data segments are retransmitted if the
receiver does not acknowledge them in due time. TCP sender also controls the
rate at which it is sending data, to avoid congesting the network. While
providing reliable service, all this adds delay to the data transmissions. We
will now take a look at a very different protocol: User Datagram protocol that
is not reliable, but offers more predictable delays, useful for applications
with time constraints on data delivery.

<div class="objectives-frame" markdown="1">

**Objectives for this module:**

- You will **learn the basics of using datagram** sockets, and their differences
  to stream socket semantics. Commonly, datagram sockets are used to communicate
  with UDP protocol.

- Through a simple example, you will get an **basic understanding how to set up
  real-time communication using UDP**, using clap library.

- You will **understand how stateful Network Address Translators and firewalls**
  affect the communication between two end-hosts, and how to solve the
  difficulties the cause in reaching the other host.

</div>

## User Datagram Protocol and datagram sockets

The **User Datagram Protocol, UDP ([RFC
768](https://datatracker.ietf.org/doc/html/rfc768))**, is different to TCP in
many ways. UDP is not reliable and does not carry per-connection state between
the endpoints. Therefore it is not connection-oriented, either: A UDP sender can
start sending data as soon as the socket is set up and bound to a local address.
UDP's communication model is **datagram service**: the message boundaries of
datagrams sent to a socket are preserved: if sender sends a data unit of 1000
bytes, we know that a read operation at receiver will return 1000 bytes (unless
the datagram is lost in the network). With TCP, we needed to build a framing
protocol (length, type headers in the beginning of message) in our application
to find message boundaries. With UDP this is not needed: when we read the next
datagram, we know that it always starts from the beginning, as sender has
intended.

Unlike TCP, unreliable UDP does not have transport-level acknowledgments to
indicate received data. If the sending application needs a confirmation that a
message was actually received, the receiving application needs to send an
acknowledgment back. Because UDP does not have retransmissions or congestion
control, there is less need of buffering (i.e., less delay), although some
buffers are used also with UDP sockets. Therefore UDP is a viable choice for
applications where predictable delays are important and preferably as short as
possible, such as conversational real-time audio and video, or online gaming.

Because UDP is not connection-oriented, no `connect()` operation is needed to
start the communication, and the same UDP socket can be used to send data to
multiple different destinations. Therefore, a **datagram socket** typically uses
the `send_to()` function that, in addition to the data buffer to be sent, takes
the destination address as an additional parameter to indicate where the
datagram is to be sent. If the underlying network support broadcast or multicast
delivery, datagram socket and UDP can also be used for multicast transmission: a
datagram sent to a specific **multicast address** can be delivered to multiple
hosts registered to receive traffic sent to that address.

### Simple example

The course repository has example
**[simple-udp](https://github.com/PasiSa/pronets/tree/main/examples/simple-udp/src/main.rs)**
that demonstrates how datagram sockets and UDP are used. The `main()` function
is at the end of the file, and is very short. It just parses a couple of command
line arguments using the `clap` crate, and depending on the arguments given,
assumes the role of server or the client, both implemented in separate
functions in the same program.

**Binding the socket.** The client function begins by creating the UDP socket
with the `bind()` call:

```rust
let socket = UdpSocket::bind("0.0.0.0:0")?;
```

Because UDP is not connection-oriented, there is no need for `connect()` call.
With these arguments, the call asks system to pick any available UDP port for
the local end, along with any IP address available in the local system.
In fact, when `connect()` is called for TCP socket, it implicitly does the same
operations. With this function we could also give a specific IP address
and port if we wanted (also for a TCP socket before calling `connect()`).

Note that even though UDP ports are also allocated from a similar unsigned
16-bit number range as TCP ports, the two port namespaces are separate.
Therefore, if your project already binds to TCP port 1234 for listening the
client connections, you can also bind to UDP port 1234 at the same time to
handle, for example, real-time communication for your project.

**Sending data.** The program reads a line from user into a `String` and sends
it to a destination using `send_to()` function:

```rust
 if let Err(err) = socket.send_to(input.trim().as_bytes(), &args.connect_addr) {
    eprintln!("Failed to send datagram to {}: {}", args.connect_addr, err);
    continue;
}
```

As in the case of TCP sockets, the string is first converted to bytes for
transmission, and given as the first argument. There is also a second argument,
for passing the destination address, because it has not yet been specified. The
destination address can be given in similar format as with `connect()`, for
example as a string, as we do here, that will be then parsed into actual IP
address and port. Even though we do not do it in this example, in each
subsequent `send_to()` call we could give a different address, even when using
the same socket.

If there is an error we report it, and jump to the beginning of the loop for the
next user input.

It is good to note that also `send_to()` call can block even when used with UDP,
unless for non-blocking I/O is used, for the same basic reason as with TCP: the
operating system places the data into socket send buffers until the network
device driver can actually send the data, and it is possible that these buffers
get full if the sending application is sending fast. Because UDP is not reliable
or does not apply congestion control, with UDP, such block events are likely
shorter and less common than with TCP, but it is possible that a fast sending
application may outpace operating system kernel under some circumstances. For
example, there may be situation that the network device is temporarily not ready
for sending, which may cause the data to be queued in the socket buffers on
upper protocol layers.

**Receiving data.** After the data is sent, the client waits for server response
in a `match` statement:

```rust
let (size, src) = match socket.recv_from(&mut buf) {
    Ok(result) => result,
    Err(err) => {
        eprintln!("Failed to receive datagram: {}", err);
        continue;
    }
};
```

Because we are using unconnected socket, we need a different function also for
receiving than what we used with TCP. `recv_from()` works otherwise similarly as
e.g. `read` function (or rather `read_all`, since datagrams are delivered as
whole), but it returns a tuple of two values: in addition to the number of bytes
received, it return the address from where the datagram arrived (`src` in this
case). As with sending, `recv_from()` can receive datagrams from different
senders in the same socket.

The `recv_from()` call waits until datagram comes in. In this example it is ok,
but in a more complicated application with different input sources, the usual
methods for handling concurrency are still needed.

**Server operation** is quite similar to client operation with UDP. There is no
separation into passive or active sockets, and no need to call `accept()` to
accept new incoming connections. The main difference is, that in the server
case, we bind to a specific UDP port so that other hosts know to which address
and port to send data to, and that we do the receive and send operations in
different order. The server does not take user input, but just echoes incoming
datagrams back to the address received with `recv_from()` return value.

It is possible to call `connect()` with a given IP address and port also for UDP
socket. This fixes the socket with one particular remote end. After this the
normal `read` and `write` operations can be used, and datagrams from other
addresses are not received. In terms of transport protocol operation the
`connect()` call does not have any effect: there is still no three-way handshake
for UDP protocol. However, there is one benefit: If a network device detects an
error, for example because the destination host was not reachable, it can send
an **ICMP error message** back to the sender, even though there is no actual
protocol-level stateful connection. If the UDP socket was connected, the
operating system can deliver this error to the socket as an error code, because
the ICMP message contains the protocol headers of the original packet, including
the port number of the source.

## Real-time audio streaming

A common traditional use case for UDP communication is real-time audio
conversation between two or more participants. Each end point samples the audio
from a microphone, converts them to data packets and sends them to the intended
receiver (or multiple receivers). If there is too much delay, interactive
conversation becomes difficult. Rather than delivering an audio sample too late,
it may be more useful to drop it, to make room for more recent samples.

There are many possible sources of delay: transmitting packets over the network
causes delay, but usually the application programmer cannot do much about this.
The application itself may contribute to delay for different reasons. Typically
the audio samples are first collected to a buffer at a constant rate, from which
they are copied to UDP datagrams and sent over network. At the receiving end the
reverse happens: data is stored to buffers waiting for the playback device to
consume them. Queueing data in buffers contributes to the delay. Therefore the
buffers should not be too large: larger the buffer, the more delay there may be.

### Example based on CPAL crate

The
**[audio-streaming](https://github.com/PasiSa/pronets/tree/main/examples/audio-streaming/src/main.rs)**
example shows a simple (yet more than 400 lines) example of an application that
uses the **[CPAL (Cross-Platform Audio Library)
crate](https://crates.io/crates/cpal)** to sample data from an input source and
output it to an output device. It applies the **Opus codec ([RFC
6716](https://datatracker.ietf.org/doc/html/rfc6716))** provided in [opus
crate](https://crates.io/crates/opus) to encode and decode the raw audio data
for efficient transmission inside UDP datagrams.

CPAL uses **input and output callback functions** to interact with the audio
devices. The _input callback function_ is set on line 194 as a closure in a
similar way than when operating with e.g. threads. `input_callback()` returns
the closure function that CPAL calls repeatedly when enough data is available.
The callback function collects the samples received from CPAL into `pcm_buffer`,
and when 20 milliseconds worth of data is received, if calls the _opus encoder_
to encode the data into `opus_packet` buffer, that is sent to the UDP socket
opened earlier in the main function. Note that in this case we use a **connected
UDP socket** created earlier in the main function, and therefore can use the
`send` function without destination address.

The below picture illustrates the input and output path of the example, and its
interaction with CPAL and various buffers.

![Audio example](/images/udp-cpal.svg){: width="100%" .center-img }

For audio output based on incoming UDP packets, a **ring buffer** `ring` is
used. Receiving incoming UDP datagrams is done in a separate thread, and the
ring buffer is split into `decoder_producer` handle that is moved inside the
thread, and `decoder_consumer` handle that is used by the **output callback
function** from the main thread. Ring buffer is a common structure in data
communication: it maintains the location where the next data item should be
written, and the location where the consumer should read next. The ring
structure allows to efficiently access the data to process a continuous ongoing
stream without having to reallocate the buffer later.

The output callback function is simple: it just checks if there is data
available in the buffer, and if so, it is copied to audio output buffer given by
the CPAL library as callback as parameter, and subsequently played out. The
items is also removed from the ring buffer. If the buffer is empty, there is just
silence (value zero is copied to audio output buffer).

The communication thread reads incoming datagrams in a loop. Because we use a
connected UDP socket we can use the `recv` function varian that does not return
the source address of the datagram (because we know it is always the same). The
datagram payload is passed to Opus decoder that returns a vector of numeric
values representing the decoded audio samples. These are converted to float
values between -1.0 and 1.0 that CPAL uses for playing, and pushed to the ring
buffer, from there the output callback can then find the values.

If the ring buffer is full, incoming datagram is dropped. The ring buffer
is allocated to hold `sample_rate` samples, i.e., one second's worth of voice.

### Improving quality of audio transfer

The example application is very simple in its audio processing. If UDP datagram
is lost in the network, or if the datagrams arrive in the wrong order at the
destination (not uncommon in Internet communication), that causes distortion in
the voice. The Opus decoding maintains decoding state across multiple frames, so
the distortion can be longer than the length of single piece of audio in the
datagram.

To mitigate the effects of reordering and variable delays, the application could
be improved by adding a **jitter buffer**, that is used to delay incoming
datagrams for a short while, giving additional chance for a delayed data to
arrive at the correct place in the data stream. This trades a bit of additional
delay in audio playback for possibly improved playback quality. Detecting out of
order datagrams would require that we add some protocol headers to the UDP
datagrams, to indicate a sequence number or sending timestamp of the packet (or
both). Opus is also able to apply "packet-loss concealment" algorithm to
mitigate the user-perceived distortion caused by the lost packet. In order for
this to work, it would need to know about the loss event, which is only possible
if we have sequence numbers with the datagrams.

The IETF has specified the **Real-time Transport Protocol (RTP, [RFC
3550](https://datatracker.ietf.org/doc/html/rfc3550))** on top of UDP for audio
and video delivery. RTP adds a header containing a sequence number, a media
timestamp, and identifiers for the source and payload format. If it had been
implemented, in our example the RTP header would precede the encoded Opus audio
inside each UDP datagram.

RTP does not itself guarantee delivery or implement a jitter buffer: the
receiving application uses the header information to reorder packets, decide
when to play them, and detect when audio needs concealment. The accompanying
**RTP Control Protocol (RTCP)** provides reception reports, including packet
loss and arrival-time variation, which sendong application can use to monitor
quality and adapt transmission rate to match the network capacity.

## Network Address Translation and firewalls

The first module shortly discussed different types of IP addresses. When
communicating over the Internet, only global IP addresses can be used.
In the 32-bit IPv4 address space there is only a limited number of
global IPv4 address available. The available address blocks are already assigned
to different network operators, that further distribute them to their customers.
At the same time the number of Internet hosts (and needed IP addresses) is
increasing with mobile devices, different IoT gadgets and virtual servers at
the datacenters.

To tackle the address limitation, in many cases the end hosts are using private
IP addresses that do not need global coordination and can be assigned to the
hosts by the local network operator. However, IP packets with private addresses
cannot be forwarded to the Internet. Therefore the local access gateway needs to
rewrite the private IP addresses in the IP header into a public IP address that
can be passed forward. This is call **Network Address Translation (NAT)**. These
days NAT has become very common, for example most WiFi routers use NAT, because
the selection off wireless hosts connected varies over time, and the addresses
need to be allocated dynamically, but for example home users are given only one
public IP address by the network operator.

Typically several hosts with different private addresses in the local network
share a public address that is visible in the Internet. The gateway needs
therefore to maintain state of the address mappings it has made. It manages a
mapping table in its memory with source and destination IP addresses and ports
before and after translation, to be able to do the same address conversion to
the consecutive packets belonging to the same connection. Because multiple hosts
may share the same publicly routable IP address, in some cases also the port
number needs to be reassigned, to distinguish between connections from to
different private addresses that happen use the same local port number in the
private side of the network. Two distinct connections cannot use the same IP
address-port pair.

The below picture shows a case with two hosts behind a wireless gateway, using
private IP addresses (`192.168.1.2` and `192.168.1.3`). The gateway has been
assigned a public IP address `195.148.2.7`. In this example both hosts open a
TCP connection to server at public IP address `128.4.64.10`, and happen to bind
the same local TCP port 5000 for connection. This is possible, because they are
separate hosts, unaware of one another. The gateway router makes a NAT
translation to the source IP address of the outgoing packets, replacing the
public IP `195.148.2.7` in place of source address. Because now the two
connections would have the exactly same source IP/port pair, also one of the
ports need to be modified to distinguish the two connections. The gateway stores
these mappings to its memory, so that the reverse mapping can be done for the
response packets. To the server it seems that the connections come from the same
source address and same host. One effect of NAT is that it hides the local
topology and different hosts behind the NAT.

![NAT example](/images/udp-nat.svg){: width="100%" .center-img }

If the gateway was rebooted (intentionally or because of some power outage), the
mappings would be lost, and all connections traversing the NAT would stop
working. In the originally designed end-to-end Internet that should not happen,
because the intermediate forwarding hosts would not have per-flow state.

In the history of Internet protocol engineering, network address translation has
introduced difficulties and complication to the protocol design. One of the
guiding design principles of the Internet protocols has been the **End to end
principle**, i.e., leaving the state management and related complexity at the
connection end hosts, to the transport layer, and keeping the network simple,
without maintaining traffic flow state. This has been important to support the
scalability and robustness of the Internet technologies. The address translation
and per-connection mappings can be seen to violate this principle, and has
affected how protocols can be designed.

The earlier Internet protocols, such as historic FTP (for file transfers) or
**SIP (Session Initiation Protocol, [RFC
3261](https://datatracker.ietf.org/doc/html/rfc3261))**, a signaling protocol
for setting up and managing conversational sessions, e.g. for voice over IP
communicated IP addresses and TCP/UDP ports of the endpoints so that direct
connections could be established between them. When NATs are present, such
direct peer-to-peer connections are not possible, because the actual private
address is not routable or visible to the other end.

More specifically, there are a few challenges:

- A host can learn the local IP address and port using the `local_addr()`
  function. If host tries the communicate this address in a protocol message
  such as in SIP, and there is a NAT on a connection path, the receiver will get
  a "wrong address", as the actual globally reachable address is only set at the
  NAT gateway. The **local host does not have any direct way of knowing what its
  public address (and port) is**, without using some additional protocol tricks.

- Even if the address was known by the remote end, it is typically **not
  possible to open a connection from the public side** of the NAT gateway,
  because the NAT is **missing the address translation state**, that is normally
  established only when connection (or UDP session) is opened from inside the
  private network.

- Because the NAT gateway needs to maintain per-connection state, it needs to
  have a way to clean up the state at some point. With connection oriented TCP
  protocol this is easier, because opening and closing connection is signalled
  using the SYN and FIN packets. However, with UDP we don't have such signaling
  data. Therefore the NAT gateway maintains per session timer for each flow that
  goes through it, and **after a period of inactivity, cleans up the translation
  state using a timer**. If application has longer periods of inactivity without
  needing to send data, it should therefore send **periodic keep-alive
  messages** to keep the NAT translation state active. This is needed also with
  TCP, because sometimes connections disappear silently, for example when a host
  is powered off or restarted. Therefore also applications using TCP need to
  send periodic keep-alive messages if there is a possibility that they are used
  behind a NAT.

It is possible, however, to configure static port forwarding mappings to a NAT
gateway, to enable a server host inside the private network so that
per-connection state is not necessarily needed. Obvious, to do this one needs to
have access to configure the NAT settings, which typically is possible only for
the system administrators.

Even network address translation is not applied on a connection path, most
organizational networks have a stateful **firewall** on the connection path that
aims to block unwanted traffic from the local network. Even though firewall does
not modify the addresses or ports, it shares the other problems caused by
keeping the per-connection state: typically only outbound connections started
from inside the local network are possible, i.e. one cannot start a network
server inside the private network without some additional configuration at the
firewall, and the connection state needs to be refreshed over longer periods of
inactivity.

One way to enable communication between hosts behind NAT is to use a **relay
host** with a publicly reachable IP address. In this case the endpoints send the
UDP datagrams to the relay server, creating NAT mappings at their gateways that
allow the relay to send datagrams back to them. All communication happens via
the relay, that forwards the datagrams to the intended recipient. The cost of
this approach is that it increases delay because the relay carries all
the forwarded traffic. Someone also has to make such relay server available in
the Internet. **TURN ([RFC
8656](https://datatracker.ietf.org/doc/html/rfc8656))** is a standardized
protocol for this purpose.

The below picture illustrates the situation with relay server. For a
peer-to-peer session (e.g. voice chat), it would be most efficient if `host 1`
and `host 2` could communicate directly. Because both are behind NAT, this is
not possible, but both need to send the first packet to the public network to
create a valid NAT mapping to the gateway. For this to be possible, we need a
relay server in the public Internet to receive this datagrams, and pass them on
to the intended receiver.

![Relay server](/images/udp-relay.svg){: width="100%" .center-img }

<div class="assignment-frame" markdown="1">

## About assignment

This module does not have a related assignment. Good luck with the project!

</div>
