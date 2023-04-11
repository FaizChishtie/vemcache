# Vemcache

Vemcache is a simple, lightweight, and efficient vector database inspired by Redis. It is designed for managing high-dimensional vector data in various applications, including machine learning, similarity search, and recommendation systems. Vemcache offers a straightforward solution for in-memory storage and retrieval of vector data.

## Features

- **Efficient in-memory storage**: Vemcache stores vector data in memory, resulting in low-latency access and fast operations.
- **Simple and easy-to-use**: The command syntax and API are designed for simplicity and ease of use, making it easy to integrate Vemcache into your projects.
- **Scalable**: Vemcache can handle large amounts of vector data and is built to scale with your application's needs.
- **Extensible**: The modular architecture allows for easy extension and customization of Vemcache's capabilities.

## Getting Started

### Prerequisites

- Rust: To build and run Vemcache, you need Rust and Cargo installed on your system. You can install them using [rustup](https://rustup.rs/).

### Building Vemcache

1. Clone the Vemcache repository:

```bash
git clone https://github.com/yourusername/vemcache.git
cd vemcache
```

2. Build Vemcache:

```bash
make build
```

## Example

```
fn main() {
    let mut db = Vemcache::new();

    let v1 = vec![0.0, 1.0, 2.0];
    let v2 = vec![1.0, 2.0, 3.0];
    let v3 = vec![2.0, 3.0, 4.0];

    let id1 = db.insert(v1.clone());
    let id2 = db.insert(v2.clone());
    let id3 = db.insert(v3.clone());

    let query_vector = vec![0.5, 1.5, 2.5];
    let (nearest_id, nearest_vector) = db.nearest_neighbor(&query_vector).unwrap();

    println!(
        "The nearest vector to {:?} is {:?} with id {}",
        query_vector, nearest_vector, nearest_id
    );
}
```