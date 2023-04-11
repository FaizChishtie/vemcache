# Use the official Rust image as the base image
FROM rust:latest as builder

# Set the working directory
WORKDIR /usr/src/vemcache

# Copy the project's source code and configuration files
COPY . .

# Build the release version of Vemcache
RUN cargo build --release

# Use a smaller base image for the final image
FROM debian:buster-slim

# Install any runtime dependencies (if necessary)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the Vemcache binary from the builder image
COPY --from=builder /usr/src/vemcache/target/release/vemcache /usr/local/bin/vemcache

# Expose the port that Vemcache will use (change this if you're using a different port)
EXPOSE 7070

# Start the Vemcache server when the container is run
CMD ["vemcache"]