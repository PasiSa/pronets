---
title: Rust basics and client sockets
---

## Rust basics

The [Rust book](https://doc.rust-lang.org/stable/book/) gives a comprehensive
learning material and overview of the Rust language. Here we summarize the most
important concepts to help you get started with Rust

### Installing Rust tools

### Variables, data types and functions

### Ownership and references

### Structures and their implementations

### Packages and crates

### Collections

## Socket basics for client applications

## Encoding binary data

Byte order, packing and structures

## HTTP basics

Needed in the fist assignment

## Assignment

Implement a program that opens a TCP socket and connects it to
"pronets.dice.aalto.fi" port 10000, and writes message `TST (some string)`.
Then the program should read bytes from the socket.

Implement another function to your program that opens a TCP connections to
"pronets.dice.aalto.fi", port 80, and makes a GET request for `index.html`.

_TBD..._
