---
title: Introduction
---

This module covers the computer networking
basics needed on this course, and introduces some useful network tools. We
assume that you have done an elementary computer networks course, such as
_ELEC-C7241 Tietokoneverkot_ or _ELEC-C7420 Basic principles in networking_.
Therefore, this section is just a brief reminder of the concepts that are
relevant to understand when going forward with this course.

<div class="objectives-frame" markdown="1">

**Objectives for this module:**

- You will get a **refresher on how TCP, UDP and IP basics work** and how IP
  addresses are assigned. These may be familiar from earlier courses already,
  but it does not refresh your memory on them.

- You will **learn basics of Domain Name System (DNS) service**. It is arguably
  the most important service in the Internet, and without it the Internet would
  not be usable.

- You will **get familiar with basic network analysis tools**, such as
  **Netcat** for sending and receiving data to chosen network destination,
  **Dig** tha can be used to inspect the DNS records, and **Wireshark** that is
  used to see the contents of network packets, including their headers at
  different protocol layers.

- The work on this course is maintained in a Git repository. In this module you
  will **learn the basic functions of Git**, and how to set up a Git repository.

</div>

## Overview of the course

The course is divided into 9 modules with dedicated topics. In most modules you
will work towards a client-server project of a chosen topic and enhance it
according to the theme of the module. Typically, in each module you will need to
implement some new code (or modify earlier code), and do a written assignment
related to discussing and analyzing your implementation. We do not evaluate and
grade the details of your code, but the reflection and analysis you have done in
the written reports.

This first module covers the basic fundamentals of the two versions of Internet
Protocol, **IPv4** and **IPv6**, and the transport protocols **TCP** and
**UDP**, and the **Domain Name System (DNS)**, that is essential to all Internet
services to function. We also cover commonly used network diagnostics and
analysis tools, and introduce the **Git version control system** that will be
used to manage the projects during the course. You will practice the use of
**Wireshark** and and **netcat** for a simple HTTP interaction.

**Module 2** starts with introduction to basic concepts of the **Rust programming
language** that is used in examples throughout the course, and which we recommend
to use also in the assignments. Using Rust you will implement a simple TCP
client application that makes a HTTP request, and you will do a small practice
assignment interacting with our course server used during the course.

In **Module 3** you will implement a basic TCP server using Rust, without yet paying
much attention to efficient handling of multiple concurrent clients or
performance overall. We will also discuss basics of **Docker** containers and
you will implement a simple TCP server and build a Docker image out of it. Using
a given API you will register your implementation to our course server, that
will build and run the image at a public IP address and agreed port.

**Module 4** starts the actual project that will be build aside the assignments
during rest of the course. You have some freedom to choose the topic and details
of the implementation, but all projects need to follow certain protocol
principles and common protocol messages documented in this module. This allows
some form of interoperability testing between different projects that are
deployed as Docker containers on our course server. You will implement some of
these core messages in your project implementation.

**Module 5** is focused on testing and observing the implementation behavior
and performance. We will get familiar with the testing framework provided by
Rust, and some tracing tools to analyze the software behavior and performance.

**Module 6** discusses more advanced forms for handling larger number of client
sessions concurrently and efficiently at the server. You will learn to implement
multi-threaded server and handle shared data safely in concurrent environment.
You will get familiar with asynchronous programming model supported by Rust, and
the popular **Tokio** library to manage asynchronous system.

In **Module 7** we will add security to our projects. From this point on, all
projects must use **Transport Layer Security (TLS)** in communication between
client and server implementations and in communication with the course server
APIs. We will learn to use **JSON web tokens** for authentication and
authorization between clients and server.

**Module 8** discusses **UDP and real-time communication**. You will add a
real-time component to your software that uses UDP instead of TCP. **Module 9**
is left for advanced topics.

## Internet Protocol (IP)

Network devices are connected to the global Internet using the IP protocol.
There are two versions of the protocol, the old version, IPv4 is still widely
used in most locations. It has its limitations however, particularly, only 32
bits are available for IPv4 address, which is not sufficient for present day
needs. IPv6 was developed, with 128-bit addresses. Its deployment has taken
time, however. Currently about 40% of Internet traffic uses IPv6, according to
[Cloudflare's Radar service](https://radar.cloudflare.com/). There are
differences in deployment based on the global region, though.

The below text gives some reference to RFC documents related to discussed
protocols. RFCs are the specifications of the Internet protocols, specified by
the **[Internet Engineering Task Force (IETF)](https://www.ietf.org/)**. The
IETF standardization process is open and public, and participation is possible
for anyone.

### Addressing

IP packets carry source and destination addresses in their protocol header.
There are different kinds of addresses, based on the scope they are used.
Typically a computer machine can have multiple IP addresses in use at the same
time, for different scopes, and because it might be connected to multiple
networks. A common case example is a wireless device that can have both WiFi and
cellular 5G device interfaces. These are typically assigned a separate IP
address. A host can also have IPv4 and IPv6 addresses in use at the same time.

The common notation used for IPv4 addresses is by four 8-bit decimal numbers
separated by space, e.g. **151.101.245.91** for _www.aalto.fi_. IPv6 addresses
are represented using a series of 16-bit hexadecinal values, separated by colon,
e.g. **2a04:4e42:3a::347** for _www.aalto.fi_ (so you can access Aalto web pages
either using IPv4 or IPv6). This IPv6 address is the same as
**2a04:4ee4:003a:0000:0000:0000:0000:0347**, but it is agreed that the repeating
zeros can be compressed into double colon, for convenience.

IP addresses are split into two parts. The most significant part stands for
network prefix and is shared by all hosts in the same local network. The least
significant bits separate the different hosts in the network. Each host must
have different address and there must not be overlaps. **Classless Inter-Domain
Routing** indicates the network prefix and its length in the following way:
**164.90.208.0/20**. Therefore we can now see that addresses **164.90.208.10**
and **164.90.209.14** belong to the same network, and do not need to be passed
to network router for forwarding.

**Dynamic Host Configuration Protocol (DHCP, [RFC
2131](https://datatracker.ietf.org/doc/html/rfc2131) /
[Wikipedia](https://en.wikipedia.org/wiki/Dynamic_Host_Configuration_Protocol))**
is a way to assign IP addresses to the machines in the local network. It is
based on a server that keep track of available IP addresses and assigns a free
address when an address request arrives. When a new host is connected to the
network, it makes a DHCP query to learn its IP address, and for example the
address of the local DNS server. IPv6 also often uses **stateless address
autoconfiguration ([RFC 4862](https://datatracker.ietf.org/doc/html/rfc4862) /
[Wikipedia](<https://en.wikipedia.org/wiki/IPv6#Stateless_address_autoconfiguration_(SLAAC)>))**.
It is based on the assumption that layer 2 MAC-addresses (typically 48 bits) are
likely unique in the local network, and the MAC address is used to compose the
64-bit host part of the IPv6 address, where as the network part, along with
routers IPv6 address, is learned from a **IPv6 router advertisement** message.
In IPv6 the network part of the address is commonly 64 bits and the host (or
interface identifier) part is 64 bits.

There are different kinds of IPv4 and IPv6 addresses:

- **Host-local (or loopback) addresses (IPv4: 127.0.0.1, IPv6: ::1)** are
  intended for communicating between applications in the same computer. These
  are useful especially for local development and testing, and these IP packets
  will never leave from the local computer system, even to the local network.

- **Private addresses (IPv4: 10.0.0.0/8; 172.16.0.0/12; 192.168.0.0/16; IPv6:
  fc00::/7)** are used in local networks (different machines under home WiFi,
  internal networks in offices, virtual machines or containers in virtual
  network, etc.). IP packets with these addresses should not be routed to the
  Internet, but are intended for communication between machines in the local
  network. These addresses are popularly used, because they can be easily
  assigned ad-hoc to local networks. Commonly the network router (e.g. Home WiFi
  access point), translates the network address into a public address for
  packets destined to the Internet.

- Majority of other addresses are **global Internet addresses** that can be
  forwarded by routers towards another host anywhere in the Internet. These
  addresses need to be allocated from the network operators.

In addition, there are other types of addresses, for example for multicast and
broadcast communication, which we do not need on this course. For example the
Wikipedia article on [IPv4](https://en.wikipedia.org/wiki/IPv4) or [IPv6
addresses](https://en.wikipedia.org/wiki/IPv6_address) discusses more about
these.

### About packet transmission

Even thought there are many kinds of link layer protocols with varying
characteristics, commonly the IP packets are transmitted over one of the IEEE
802 Local Area Network protocols, for example the fixed Ethernet (802.3) or
Wireless LAN / WiFi (802.11), which commonly assume 1500-byte IP packets. This
is called the **Maximum Transmission Unit (MTU)**. If there are links along the
communication path that assume smaller packets, the IP packets either need to
fragmented into multiple pieces, or a router needs to send a notification the
packet sender requesting smaller MTU for that destination. The latter is more
common in current Internet, because fragmentation is harmful for performance.
Because 1500 bytes is very common MTU, in many cases this is not needed.

**Internet Control Message Protocol (ICMP)** is used to deliver different kinds
of diagnostics and error messages, such as "Packet too Big" in the
above-described case, or "Destination unreachable" if the packet cannot be
delivered to destination. ICMP is also used commonly by the **ping** tool, to
test that the destination is reachable, and to measure the round-trip delay to
destination. _Ping_ sends series of _ICMP Echo Request_ messages that trigger
_ICMP Echo Response_ at the receiver.

## Transmission Control Protocol (TCP)

On top of IP, the **TCP protocol ([RFC
9293](https://datatracker.ietf.org/doc/html/rfc9293) /
[Wikipedia](https://en.wikipedia.org/wiki/Transmission_Control_Protocol))** is
most commonly used to set up a reliable communication pipe between two Internet
hosts. TCP provides an abstraction of reliable byte stream to the upper protocol
layers. It does not preserve the message boundaries as sent by the application,
which needs to be considered when designing the application communication
operations. Applications just send data first to TCP's send buffers, from which
TCP then splits them into segments based on the network MTU.

TCP is a connection-oriented protocol between two end points: connection needs
to be opened first by the client to a specified IP address and TCP port, before
data can be sent. Other end of the connection is the **server** that listens to
incoming TCP connections from clients at a known IP address and that TCP port.
The connection begins with **three-way handshake** that is initiated by the
client, and it is usable only after the handshake is complete. After this,
either end can send data independently, although a common pattern is that the
client starts the conversation (e.g. in HTTP protocol).

Like IP address, the 16-bit TCP port is specified for both ends of the
connection, and is used to separate different TCP connections between hosts. The
server-side port is also used to as well-known identifier for a particular
Internet service. For example, ports 80 and 443 are assigned for insecure and
secure HTTP protocol (i.e., web transfer), and port 25 has been used for the
SMTP protocol (for transferring Emails). The well-known port assignments are
managed by the **[Internet Assigned Numbers Authority
(IANA)](https://www.iana.org/assignments/service-names-port-numbers/service-names-port-numbers.xhtml)**.
Typically, when viewing TCP packets in a network trace, the server-side port
indicates the service that is used for the connection, but the client side port
seem random, typically a number above 48000, which is the number range reserved
for automatically chosen client ports. Typically client implementations do not
choose the client port, but the operating system automatically chooses an
available one (but it is also possible to manually select it).

There is delay in delivering packets between client and server. In addition to
the limitations of the physical world (propagation delay and delay of processing
and queueing packets at network routers), the TCP sender limits the sending rate
of packets based on its congestion control and flow control algorithms. Because
TCP guarantees reliable delivery of ordered data stream, if a packet is lost in
the network (a common situation), TCP receiver does not deliver data to
application, before the missing piece is received from subsequent
retransmission. To receiving application this may appears as variable delays in
data delivery, and sudden pauses in transmission. Also this is a factor that an
application designer needs to take into account.

## User Datagram Protocol (UDP)

**User Datagram Protocol (UDP)** is a simple protocol format on top of IP
packets to send fixed-size datagrams to a destination. As with TCP, in addition
to IP address, the destination is identified by a 16-bit UDP port. Compared to
TCP, UDP is very simple: it is unreliable, connectionless, and stateless and
does not make guarantees of data delivery. Application can just start sending
UDP datagrams, but does not know if they reach anywhere, unless the receiving
application sends something back. Due to its properties, UDP can be used for
lightweight signaling where reliability is not required, or real-time streaming
uses such as audio/videoconferencing or online games, where variable delays are
more harmful than a possible data loss. Because of its connectionless
properties, UDP can also be used for IP broadcast and IP multicast such that
single IP packet will have multiple receivers. This is useful, e.g. for service
discovery in the local network.

## Domain Name System

Well-know ports are also specified for UDP, and perhaps the most commonly used
is port 53, that is assigned for the **Domain Name System** ([Wikipedia
article](https://en.wikipedia.org/wiki/Domain_Name_System)). Domain name system
is a hierarchically organized name database that can map the domain names into
IPv4 addresses, IPv6 addresses or certain commonly used network services.
Technically it is an application on top of UDP and IP, but it is so inherently
built into many of the current network APIs, that this distinction is not always
very obvious to application programmer, not to mention the users of the
application.

Normally, when client wants to connect to particular network server, it does not
know its IP address. Therefore, before opening the TCP connection, the client
system needs to make a DNS query, where it indicates the name of the system it
wants to connect, e.g. "**www.aalto.fi**". The client also indicates the type of
query it wants to make, for example an **'A' query** asks for an IPv4 address
corresponding to a name, and **'AAAA' query** asks for an IPv6 address
corresponding to a name. Each system may have multiple IP addresses
corresponding to a name. This feature can be used for load balancing or
robustness through redundancy, in case some of the server are temporarily
unreachable. These days it is also common that the client system triggers both A
and AAAA queries at the same time, if it is not known which IP address family is
available.

The Domain Name System is distributed, hierarchic and heavily replicated. The
**root zone** that holds the **top-level domains (TLD)** (such as '_.fi_' or
'_.com_' is distributed into number of root servers across the world that hold
the **NS type resource records** for the authoritative name servers for each
top-level domain. These authoritative name servers resolve the next level of
domain names, until the actual IP address or other queried resource type is
resolved.

Because the hierarchic resolution would cause delay if performed separately
every time (each host needs many name queries in short period of time), and
would cause burden towards the root, the names are cached along the request
path. Quite commonly the response for a name query comes from nearby name
server, if it represents a commonly used name. For this reason the DNS
**resource records** also have a lifetime for how long they can be stored in the
cache.

It is useful to understand that a record in particular place in name hierarchy
does not necessarily have any connection to how the resolved IP address is
located in the actual network topology. For example, when writing this, name
'_www.aalto.fi_' resolves to alias (CNAME record)
'_dualstack.n.sni.global.fastly.net_' that resolves to IPv4 address
_151.101.245.91_ or IPv6 address _2a04:4e42:3a::347_, depending on whether we
made a query for 'A' type record or 'AAAA' type record, hinting that Aalto web
pages are hosted by an external web hosting service, that might actually be far
away from the servers at Aalto campus.

The below image illustrates how the DNS resolution process typically works, and
why it may take time to get the actual response. The picture is taken from blog
article "[How DNS Resolution
Works](https://dev.to/swadesh_chatterjee_b35563/how-dns-resolution-works-55jm)".

![DNS resolution](https://media2.dev.to/dynamic/image/width=800%2Cheight=%2Cfit=scale-down%2Cgravity=auto%2Cformat=auto/https%3A%2F%2Fdev-to-uploads.s3.amazonaws.com%2Fuploads%2Farticles%2Frklsx8po24i2biaoscvm.png){: width="90%" .center-img }

The common pattern is that client sends a DNS query to the local DNS server. The
local DNS server/resolver starts resolving the name hierarchy starting from the
top-level domain. There are a number of well-known root server that store
information about the top-level domains, which is resolved first. Then, a name
server for that TLD is connected to resolve the address of the authoritative
name server for the requested domain. Finally the local DNS server finds out the
actual IP address of the server, that is delivered to the client. Because DNS
resource records are cached, for commonly used names some of these steps can be
skipped.

## Network analysis tools

We will now take a look at few Network tools that are useful for investigating
the network behavior. We take a look ar **Dig** to inspect DNS records,
**Netcat** that sends and receives data over TCP or UDP, and **Wireshark** that
capture and examine the packets transmitted by network applications.

Dig and Netcat are command-line tools. On Linux, they can be run in a terminal
and installed with the system's package manager if they are not already
available. On Mac, they can be run in the **Terminal** application and are
included in the standard installation. On Windows, the commands in this
material should be run in a Linux terminal provided by **Windows Subsystem for
Linux (WSL)**. Wireshark is a graphical application and must be installed
separately on Linux, macOS, or Windows.

### The 'Dig' tool

**Dig** (Domain Information Groper) is a command-line tool for making DNS
queries. It shows both the records returned by a DNS server and information
about the query, such as its response status and duration. The most relevant
part of the output is usually the `ANSWER SECTION`, which lists the matching
resource records and their time-to-live (TTL) values.

The basic command takes a domain name and, optionally, the requested record
type. For example, here is an example of IPv4 query for `www.aalto.fi` and a
possible response:

```bash
$ dig www.aalto.fi A

; <<>> DiG 9.10.6 <<>> www.aalto.fi A
;; global options: +cmd
;; Got answer:
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 16279
;; flags: qr rd ra; QUERY: 1, ANSWER: 2, AUTHORITY: 0, ADDITIONAL: 1

;; OPT PSEUDOSECTION:
; EDNS: version: 0, flags:; udp: 1232
;; QUESTION SECTION:
;www.aalto.fi.			IN	A

;; ANSWER SECTION:
www.aalto.fi.		373	IN	CNAME	dualstack.n.sni.global.fastly.net.
dualstack.n.sni.global.fastly.net. 27 IN A	199.232.173.91

;; Query time: 8 msec
;; SERVER: 192.168.0.1#53(192.168.0.1)
;; WHEN: Thu Jul 30 18:33:09 EEST 2026
;; MSG SIZE  rcvd: 104
```

The above shows the content of received DNS response in a UDP packet. It says:

- The message ID was 16279, and it contains 1 resource record entry in QUERY
  section, and 2 RR entries in answer section. The DNS answers echo back also
  the query, i.e. the protocol is stateless.
- The first record is of **CNAME** type. This represents an alias to a
  **canonical DNS name** of the server that actually serves the data for
  _www.aalto.fi_. We see from the canonical name that the web content is
  actually served by company called [Fastly](https://www.fastly.com/). Lifetime
  of this record entry is 373 seconds. The DNS records are cached to reduce the
  number of actual request messages sent, and this information is used by caches.
- The second record is the actual IPv4 address for
  _dualstack.n.sni.global.fastly.net_, with 27 seconds of lifetime. This
  lifetime is shorter, because the operators often do load balancing in their
  datacenters and do not want the addresses to linger to long in network caches.
  If we did another request half a minute later, we would likely receive a
  different IP address, for another datacenter host that serves the web
  content for Aalto.

In some cases there could be multiple IP address records in the answer section.
These are to give backup options for the client that is to establish the
connection, for better robustness, should it happen that the first address(es)
are unresponsive.

Finally there is some general information about the response: it took 8
milliseconds, and arrived from a private IP address _192.168.0.1_, UDP port 53
(as expected for DNS). This is my home router in this case, that uses private
IP network for the devices in my home (a very common case).

To do similar query for IPv6 address, you just do:

```bash
$ dig www.aalto.fi AAAA
```

Dig can also inspect records used for purposes other than host addressing. An
`MX` query lists the mail servers for a domain, while an `NS` query lists its
authoritative name servers (but these are not so important during this course):

```bash
dig aalto.fi MX
dig aalto.fi NS
```

### Netcat

**Netcat** is a command line tool (command shortly: `nc`) that opens a socket
and forwards the user input to the socket, and vice versa. It can open both
server and client sockets, both for TCP and UDP, being a good tool e.g. for
testing and debugging the behavior of network software.

For example, the following command opens a TCP connection to the HTTP port
(port 80) of _www.aalto.fi_ and sends a simple HTTP request:

```bash
$ nc www.aalto.fi 80
GET / HTTP/1.1
Host: www.aalto.fi
Connection: close

```

After we type the command and its arguments (the first line), _netcat_ will
first resolve the IP address for _www.aalto.fi_, and then opens a TCP connection
to port 80 (unecrypted HTTP) on this address. After this, everything we type to
terminal will be sent to the server. In this case we make a simple HTTP request:
We do a GET request for `/` assuming HTTP version 1.1. _Host_ is an HTTP header
that tells the web server we are interested in website _www.aalto.fi_. One
server host can virtually server multiple web sites (like e.g. Fastly does),
which is why this information is needed. The _Connection_ header asks server to
close the connection after response has been sent. After the headers there needs
to be one empty line (just press enter). After this we should see the HTTP
response from the server on our terminal.

Netcat server can be started with the `-l` option at a particular port:

```bash
nc -l 6000
```

This starts a TCP server socket that listens for incoming connections at port 6000.

Now, if you open another terminal window and connect to the localhost address,
port 6000, you can start communication session between the client and server
socket. Try it: you should see typing passed back and forth between the two
windows.

```bash
nc 127.0.0.1 6000
```

By adding `-u` command line option, the same can be done using UDP instead of
TCP (which is the default).

### Wireshark

**Wireshark** is a network analysis tool that captures all packets going through
a network device and lets user analyze them in a graphical user interface. You
can download an installation package for your system in [Wireshark home
page](https://www.wireshark.org/). Note that to capture network packets,
Wireshark needs to be run using system admin privileges.

When you start Wireshark, you will be shown the following view. The bottom part
of the window shows the network interfaces in your system from which you can
capture packets. The screenshot is taken from my Mac laptop, that contains
various local interfaces, but worth noting are "**en0**" the local wireless
interface through which all the Internet traffic goes, and "**lo0**", the
loopback interface used for host-internal communication.

![Initial view in Wireshark](/images/intro-wireshark-ifaces.png)

When double-clicking one of the interfaces, for example "_en0_", all packets
going through the interface are shown in the following view, each on their own
line. You will notice that even on a local laptop machine, there is a lot of
communication going on, and within a few seconds there are hundreds of packets
captured.

To actually analyze protocol behavior from the hundreds or thousands of packets,
one needs to set a packet filter to select the interesting traffic. Wireshark
support flexible notation for selecting packets, e.g., based on different
protocol field values, and logical operations between multiple criteria. The
below screenshot shows a simple selection based on UDP (source or destination)
port 53 on the packet header. The screenshot shows the result after using _dig_
for making an A query for _www.aalto.fi_.

![Packet capture view](/images/intro-wireshark-dns.png)

The top part of the window shows each matching packet on its own line: there is
the DNS query packet, and the DNS response packet. I have selected the latter
packet, and the details are shown on the bottom half of the window. On the left
there is a readable description of the headers on different protocol layers,
starting from layer 2 (Ethernet --> IPv4 --> UDP --> DNS). On the right side the
same content is shown as raw hex dump. We can see much of the same information
as we saw in the _dig_ output.

## Using Git

**[Git](https://git-scm.com/)** is the most prominently using version control
system today, and majority of current open source software projects are using
it. If you work on any software development in future, you will almost certainly
need to know how to use git. Also on this course we use git for maintaining the
software developed during the course.

Git is a distributed system: there is a main **repository** on a server
accessible to all developers, and the developers clone a copy of the repository
for their development work. The repository consists of **commits**, events that
have modified the source code (or text, or other kinds of files stored in the
repository). Each commit represents a logical increment that can be synchronized
with the other developers. The synchronization happens through **push** actions,
where the changes in local repository is pushed to the shared server, and
**pull** actions, where developers can download the recent commit events from
the server to their own repository.

The picture below shows how this might look on this course: Jukka and Liisa have
a project team, and they have created a common repository on _version.aalto.fi_
server. Both of them have cloned a local copy on their machines. In addition to
the repository (which contains the git commits and other metadata), there are
local work copies of the source files belonging to the project. On this course
we also have our course server that also clones your git repositories, and
builds and runs the server implementations in a Docker container at a publicly
accessible address, so that other students can test them.

![Git overview](/images/intro-git.svg){: width="90%" .center-img }

### Setting up a git repository

There are publicly available Git hosting services such as
**[GitHub](https://github.com/)** and **[GitLab](https://about.gitlab.com/)**,
that come with a web user interface for setting up and operating with the
repositories. Particularly, if you manage a publicly accessible open source
project such service is useful, as it offers various services for bug reporting
and workflow management, in addition to the basic git repository service. On
this course we primarily use Aalto's own
**[version.aalto.fi](https://version.aalto.fi/)** service, which is based on the
GitLab software, and can be used with the Aalto user accounts. It is missing
some services that e.g. GitHub provides, for example related to **continuous
integration**, and if you therefore want to use GitHub for your project, it is
possible, if you agree about it with the course staff.

After signing in to _version.aalto.fi_, new repository can be created by
clicking a "plus" sign on the top right part of the page. Choose "Create blank
project", and start filling in the project details. First, pick a name for your
project. The system proposes a project URL for you based on the name. **Make
your project private** at this point, so that it is only accessible to you and
your project partner, and the course staff. Leave "Initialize with README"
checked, so that you have some initial content on your new repo. After this you
just click "Create project".

### Setting up ssh keys

Git is mainly used from command line, although the IDE development environments
provide graphical user interface for common operations. Your local command line
communicates with the public server using **ssh** protocol, that is often used
for remote shell access, but can also be used as authenticates secure
communication channel between systems.

To use ssh, you'll need to create a key pair for authentication, with the public
key configured at the Git server, and the private key kept securely on your own
machine. If you have done this before, you can use your existing keys. If this
is new to you, our git server has
**[instructions](https://version.aalto.fi/gitlab/help/user/ssh.md)** on how this
is done. Follow those.

### Cloning the repository

After the keys have been set up, you should be able to clone a copy of the
repository to your own machine:

    git clone git@version.aalto.fi:psarolah/my-repo.git

Where you will replace the actual URL with your correct repository. You can find
the URL from your repository's main page, under blue "Code" button. Choose the
**ssh** version.

Now you should have the local copy of the README file on your machine, and you
can start adding new files as needed in the project.

### Basic operations

Graphical development environments, such as **VScode** can support basic git
operations, look for the left-side bar and icon with tree-like symbol (typically
third from up). If you want to operate from command line, here are the most
important basic operations.

After you have done some development, at some point it is time to make a commit
event. You don't want to commit something that is known not to work or compile,
so before committing, check that the code works at least to the extent that
possible other developers can continue from there.

You mark the files you want to commit by

    git add <file1> <file2>...

These include also new files. If you have added new subdirectories, just mark
the files inside them.

Then you make the actual commit event:

    git commit

A text editor will open where you can write a short note describing the commit.
This creates a new commit event in your local git repository, but it is not yet
synchornized with the remote. You will do this by:

    git push

That uploads the commits to the server. Now you should be able to see the
changes in _version.aalto.fi_ web interface, too.

If you are working together with someone, it is a good idea to synchronize
possible changes from the server to the local repository by:

    git pull

This is also useful in solo projects, if you are working on a multiple machines
and want to synchronize the work between them.

<div class="assignment-frame" markdown="1">

## Assignment

The first assignment is about setting up the git repository for future tasks,
and to get familiar with the tools discussed in this module.

First, set up a **private** git repository for your course work. Once you have
created the repository, and you have tested that it works, report its URL in the
[MyCourses
questionnaire](https://mycourses.aalto.fi/mod/questionnaire/view.php?id=1528203).
Give read permissions to the repository also to course personnel.

Open Wireshark and start capturing packets from your network interface. Pick a
well-known organization, but not Aalto University (e.g. a company, or another
university than Aalto). Take the following steps and report the outcome in your
assignment report (submitted in MyCourses).

1. Using **dig**, check if the main web page of your selected organization has only
   IPv4 address, or if it also has IPv6 address for serving the web content. Is
   there a CNAME record that would indicate the actual host serving the content?

2. How many DNS packets do you see in Wireshark as a results of above operation?
   Use filter for UDP port 53 to see these packets better.

3. Create a HTTP request using **netcat** to TCP port 80. What kind of response
   do you get? What is the HTTP response code on the first line, and what does
   it mean? (You may use resources in the Internet to find this information)

4. How many TCP packets were transferred back and forth to TCP port 80 as a
   result of this operation. Explain in your own words what happened in each
   packet. (If the HTTP response is large and spans over multiple packets, you
   don't need to explain every packet separately)

Finally, answer the following questions:

- How much time did you use for this assignment?
- What was easy or difficult in the assignment?

</div>
