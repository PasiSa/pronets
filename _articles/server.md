---
title: Simple server and Docker containers
---

## Server programming

_TBD_

### Passive and active sockets

_TBD: How to set up a listening socket, how it spawns into active socket_

### Simple iterative server

_TBD: idea and example, Advanced Networking has something that can be used_

### I/O multiplexing and non-blocking sockets

_TBD: idea and example, Advanced Networking has something that can be used_

## Docker basics

Docker is a way to package an application with its runtime environment so that
it behaves consistently across different machines. On this course we will use
Docker to deploy the server implementations between your local machines to the
course servers hosted by the university. Docker allows you to test the
application first locally and then move it to a server where everyone can access
it.

First, one produces a Docker image that contains a packages filesystem and
needed metadata. `Dockerfile` specifies the recipe for building an image. Docker
images are typically layered: new files and data are built on top of an base
image (for example one that contains the basic Linux tools), so that a single
image does not have to contain a full disk image, like a virtual machine would
have.

Docker container is a running instance of a image. On large services, Docker is
used to dynamically replicate and scale services into multiple, typically
distributed containers.

Often Docker images are deployed in registry (such as `docker.io`), where they
can be found and used by anyone needing them. On this course we are not using a
registry, but the course server rebuilds the image based on student git
repositories.

On most systems, easiest way to get Docker in a local system is to install
[Docker Desktop](https://docs.docker.com/desktop/), that comes also with the
command line tools mentioned below.

### Building a Docker image

Below is an example of a `Dockerfile` that has the **builder part** for building a
binary server application from a Rust source code, and **runtime part** that
runs the binary server application, opening TCP port 1234 for connections from
the outside world.

```dockerfile
# Build the Rust server application.
FROM rust:1.96 AS builder

# Set the working directory inside the container.
WORKDIR /usr/src/my-server-app

# Copy the source code from local machine to the container.
COPY ./ ./

# Build the server in release mode for better performance (inside the container).
# In this case, the server is located in a separate 'server' package under
# the Rust project.
RUN cargo build -p server --release


# Run the compiled server from a smaller runtime image.
FROM debian:bookworm-slim AS runtime

WORKDIR /usr/local/bin

# Copy the compiled server binary from the builder stage to the runtime stage.
COPY --from=builder /usr/src/pronets-demo-chat/target/release/server ./server

# Expose the port that the server will listen on.
EXPOSE 1234

# Set the command to run the server when the container starts.
CMD ["./server", "--port", "1234"]
```

The builder part of the Dockerfile can be run, for example, with the following
command on the command line.

```bash
docker build -t my-server-app .
```

The `-t my-server` option gives the built image a local name. The final `.`
tells Docker to use the current directory as the build context.

After this the container can be executes as follows:

```bash
docker run -d -p 1234:1234 --name my-server-app my-server-app:latest
```

The `-d` option causes the server to be run in the background. To keep it on the
terminal foreground (e.g. for development and debugging), you can leave the `-d`
option out. The `-p 1234:1234` causes TCP port 1234 from the container to be
exposed at the host machine, so that it can be connected from outside
applications.

### Using Docker

_TBD: docker ps, docker logs, stopping container, running commands inside
container_

## Assignment

_TBD: clarify_

Implement a simple server that listens to incoming connections at the port
assigned for you.

The server should be registered to course server by using a HTTP POST request.
The exact HTTP endpoint is `POST /run-docker`. The body of the POST request
should have the following structure: `username git_repo_URL ports`. Implement a
program that generates this request. This program is run outside the course
server, e.g. from your own machine.

The server should respond to `TST <string>` message by echoing the string back
to the client.
