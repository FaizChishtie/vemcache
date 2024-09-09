import socket
import random

# Configuration
vemcache_host = 'localhost'
vemcache_port = 7070
num_vectors = 1000
vector_dim = 5
num_operations = 100  # Number of random operations to perform

# Generate a list of keys for the vectors in Vemcache
keys = [f'vector{i}' for i in range(num_vectors)]

# Define the operations and their corresponding commands
operations = [
    'get', 'remove', 'knn', 'vadd', 'vsub', 'vscale', 'vcosine'
]

loader = ['|', '/', '-', '\\']

# Perform random operations on Vemcache
def perform_random_operations():
    # Create a TCP/IP socket
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
        # Connect to Vemcache
        sock.connect((vemcache_host, vemcache_port))

        # List to accumulate responses
        responses = []

        # Perform random operations
        for i in range(num_operations):
            # Print a dot on the same line for each iteration
            print(f'Operations completed: {i + 1} {loader[i % len(loader)]}', end='\r', flush=True)

            # Choose a random operation
            operation = random.choice(operations)

            # Generate the command based on the chosen operation
            if operation == 'get':
                key = random.choice(keys)
                command = f'get {key}\n'
            elif operation == 'remove':
                key = random.choice(keys)
                command = f'remove {key}\n'
            elif operation == 'knn':
                key = random.choice(keys)
                k = random.randint(1, 10)
                command = f'knn {key} {k}\n'
            elif operation == 'vadd':
                key1, key2 = random.sample(keys, 2)
                command = f'vadd {key1} {key2}\n'
            elif operation == 'vsub':
                key1, key2 = random.sample(keys, 2)
                command = f'vsub {key1} {key2}\n'
            elif operation == 'vscale':
                key = random.choice(keys)
                scalar = random.uniform(-2, 2)
                command = f'vscale {key} {scalar}\n'
            elif operation == 'vcosine':
                key1, key2 = random.sample(keys, 2)
                command = f'vcosine {key1} {key2}\n'

            # Send the command to Vemcache
            sock.sendall(command.encode())
            response = sock.recv(1024).decode().strip()

            # Accumulate the response in the list
            responses.append(f'Response for {operation} command: {response}\n')

        # Write the accumulated responses to the file in one go
        with open('vemcache_responses.txt', 'w') as file:
            file.writelines(responses)

if __name__ == '__main__':
    perform_random_operations()
