---
title: Security
---

The original design of the Internet protocols was based on insecure
communication where the messages could be read by anyone having access to the
communication path. Also today, by default the Internet traffic on top of the IP
protocol is unencrypted and could be read with tools like Wireshark. Anyone
along the connection path could therefore read and modify the messages, for
example by hacking to a local WiFi access router which sometimes are insecurely
designed, unless some countermeasures are made.

Also the domain name system that is used, to map domain names into IP addresses
is based on unencrypted communication. In addition it is heavily distributed and
cached across multiple intermediaries in the network. IP addresses can also be
spoofed, i.e. modified, by an intermediary. Therefore, in the basic insecure
communication we cannot be sure if the server actually is who we believe it to
be. We need to have solution that allows authenticating the server by a trusted
third party.

Since the original design of the Internet, different solutions for security have
been developed. On common solution is **IPsec [RFC
4301](https://datatracker.ietf.org/doc/html/rfc4301)**, that adds integrity
protection and encryption to IP traffic. IPsec is often used in Virtual Private
Networks for organization networks. IPsec typically requires operating system
support, and requires therefore little effort to set up.

Another, probably currently the most popular solution that we are going to
discuss in the following, is the **Transport Layer Security (TLS, [RFC
9846](https://datatracker.ietf.org/doc/html/rfc9846))** protocol. It works on
top of the transport layer, typically TCP, and is used, for example, practically
in all HTTP communication today. TLS is easier to take into use, as it can be
deployed as user-space library as part of the software implementations, and does
not need separate operating system support. TLS does not protect the IP and TCP
headers, however, unless the traffic is tunneled inside a TLS-protected
connection.

<div class="objectives-frame" markdown="1">

**Objectives for this module:**

- You will **understand the security architecture based on Transport Layer
  Security (TLS)**

- You will **learn to implement applications using TLS-secured communication** in
  Rust language.

- You are able to **work with TLS certificates** using OpenSSL tool, for example
  validating and checking the information in the certificate, and to create them.

- You will be **able to use JSON Web Tokens** for authentication and authorization

</div>

## TLS fundamentals

**Transport Layer Security (TLS)** aims to provide three types of security:

- **Confidentiality**: The traffic is encrypted so that outsiders cannot read it.
- **Integrity**: Detecting is data has been modified
- **Authentication**: Verifying that the client is communicating with the
  intended server.

### Certificates

An essential part of the TLS protocol is the **certificate** that establishes
authenticity of the server. Certificate is created and signed by a trusted
**certificate authority (CA)**, and the server sends it to the client when a TLS
connection is established. Certificate includes (among some other things) the
following parts:

- **server name** that should match the DNS name of the server.
- **public key** of the server that can be used to decrypt the encrypted
  messages.
- **validity period** during which the certificate can be used.
- **issuer** name of the certificate authority that has created the certificate.
- **digital signature** that the CA has created, and that can be verified using
  the public key of the CA.

The client then verifies the certificate, for example that the server name
matches the connected server, and that the certificate is still valid. The
issuer should also be trusted by the client. There are generally trusted root
certificate authorities configured to the local system. Root CAs may delegate CA
responsibility to intermediate CAs that then create and provide the actual
certificate to a server owner. In other words, there is a certificate chain that
starts from the server certificate and and should terminate at one of the
trusted root CAs.

For some internal uses, such as corporate intranets that may also be more
flexible and variable, such as used on the server implementation on this course,
getting an actual certificate from a trusted CA may not be practical. It is
typically possible, however, to configure custom local root CAs in such
environments for easier certificate deployment, and this is what we also do on
our course (more about the practicalities a bit later).

_TODO: shortly mention mutual TLS and client certificates, not using on this
course_

### The protocol

When a TLS communication is desired, the client opens the TCP connection
normally, but after the connection establishment, it starts a TLS hanshake that
goes in the following way:

- The client sends **ClientHello** message, where it indicates, for example,
  which TLS versions and cipher algorithms it supports.
- The server responds with a **ServerHello** message where is chooses one of the
  suggested cipher algorithms, provides the certificate and information needed
  for choosing a symmetric key used for the actual data communication.
- The client verifies certificate, and both sides compute a shared secret that
  becomes the symmetric session key used for actual communication.
- Both sides confirm the exchange with "**Finished**" message, and the
  communication continues using the session key.

_TODO: add diagram_

Using symmetric session key has some benefits: it is more efficient to use, and
using a variable key for each session improves the security.

## TLS in Rust

A popular library (or crate) for using TLS in Rust applications is
**[rustls](https://crates.io/crates/rustls)**. It provides the API for all
necessary TLS protocol operations on top of TCP, and can be then used with
normal `TcpSocket`.

### Client operation

We have a modified version of our earlier simple client in
examples directory, under
**[tls-client](https://github.com/PasiSa/pronets/blob/main/examples/tls-client/src/main.rs)**.
We will walk through this example to see how **rustls** works in this simple
client case. Again, you can clone the repository and try the example also on
your own machine.

Like previous examples, we have a few command line arguments, and now an
additional optional command line argument for specifying a root certificate
file, if we want to use such. If you test the program using one of the public
servers (there is an example of `www.aalto.fi` case), this is not needed because
the `webpki-roots` crate contains the commonly accepted root certificates.
However, if we want test this locally, we need to specify our own root
certificate that we can use to validate the incoming server certificate.
Included file `cert/ca.crt` can be used, if you test using the certificate
provided in **tls-server** example. That was used to sign the server certificate
provided with the example.

As a first step, starting on [line
34](https://github.com/PasiSa/pronets/blob/dbf3770c7a4858e826b7782a2c7e496c57863c88/examples/tls-client/src/main.rs#L34),
we separate the server name from command line argument, because it is compared
to the name in incoming certificate, which must match. _rusttls_ has a dedicated
`ServerName` type for holding this.

Starting on [line
44](https://github.com/PasiSa/pronets/blob/dbf3770c7a4858e826b7782a2c7e496c57863c88/examples/tls-client/src/main.rs#L44),
we take different branch depending on whether we use a user-provided root
certificate, or whether we use on of the public certificates. The set of
available root certificates is stored in `roots` variable, of type
`RootCertStore`.

Then we build TLS client configuration from this information ([line
62](https://github.com/PasiSa/pronets/blob/dbf3770c7a4858e826b7782a2c7e496c57863c88/examples/tls-client/src/main.rs#L62)),
and connect the TCP socket normally to the server, which may succeed or fail for
various reasons, for example if server is not reachable. We create a new
**rustls** `ClientConnection` object, which also starts the TLS handshake. If
the handshake is successful and the server certificate is valid, we connect the
TLS ClientConnection to the TCP socket as **rustls** `StreamOwned` type, as
variable `socket`. After this we can use the read and write operations on the
socket normally, except that now all communication happens as TLS encrypted.

Finally, with TLS it is good to close the connection gracefully using the
`send_close_notify` function, as we do on [line
78](https://github.com/PasiSa/pronets/blob/dbf3770c7a4858e826b7782a2c7e496c57863c88/examples/tls-client/src/main.rs#L78).

### Server operation with tokio

A TLS server example can be found under
**[tls-server](https://github.com/PasiSa/pronets/blob/main/examples/tls-server/src/main.rs)**
directory of the course git repository. It similar to earlier [asynchronous
server](https://github.com/PasiSa/pronets/blob/main/examples/async-server/src/main.rs)
example with Tokio, but now uses TLS. For using TLS with Tokio, there is a
separate **[tokio-rustls](https://crates.io/crates/tokio-rustls)** crate that we
are using.

In addition to the listening IP and port, the server now takes also the
PEM-encoded files representiing **certificate** and **private key** that
corresponds the public key in certificate as arguments. The _rustls_ crate reads
these files and sets the server configuration based on these ([line
46](https://github.com/PasiSa/pronets/blob/dbf3770c7a4858e826b7782a2c7e496c57863c88/examples/tls-server/src/main.rs#L46)).
We can use the server certificate (_cert/server.crt_ and _cert/server.key_) and
private key included in the repository. Note that **in reality, you would never
add these files to git repository, and especially the private key should be kept
secret!** (and not readable by other uses on the system). For the sake of the
example, we make an exception here. The certificate is created using the
_ca.crt_ included in the _tls-client_ directory.

Similarly to **ClientConnection** on the client side, **TlsAcceptor** uses the
TLS configuration to turn TCP connection into a TLS stream ([line
51](https://github.com/PasiSa/pronets/blob/dbf3770c7a4858e826b7782a2c7e496c57863c88/examples/tls-server/src/main.rs#L51)).
It is wrapped inside asynchronous reference counter (**Arc**), because each
Tokio task needs to have access it.

The TCP server socket is bound normally, and new connection is first accepted
normally with **TcpSocket**'s `accept()` function, but after that accepted
separately with **TcpAcceptor**'s `accept()` ([line
69](https://github.com/PasiSa/pronets/blob/dbf3770c7a4858e826b7782a2c7e496c57863c88/examples/tls-server/src/main.rs#L69))
that finishes the TLS negotiation. After this the read and write can be used on
the socket as usual.

### Working with certificates

**OpenSSL** is a common tool used for various TLS-related tasks, for example to
review and create certificates (SSL, or Secure Sockets Layer, is the predecessor
to TLS, that is the current standard). It is usually readily installed in many
systems, but if not, for example in Ubuntu Linux you could install it normally
using _apt_: `sudo apt install openssl`

You can review a certificate, for example, in following way on a command line
(for our _server.crt_ certificate):

    openssl x509 -in server.crt -text -noout

As you can see the certificate is not very good: for example, it has
ridiculously long lifetime. But we are only using it for this simple example.

_openssl_ can also be used to verify and show information about certificate at
given network address. For example, we can see that _www.aalto.fi_ has a more
serious certificate:

    openssl s_client -connect www.aalto.fi:443

If you want to create a **root certificate authority** for local testing needs,
you will first need to create a private key:

    openssl genpkey -algorithm RSA -out rootCA.key -pkeyopt rsa_keygen_bits:4096

This key should be kept secret in safe place. Anyone having access to this key
can create certificates under your name.

Then you create a OoenSSL configuration file that specifies information in the
CA certificate:

    [ req ]
    distinguished_name = dn
    x509_extensions = v3_ca
    prompt = no

    [ dn ]
    CN = My Example Root CA
    O = Example Organization
    C = FI

    [ v3_ca ]
    basicConstraints = critical,CA:true
    keyUsage = critical,keyCertSign,cRLSign
    subjectKeyIdentifier = hash
    authorityKeyIdentifier = keyid:always

Then you will create the actual certificate to file _rootCA.crt_. In this case
the private key is in file _rootCA.key_ and the configuration in _root.cnf_.

    openssl req \
        -new \
        -x509 \
        -days 3650 \
        -key rootCA.key \
        -out rootCA.crt \
        -config root.cnf

Once you have the private key for your root CA, you can create the actual server
certificate. Creating the server certificate follows similar steps, although in this case it
is useful to distinguish who does what. The **server administrator** creates a
private key:

    openssl genpkey \
        -algorithm RSA \
        -out server.key \
        -pkeyopt rsa_keygen_bits:2048

The permissions for the private key file, _server.key_, should be set such that
only the owner can read it.

The server owner creates the certificate configuration file, for example:

    [ req ]
    distinguished_name = dn
    req_extensions = server_extensions
    prompt = no

    [ dn ]
    CN = my.server.fi
    O = Example Organization
    C = FI

    [ server_extensions ]
    basicConstraints = critical,CA:false
    keyUsage = critical,digitalSignature,keyEncipherment
    extendedKeyUsage = serverAuth
    subjectAltName = @alternative_names

    [ alternative_names ]
    DNS.1 = my.server.fi
    DNS.2 = localhost
    IP.1 = 192.168.1.20
    IP.2 = 127.0.0.1
    IP.3 = ::1

The server's DNS name and IP address needs to match the CN or alternative names
fields. If you are testing locally, you could just use localhost.

Then the _server administrator_ creates a **certificate signing request** using
the configuration file (_server.cnf_ in this case) and private key. In this
case, this creates a file _server.csr_.

    openssl req \
        -new \
        -key server.key \
        -out server.csr \
        -config server.cnf

The resulting _server.csr_ file contains public key and the information from the
configuration file. It is given to the _certificate authority_ for signing. The
request file is signed using the server administrator's private key, so that the
CA can verify the authenticity of the public key included in the file.

Then the _certificate authority_ creates the actual certificate (_server.crt_ in
this case). i.e. essentially adds its digital signature covering the certificate
information provided by the _server administrator_, and passes it to the _server
administrator_.

## Authentication and JWT

_TODO_

- Authentication vs authorization
- JWT structure
- Signing and verification
- Claims

## Secure application practices

_TODO_

- Secret management
- Password hashing
- Common security mistakes

## Assignment

_TODO_

<div class="assignment-frame" markdown="1">

You (and your team, if you have one) should have been given a JWT token (_TODO:
decided how to distribute these_). Keep the token in secret, and **do not add it
to your git repository**.

Add TLS support for your project communication, and token based authentication
for client connections.

Use a new TLS-encrypted HTTPS endpoint to load your container
(run-docker-secure). It will have one new JSON attribute, "**token**" that
contains the JWT token you were given. The token contains the _username_, so you
don't have to include that field. It should also contain field "**secret**" that
is used as a secret for, e.g., operating with JWT tokens at your server.

</div>
