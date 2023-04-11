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

## Using Vemcache

Connect to Vemcache with a TCP client like `telnet` or `nc`.

Use `telnet` to connect to Vemcache

```bash
telnet 0.0.0.0 7070
```

Or use `nc`

```bash
nc 0.0.0.0 7070
```

Once connected, you can interact with the Vemcache server by sending commands.

### Inserting Vectors
To insert a vector into the database, use the insert command followed by the vector values:

```bash
insert 0.5 0.7 0.2
```
To insert a vector with a specified key, use the named_insert command followed by the key and vector values:

```bash
named_insert my_vector 0.5 0.7 0.2
```

### Retrieving Vectors
To retrieve a vector from the database using its key, use the get command followed by the key:

```bash
get my_vector
```

### Removing Vectors

To remove a vector from the database using its key, use the remove command followed by the key:

```bash
remove my_vector
```

### Performing Vector Operations

To calculate the cosine similarity between two vectors, use the vcosine command followed by the keys of the two vectors:

```bash
vcosine vector1 vector2
```

To find the k nearest neighbors of a vector, use the knn command followed by the key of the query vector and the value of k:

```bash
knn query_vector 3
```

To perform element-wise addition of two vectors, use the vadd command followed by the keys of the two vectors:

```bash
vadd vector1 vector2
```

To perform element-wise subtraction of two vectors, use the vsub command followed by the keys of the two vectors:

```bash
vsub vector1 vector2
```

To scale a vector by a scalar value, use the vscale command followed by the key of the vector and the scalar value:

```bash
vscale vector1 2.0
```

### Closing the Connection

To exit the client, press Ctrl+C or type quit (if using telnet).

This concludes the basic usage of Vemcache for vector operations. For more advanced operations and detailed explanations of each command, refer to the [Vemcache documentation](vemcache.com).

### Example Session

```bash
# User connects to Vemcache using telnet
$ telnet 0.0.0.0 7070
Trying 0.0.0.0...
Connected to 0.0.0.0.
Escape character is '^]'.

# User inserts a vector with the key "vector_a"
named_insert vector_a 0.5 0.7 0.2
OK

# User inserts another vector with the key "vector_b"
named_insert vector_b 0.1 0.9 0.4
OK

# User retrieves the vector with the key "vector_a"
get vector_a
[0.5, 0.7, 0.2]

# User calculates the cosine similarity between "vector_a" and "vector_b"
vcosine vector_a vector_b
Cosine Similarity: 0.8693

# User finds the nearest neighbor of "vector_a" (k=2)
knn vector_a 2
ID: vector_a, Vector: [0.5, 0.7, 0.2]
ID: vector_b, Vector: [0.1, 0.9, 0.4]

# User performs element-wise addition of "vector_a" and "vector_b"
vadd vector_a vector_b
[0.6, 1.6, 0.6]

# User scales "vector_a" by a factor of 2
vscale vector_a 2.0
[1.0, 1.4, 0.4]

# User removes "vector_a" from the database
remove vector_a
OK

# User attempts to retrieve "vector_a" again (expecting an error)
get vector_a
null

# User exits the session
quit
Connection closed by foreign host.
```