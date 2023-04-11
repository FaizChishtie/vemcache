# Vemcache

Vemcache is an in-memory vector database.

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

3. Run Vemcache

```bash
make run
```

4. Use Vemcache

Use `telnet` to connect to Vemcache

```bash
telnet 0.0.0.0 7070
```

Or use `nc`

```bash
nc 0.0.0.0 7070
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