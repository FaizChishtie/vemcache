# Vemcache

Vemcache is an in-memory vector database. 

Vemcache can be thought of as the Redis equivalent for vector databases.

## Getting Started

### Prerequisites

- Docker: To run Vemcache using Docker, you need to have Docker installed on your system. You can install Docker by following the instructions on the [Docker website](https://docs.docker.com/get-docker/).
- Rust: To build and run Vemcache, you need Rust and Cargo installed on your system. You can install them using [rustup](https://rustup.rs/).

### Building Vemcache

#### Using Docker

1. Pull the [Vemcache image from dockerhub](https://hub.docker.com/r/faizchishtie/vemcache)

```bash
docker pull faizchishtie/vemcache
```

2. Run the image

```bash
docker run --rm -it -p 7070:7070 faizchishtie/vemcache:latest
```

#### Using docker-compose

Add the following to your `docker-compose.yml`

```
version: "3.7"

services:
  vemcache:
    image: faizchishtie/vemcache:latest
    ports:
      - "7070:7070"
    environment:
      - VEMCACHE_HOST=0.0.0.0
      - VEMCACHE_PORT=7070
      - VEMCACHE_SECRET=mysecret
```

#### Locally

1. Clone the Vemcache repository:

```bash
git clone https://github.com/yourusername/vemcache.git
cd vemcache
```

2. Build Vemcache:

```bash
make build
```

3. Run Vemcache

```bash
make run
```

### Using Vemcache

Connect to Vemcache with a TCP client like `telnet` or `nc`.

Use `telnet` to connect to Vemcache

```bash
telnet 0.0.0.0 7070
```

Or use `nc`

```bash
nc 0.0.0.0 7070
```