import socket
import random

# Configuration
vemcache_host = 'localhost'
vemcache_port = 7070
num_vectors = 100000
vector_dim = 10

# Generate random vectors and write to Vemcache
def write_vectors_to_vemcache():
    # Create a TCP/IP socket
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
        # Connect to Vemcache
        sock.connect((vemcache_host, vemcache_port))

        # Generate and write vectors
        for i in range(num_vectors):
            # Generate a random vector
            vector = [random.uniform(-1, 1) for _ in range(vector_dim)]
            vector_str = ' '.join(map(str, vector))

            # Create a key for the vector
            key = f'vector{i}'

            # Construct the NamedInsert command
            command = f'named_insert {key} {vector_str}\n'

            # Send the command to Vemcache
            sock.sendall(command.encode())

            # Receive the response from Vemcache
            response = sock.recv(1024).decode().strip()
            print(f'Response for vector {i}: {response}')

if __name__ == '__main__':
    write_vectors_to_vemcache()
