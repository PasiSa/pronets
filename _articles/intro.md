---
title: Introduction
---

This module covers overview of the course arrangements, Computer networking
basics needed on this course, and introduces some useful network tools.

## Arrangements

Specific day-to-day arrangements are discussed on opening lecture and in
MyCourses. These GitHub pages are focused on the course contents.

Course approach: as we progress through this material, you will develop a fully
functioning client-server network application.

Course structure overview: _TBD_

## TCP / IP Basics

We assume that you have done an elementary computer networks course, such as
ELEC-C7241 Tietokoneverkot of ELEC-C7420 Basic principles in networking. Here is
a quick summary of the relevant concepts, however.

Network devices are connected to the global Internet using IP protocol. There
are two versions of the protocol, the old version, IPv4 is still widely used in
most locations. It has its limitations however, particularly, only 32 bits are
available for IPv4 address, which is not sufficient for present day needs. IPv6
was developed, with 128-bit addresses. Its deployment has taken time, however.
Currently 40% of Internet traffic uses IPv6.

On top of IP, the TCP protocol is most commonly used to set up a reliable
communication pipe between two Internet hosts. One end of the connection is the
**server** that listens to incoming TCP connections from clients at a well-known
and agreed IP address and TCP port. TCP port is used to multiplex multiple TCP
connections between hosts. The port is also used to as well-known identifier for
an Internet service. For example, ports 80 and 443 are used for insecure and
secure HTTP protocol (i.e., web transfer), and port 25 has been used for the
SMTP protocol (transferring Emails).

Congestion control and flow control, reliable byte streams

### UDP

Also UDP....

### DNS

Uses UDP port 53, modern variants use also other protocols.

## Network tools

### Netcat

### Dig

### Wireshark

_TBD..._
