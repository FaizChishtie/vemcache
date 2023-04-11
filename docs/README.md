# Vemcache

Vemcache is an in-memory vector database.

## Commands

Vemcache supports a variety of commands to interact with vector data. Below is a list of available commands and their descriptions:

`ping`: Check the connection to the Vemcache server. The server responds with "pong" when it receives this command.

`insert` [values]: Insert a vector into the database. The server generates a unique identifier (UUID) for the vector. Provide space-separated floating-point values as the vector components.

`named_insert` [key] [values]: Insert a vector into the database with a specified key. Provide the key as a string and space-separated floating-point values as the vector components.

`get` [key]: Retrieve a vector from the database using its key.

`remove` [key]: Remove a vector from the database using its key.

`knn` [key] [k]: Find the k nearest neighbors of a vector. Provide the key of the query vector and the value of k (number of neighbors).

`vadd` [key1] [key2]: Perform element-wise addition of two vectors. Provide the keys of the two vectors to be added.

`vsub` [key1] [key2]: Perform element-wise subtraction of two vectors. Provide the keys of the two vectors to be subtracted.

`vscale` [key] [scalar]: Scale a vector by a scalar value. Provide the key of the vector to be scaled and the scalar value.

`vcosine` [key1] [key2]: Calculate the cosine similarity between two vectors. Provide the keys of the two vectors to be compared.

`dump` [filename]: Dump Vemcache DB to a JSON file.

## Usage

To use Vemcache, connect to the server using a TCP client like telnet or nc. Once connected, you can send commands to interact with the server.

Use `telnet` to connect to Vemcache

```bash
telnet 0.0.0.0 7070
```

Or use `nc`

```bash
nc 0.0.0.0 7070
```

## Examples

Here are some example commands to interact with Vemcache:

```bash
# Insert a vector with values 0.0, 1.0, 2.0
insert 0.0 1.0 2.0

# Insert a vector with key "my_vector" and values 0.0, 1.0, 2.0
named_insert my_vector 0.0 1.0 2.0

# Get the vector associated with key "my_vector"
get my_vector

# Remove the vector associated with key "my_vector"
remove my_vector

# Find the 3 nearest neighbors of the vector with key "query_vector"
knn query_vector 3

# Add vectors with keys "vector1" and "vector2"
vadd vector1 vector2

# Subtract vectors with keys "vector1" and "vector2"
vsub vector1 vector2

# Scale vector with key "vector1" by scalar 2.0
vscale vector1 2.0

# Calculate cosine similarity between vectors with keys "vector1" and "vector2"
vcosine vector1 vector2

# Dump vemcache db
dump vemcache.json
```