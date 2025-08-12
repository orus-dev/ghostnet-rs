import socket

HOST = "0.0.0.0"  # Standard loopback interface address (localhost)
PORT = 7525        # Port to listen on (non-privileged ports are > 1023)

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.bind((HOST, PORT))  # Bind the socket to the specified host and port
    s.listen()            # Start listening for incoming connections
    print(f"Server listening on {HOST}:{PORT}")

    conn, addr = s.accept()  # Accept a new connection
    with conn:
        print(f"Connected by {addr}")
        while True:
            data = conn.recv(1024)  # Receive up to 1024 bytes of data
            if not data:
                break  # If no data is received, the client has disconnected
            conn.sendall(data)  # Send the received data back to the client
            print(f"Echoed: {data.decode()}") # Decode and print the echoed data