---
title: Security
lang: en
translation_key: security
---

The original design of the Internet protocols was based on insecure
communication where the messages could be read by anyone having access to the
communication path. Without additional security protocols, the Internet traffic
on top of the IP protocol is unencrypted and could be read with tools like
Wireshark. Anyone along the connection path could therefore read and modify the
messages, for example from a local WiFi access router.

Also the domain name system that is used to map domain names into IP addresses
is based on unencrypted communication, in addition being cached and replicated,
which allows injecting wrong IP address information. IP address in packet
headers can also be modified by an intermediate node on a communication path.
Therefore, in a basic insecure communication session we cannot be sure if the
server actually is who we believe it to be, and a solution that allows
authenticating the server will be useful.

Since the original deployment of the Internet, different solutions for security
have been developed. One solution is **IPsec [RFC
4301](https://datatracker.ietf.org/doc/html/rfc4301)**, that adds integrity
protection and encryption directly on network layer to IP packets. IPsec is
often used in Virtual Private Networks for organization networks. IPsec
typically requires operating system support, and requires therefore little
effort to set up.

Another, currently the most popular solution that we are going to
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

- You will be **able to use JSON Web Tokens** for authorizing messages.

</div>

## Transport Layer Security

**Transport Layer Security (TLS)** aims to provide three types of security:

- **Confidentiality**: The traffic is encrypted so that outsiders cannot read it.
- **Integrity**: Detecting if data has been modified by an outsider
- **Authentication**: The identity of communication end point is verified (using
  a trusted third party). Usually this is done with servers, but could also be
  done with clients.

TLS could be seen as a separate layer on top of TCP and below the applications,
so it fits well in the traditional layered protocol model. The below
illustration is taken from an online book "[High Performance Browser
Networking](https://hpbn.co/transport-layer-security-tls/)".

![TLS in the protocol stack](https://hpbn.co/assets/diagrams/9873c7441be06e0b53a006aac442696c.svg){: width="120%" .center-img }

### Certificates

An essential part of the TLS protocol is the **certificate** that establishes
authenticity of the server. Certificate is created and signed by a trusted
**certificate authority (CA)**, and the server sends it to the client when a TLS
connection is established. Certificate includes (among some other things) the
following parts:

- **common name** that is typically compared to the DNS name of the connected
  server.
- **public key** of the server that is used to authenticate the server.
- **validity period** during which the certificate can be used.
- **issuer** name of the certificate authority that has created the certificate.
- **digital signature** that the CA has created, and that can be verified using
  the public key of the CA.

The client verifies the certificate that it receives from server during the TLS
handshake, for example that the common name matches the connected server, and
that the certificate is still valid. The issuer should also be trusted by the
client. There are generally trusted **root certificate authorities** configured
to the local system. Root CAs may delegate CA responsibility to intermediate CAs
that then create and provide the actual certificate to the server operator,
forming a certificate chain that starts from the server certificate and and
should terminate at one of the trusted root CAs.

For some internal uses, such as corporate intranets, or in our course server
containers, getting an actual certificate from a trusted public CA may not be
practical. It is typically possible, however, to configure custom local root CAs
in such environments for easier certificate deployment, and this is what we also
do on our course (more about the practicalities a bit later).

### TLS handshake protocol

When a TLS communication is desired, the client opens the TCP connection
normally, but after the connection establishment, it starts a TLS handshake that
takes the following steps:

- The client sends **ClientHello** message, where it indicates, for example,
  which TLS versions and cipher algorithms it supports.
- The server responds with a **ServerHello** message where is chooses one of the
  suggested cipher algorithms, provides the certificate and information needed
  for choosing a symmetric key used for the actual data communication.
- The client verifies certificate, and both sides compute a shared secret that
  becomes the symmetric session key used for actual communication.
- Both sides confirm the exchange with "**Finished**" message, and the
  communication continues using the session key.

Using symmetric session key has some benefits: it is more efficient to use, and
using a unique key for each session improves the security.

The above-described handshake is illustrated in the below graph, originally from
Cloudflare's blog article "[What happens in a TLS
handshake](https://www.cloudflare.com/learning/ssl/what-happens-in-a-tls-handshake/)".
From the picture, one can see that even though TLS is commonplace these days for
securing the network communication, its downside is that it increases the
connection establishment delay before actual data can be transmitted. This can
be hinder the communication performance if the actual connection payload is
short.

![TLS protocol handshake](https://images.ctfassets.net/slt3lc6tev37/5aYOr5erfyNBq20X5djTco/3c859532c91f25d961b2884bf521c1eb/tls-ssl-handshake.png){: width="90%" .center-img }

## TLS in Rust

A popular library (or crate) for using TLS in Rust applications is
**[rustls](https://crates.io/crates/rustls)**. It provides the API for using TLS
on top of TCP.

### Client operation

We have a modified version of our earlier simple client in examples directory,
under
**[tls-client](https://github.com/PasiSa/pronets/blob/main/examples/tls-client/src/main.rs)**.
Again, you can clone the repository and try the example also on your own
machine.

Like previous examples, we have a few command line arguments, and now an
additional optional command line argument for specifying a root certificate
file, if we want to use such. If you test the program using one of the public
servers (there is an example of `www.aalto.fi` case), this is not needed because
the `webpki-roots` crate contains the commonly accepted root certificates.
However, if we want test this locally, we need to specify our own root
certificate that we can use to validate the incoming server certificate. The
example directory contains file `cert/ca.crt` that is used to sign the
certificate provided in **tls-server** example, and that can be used when
testing against the server example.

As a first step, starting on [line
34](https://github.com/PasiSa/pronets/blob/dbf3770c7a4858e826b7782a2c7e496c57863c88/examples/tls-client/src/main.rs#L34),
we separate the server name from command line argument, because it is compared
to the common name in incoming certificate, which must match. _rusttls_ has a
dedicated `ServerName` type for holding this.

Starting on [line
44](https://github.com/PasiSa/pronets/blob/dbf3770c7a4858e826b7782a2c7e496c57863c88/examples/tls-client/src/main.rs#L44),
we take different branch depending on whether we use a user-provided root
certificate, or whether we use on of the public certificates. The set of
available root certificates is stored in `roots` variable, of type
`RootCertStore`.

Then we build TLS client configuration from this information ([line
62](https://github.com/PasiSa/pronets/blob/dbf3770c7a4858e826b7782a2c7e496c57863c88/examples/tls-client/src/main.rs#L62)),
and connect the TCP client socket normally to the server. After this, we create
a new `ClientConnection` object with the TLS configuration, which also starts
the TLS handshake. If the handshake is successful and the server certificate is
valid, we connect the TLS ClientConnection to the TCP socket as **rustls**
`StreamOwned` type, as variable `socket`. After this we can use the read and
write operations on the socket normally, except that now all communication
happens as TLS encrypted.

Finally, the client signals that it is done sending data and about to close the
connection using the `send_close_notify` function, on [line
78](https://github.com/PasiSa/pronets/blob/dbf3770c7a4858e826b7782a2c7e496c57863c88/examples/tls-client/src/main.rs#L78).
This tells the other end that closing is planned, and not cut short for other
reason.

### Server operation

A TLS server example can be found under
**[tls-server](https://github.com/PasiSa/pronets/blob/main/examples/tls-server/src/main.rs)**
directory of the course git repository. It similar to earlier [asynchronous
server](https://github.com/PasiSa/pronets/blob/main/examples/async-server/src/main.rs)
example with Tokio, but now uses TLS. For using TLS with Tokio, there is a
separate **[tokio-rustls](https://crates.io/crates/tokio-rustls)** crate that we
are using.

In addition to the listening IP and port, the server now takes also the
PEM-encoded files representing **certificate** and **private key** that
corresponds the public key in certificate as arguments. The _rustls_ crate reads
these files and sets the server configuration based on these ([line
46](https://github.com/PasiSa/pronets/blob/dbf3770c7a4858e826b7782a2c7e496c57863c88/examples/tls-server/src/main.rs#L46)).
To test the example, we can use the server certificate (_cert/server.crt_ and
_cert/server.key_) and private key included in the repository. Note that **in
reality, you would never add these files to git repository, and especially the
private key should be kept secret!** (and not readable by other uses on the
system). The certificate was created using the _ca.crt_ included in the
_tls-client_ directory, that the client can use to verify it.

Similarly to **ClientConnection** on the client side, **TlsAcceptor** uses the
TLS configuration to turn TCP connection into a TLS stream ([line
51](https://github.com/PasiSa/pronets/blob/dbf3770c7a4858e826b7782a2c7e496c57863c88/examples/tls-server/src/main.rs#L51)).
It is wrapped inside atomic reference counter (**Arc**), because each
Tokio task needs to have access it.

The TCP server socket is bound normally, and new connection is first accepted
normally with **TcpSocket**'s `accept()` function, but after that accepted
separately with **TcpAcceptor**'s `accept()` ([line
69](https://github.com/PasiSa/pronets/blob/dbf3770c7a4858e826b7782a2c7e496c57863c88/examples/tls-server/src/main.rs#L69))
that finishes the TLS negotiation. After this the read and write can be used on
the socket as usual.

## Working with certificates

**OpenSSL** is a common tool used for various TLS-related tasks, for example to
review and create certificates (SSL, or Secure Sockets Layer, is the predecessor
to TLS, that is the current standard). It is usually readily installed in many
systems, but if not, for example in Ubuntu Linux you could install it normally
using _apt_: `sudo apt install openssl`

A certificate can be investigated in following way on a command line
(for certificate in file _server.crt_):

    openssl x509 -in server.crt -text -noout

This certificate is not very good: for example, it has very long lifetime. But
it is only used on the course, for practising operations with certificates.

_openssl_ can also be used to verify and show information about certificate at
given network address. For example, we can see the _www.aalto.fi_ certificate:

    openssl s_client -connect www.aalto.fi:443

### Creating root certificate

To create a certificate for a local **root certificate authority**, the root
CA operator (which in case of local testing of your implementation will be you)
will first need to create a **private key**:

    openssl genpkey -algorithm RSA -out rootCA.key -pkeyopt rsa_keygen_bits:4096

The private key should be kept secret in safe place. Anyone having access to
this key can create certificates as the root CA.

The next steps is to create a OpenSSL configuration file that specifies
information in the CA certificate:

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

After this, the actual self-signed root CA certificate to file _rootCA.crt_ can
be created based on the configuration. In this case the private key is in file
_rootCA.key_ and the configuration like above in _root.cnf_.

    openssl req \
        -new \
        -x509 \
        -days 3650 \
        -key rootCA.key \
        -out rootCA.crt \
        -config root.cnf

### Creating server certificate

After these steps, the root CA operator can create the actual server
certificate. Creating the server certificate follows similar steps, although in
this case it is useful to distinguish the tasks between the root CA and the
server administrator. The **server administrator** creates a private key:

    openssl genpkey \
        -algorithm RSA \
        -out server.key \
        -pkeyopt rsa_keygen_bits:2048

The permissions for the private key file, _server.key_, should be set such that
only the server administrator can read it.

The _server administrator_ then creates the certificate configuration file, for
example:

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

The server's DNS name and IP address needs to match the common name (`CN`) or
one of the alternative names fields (`DNS.X` or `IP.X`). As one can note, the
same certificate can support multiple names and addresses, for example for
different network interfaces or addressing scopes. If you are testing locally,
you could just use `localhost`.

Then the _server administrator_ creates a **certificate signing request** using
the configuration file (_server.cnf_ in this case) and private key. In this
case, this creates a file _server.csr_:

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
this case), valid for 365 days. The CA signs the certificate information
provided by the _server administrator_, and passes it to the _server
administrator_.

    openssl x509 \
        -req \
        -in server.csr \
        -CA rootCA.crt \
        -CAkey rootCA.key \
        -CAcreateserial \
        -out server.crt \
        -days 365 \
        -sha256 \
        -extfile server.cnf \
        -extensions server_extensions

Of course, when creating certificates for our own use, we do all above steps by
ourselves.

## Authorization and JSON Web Tokens

There are different techniques for **authenticating** and **authorizing** users
and clients. For example, TLS certificates can also authenticate clients,
similarly to how they authenticate servers above. When both parties authenticate
using certificates. This is called **mutual TLS (mTLS)**.

In the following we will take a look into how to authorize client requests using
**JSON Web Tokens (JWT)**, which are commonly used in network APIs. Typically,
HTTP clients send the JWT in "`Authorization`" header with value `Bearer
JWT-TOKEN`. Starting from this module, all requests to start containers need to
must include a valid JWT, and use a TLS-protected connection.

### Authentication vs authorization

**Authentication** responds to the question _"Who are you?"_. The system should be
able to determine the identity of the user (or client). This could happen
through username / password login, using certificates, or some external identity
service (e.g. based on bank credentials).

When user identity is known, the it needs to be decided _what permissions does
the user have and what resources they can access_. This is called
**authorization**. In distributed service, one way to deliver this information
the different service components is to use JSON Web Token that is signed by a
entity deciding on authorization. JWT therefore is not an authentication
protocol, but a format to deliver authorization information in a way that the
intended receiver can parse in stateless and reliable way.

Our course server does not implement authentication, but from this module
onwards it requires a JWT token as an attribute to requests starting a Docker
container. The token will be delivered to each project separately via MyCourses
by the course staff, and it is signed by the course staff. We trust that
authentication has happened through normal Aalto login practices in MyCourses.

### JWT structure

**JSON Web Token** ("JWT", specified in [RFC
7519](https://datatracker.ietf.org/doc/html/rfc7519)) is a JSON - formatted data
structure that is encoded using _Base64URL_ encoding (specified in [RFC
4648](https://datatracker.ietf.org/doc/html/rfc4648)), that converts the JSON
data into a compact string stream composed from 64-bit character set that can be
handled by text based systems and protocols (such as old versions of HTTP). JWTs
can be either **JSON Web Signature (JWS)** or **JSON Web Encryption (JWE)**
type. In the following we will discuss JWS, that is described in
more detail in [RFC 7515](https://datatracker.ietf.org/doc/html/rfc7515).

One nice property of the JWT is that it does not necessarily require state at
the server: all needed information for authorization is contained within the
token itself. The JSON data is composed from three parts:

- **Header** (also known as JSON Object Signing and Encryption (JOSE) Header)
  contains information about the algorithm used for signing the data.
- **Payload** contains the attributes (or "claims") that the token asserts.
- **Signature** contains the signature made by the issuer of the token, using
  the algorithm indicated in the header.

The token header is often like this:

```json
{
  "typ": "JWT",
  "alg": "HS256"
}
```

This indicates that what follows is a JSON Web Token, and we are using **HS256**
algorithm based on shared secret to sign the token. The payload has claims such
as:

```json
{
  "sub": "alice",
  "role": "student",
  "exp": 1791910800
}
```

The Base64URL encoded the tokens typically start something like: `eyJ0...`,
because the header part is often the same and starts with curly braces. It is
good to understand that even though the token seems unreadable at first sight,
it is not encrypted, and can be parsed from the Base64URL encoding back to the
JSON format. This is a fairly straightforward programming task following [RFC
4648](https://datatracker.ietf.org/doc/html/rfc4648), and there are web-based
tools for [encoding](https://www.base64encode.org/) and
[decoding](https://www.base64decode.org/) tokens. However, the signature at the
end of the token aims to ensure that the content of the claims cannot be
modified by an outsider, without being caught at the token verification.

### Claims

There are two kinds of JSON claims: the registered claim names are defined in
the JWT specification ([RFC 7519, section
4](https://datatracker.ietf.org/doc/html/rfc7519#section-4)), and are common
attributes that can be used across different services and applications. In
addition to registered claims, there can be application-specific claims that
application developer can freely choose based on the needs of the application.

There are altogether seven registered claim names as detailed in the
specification, here are a few common ones:

- **`iss` (issuer)** describes who issued (or created) this JWT. The format of
  issuer name is not specified and is application specific. It could be based on
  domain name of the identity service that authenticated an user, for example.
- **`sub` (subject)** describes the target or the subject of the JWT. For
  example, it could be the username or an authenticated user (or a container
  name of an authorized Docker container, as with our course server).
- **`exp` (expiration time)** indicates the time after which the JWT is not
  valid, and should not be accepted anymore. The expiration time is indicated as
  number of seconds from the start of 1970, a common format in standards and
  computer systems to indicate times using integers. Usually it is a good
  practice to specify lifetime for tokens, and then provide a way to update a
  token in an authenticated way before the expiration time.
- **`iat` (issued at)** tells when the token was created. It follows a similar
  time format as the expiration time (seconds since 1970).

### Signing and verification

There are two kinds of models for signing the JWTs. In the **symmetric** variant
he token issuer and server that validates the token can use a **shared secret**.
Commonly, **HS256** is the algorithm used in this case. Shared secret is
recommended to be at least 32-byte random value that is distributed between
issuer and server instances using the token. OpenSSL can be used to generate
random values, for example with following, for a 32-byte value:

    openssl rand -hex 32

Alternatively, an **asymmetric** variant is to use **private key and public
key**. In this case only the token issuer holds the private key, and all servers
that use the token to validate requests use public key. Commonly, **RS256**
algorithm is used in this case.

Benefit of the symmetric variant is that it is simple, but sharing the secret
means that anyone possessing the secret can create tokens, so all parties need
to trust each other. Therefore the asymmetric variant is often preferred, if
there are multiple servers that need to verify the tokens.

At the server, after the token signature is verified, the server will
investigate the claims to determine what the client is allowed to do. For
example, it checks that the issuer `iss` is as expected, and that the token is
still valid based on the expiry time (`exp`). There may also be other
application-specific claims that further clarify what the token is authorized
for, for example different user roles (e.g., student, teacher, admin).

### Rust implementation

The [`jsonwebtoken` crate](https://docs.rs/crate/jsonwebtoken/10.3.0) can
serialize claims into Base64URL encoded format, and sign them to create a JWT.
Below is an example of `Cargo.toml` additions to take it into use. **serde** is
the crate for serializing structure data into e.g. JSON format, and vice versa.

```toml
[dependencies]
jsonwebtoken = { version = "10", features = ["rust_crypto"] }
serde = { version = "1", features = ["derive"] }
```

The following example creates a token signed with HS256, valid for one hour.
Before running the program, the `JWT_SECRET` environment variable needs to be
set to contain a random secret of at least 32 bytes. The secret is then shared
with the server that verifies the token. This could happen, for example in the
following way on a command line, if we are using cargo to run the program:

    JWT_SECRET="myrandomsecret" cargo run

The secret could also be included in a file that is protected from outside
users. In this case the example would need to be slightly modified where secret
is set, though.

```rust
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

// Generate Serde's Serialize implementation so Claims can be encoded as JSON.
#[derive(Serialize)]
struct Claims {
    iss: String,
    sub: String,
    iat: u64,
    exp: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // In this case the shared secret is stored in environment variable JWT_SECRET.
    let secret = std::env::var("JWT_SECRET")?;
    if secret.len() < 32 {
        return Err("JWT_SECRET must contain at least 32 bytes".into());
    }

    // Read the current time for 'iat' claim, and set lifetime to one hour
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let claims = Claims {
        iss: "pns-course-staff".to_owned(),
        sub: "project-123".to_owned(),
        iat: now,
        exp: now + 60 * 60,
    };

    // The `encode` function from jsonwebtoken creates the actual
    // Base64URL - encoded token and signs it using the secret.
    let token = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    println!("{token}");
    Ok(())
}
```

Here `sub` identifies the project and `iss` identifies the token issuer. The
server uses the same secret to verify the signature before trusting the claims.

The following server-side example reads a received token from a command-line
argument. It verifies the HS256 signature, expiration time, and expected issuer
before using the project identifier. It uses the same dependencies and
`JWT_SECRET` as the token issuer above. Note that the environment variable is
always specific to local session, so it needs to be separately set at server.

```rust
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::Deserialize;

// `Debug` enables debug formatting (printed using {:?} in formatted output).
// `Clone` enables explicit copying of the struct.
// `Deserialize` lets Serde construct Claims from the token's JSON payload.
#[derive(Debug, Clone, Deserialize)]
struct Claims {
    iss: String,
    sub: String,
    iat: u64,
    exp: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // secret is given in environment variable as in previous example.
    let secret = std::env::var("JWT_SECRET")?;
    if secret.len() < 32 {
        return Err("JWT_SECRET must contain at least 32 bytes".into());
    }
    // JWT is given as command line argument.
    let token = std::env::args().nth(1).ok_or("missing JWT argument")?;

    // Use the `Validation` type and its methods from jsonwebtoken crate
    // to validate the token. Accept only HS256 tokens from the expected issuer.
    // Validation checks 'exp' by default; require the other claims as well.
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_issuer(&["pns-course-staff"]);
    validation.set_required_spec_claims(&["exp", "iss", "sub", "iat"]);
    validation.leeway = 0;

    let verified = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;

    println!("Authenticated project: {}", verified.claims.sub);
    Ok(())
}
```

The `leeway` validation parameter allows a tolerance in seconds when checking
the expiration time, to account for clock differences between the issuer and
server. For example, a leeway of 60 allows a token to pass the expiration check
for up to 60 seconds after its `exp` time. The example sets `leeway` to zero so
that expired tokens are rejected without this tolerance.

The network server should pass the token received from the client to `decode`
function and use the verified claims to check whether the project is authorized
for the requested operation. If verification fails, `decode` returns an error
and the server must reject the request.

<div class="assignment-frame" markdown="1">

## Assignment #7

You have received a JWT token you should use from this point on to authenticate
your requests to start and restart your container (detailed instructions as
MyCourses announcement).

**Part 1:** Decode the token you received, and report what claims it contains.
When does the token expire? Is it good idea to have a token with long
lifetime (justify any benefits and drawbacks)? Is the token valid or not?

**Part 2 (a)**: Check the certificate of our course server
[pronets1.dice.aalto.fi](https://pronets1.dice.aalto.fi/). Who has issued the
certificate? When does the certificate expire?

**Part 2 (b)**: Check the same information about the internal root certificate
we have used to created container certificates. Who has issues the certificate,
and when does it expire?

**Part 3**: Implement TLS support to your server and client implementation. When
testing your implementation locally, you can create yourself local root
certificate and server certificate and private key following the instructions in
this module.

When you deploy your implementation to the course server, your certificate is
mounted in file `/run/tls/server.crt` inside the container, and the private key
is in file `/run/tls/server.key`. Note that these files are different for each
project container, despite the same name (Docker magic!). Before deploying your
implementation to server, read part 4 instructions below first.

To support local testing and server deployment with different certificate by the
same implementation, it is a good idea to use either command line attributes or
environment variables to configure the files for certificate and private key.

Shortly describe in your report the steps you did to test your implementation
locally, and the approach you took to configure the certificate and key in your
implementation.

**Part 4**: Modify the program you have used to send the `run-docker` HTTP
request in the following ways:

- Connect the course server, `pronets1.dice.aalto.fi` using TLS, to TCP port 443
  (i.e., "HTTPS"). Note that the course server certificate is created by
  generally trusted public CA and therefore you should not need to add any
  custom root CAs in your implementation for verifying the server.

- We will now use a new POST endpoint, `run-docker-secure` that only works with
  TLS. There should be new header field in the HTTP request that follows the
  common form of passing the JWT tokens in HTTP requests:

  `Authorization: Bearer YOUR-JWT-HERE`

  If this header field is missing, or is wrong (e.g., because token is not
  valid), the request will not be accepted. The JSON attributes are otherwise
  same than with the `run-docker` request, but you do not include the `name`
  field anymore for container name (why?). However, you should add a new
  attribute

  `"secure": true`

  to indicate that your server now requires TLS (there is a small icon in course
  server user interface to tell this). In case you need it, there is also an
  optional `env` attribute that can be included in JSON payload, to set
  environment variables to your container, for example:

  `"env": ["RUST_LOG=debug", "SECRET_VALUE=xyz"]`

Tell your report what JSON parameters you used in the POST request, and whether
you got your secure implementation successfully running. Connect your server
container using your client running in your local machine and test the messages
implemented so far (TST, USR, MSG, MET).

**Part 5 (optional):** If you have time and interest to do something extra,
implement a new message `AUT` ("authorized user") that takes a JWT as a
parameter. This would be used as a more secure alternative to the earlier `USR`
message, to register users to your service. You can decide what claims to use in
the token (probably at least `sub` for the username), based or your project
needs, and how to distribute the tokens (you might, for example, have a simple
login service that creates the tokens with valid username and password). This
part is not required for full points from this assignment. **Don't include the
secret used to create and validate in your git repository!** This is a good use
case for the `env` attribute described above.

If you implemented part 5, tell in the report what claims you use in your
implementation.

As before, include also the following information:

- How much time did you use for this assignment?
- What was easy or difficult in the assignment?
- What tools or other information sources did you use? In particular, if you
  used AI assistants, tell how did you use them and if they were helpful.

</div>
