import socket
import subprocess
from datetime import datetime


def main():
    server_ip = "10.4.44.3"
    server_port = 8080

    server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server_socket.bind((server_ip, server_port))

    try:
        serverRuntime(server_socket, server_ip, server_port)
    except KeyboardInterrupt:
        print("Keyboard interrupt received. Closing server.")
    finally:
        server_socket.close()


def serverRuntime(server_socket, server_ip, server_port):
    while True:
        server_socket.listen(1)
        print(f"Server listening on {server_ip}:{server_port}")

        client_socket, client_address = server_socket.accept()
        print(f"Connection from {client_address}")

        received_data = client_socket.recv(1024)  # read in data to buffer

        timestamp = datetime.today().strftime('%Y-%m-%d %H:%M:%S')

        subprocess.run(args=["cp", "master_names.txt",
                             f"names_archive/{timestamp}.txt"], check=False, text=True)  # copy the existing file to the archive

        with open("master_names.txt", "wb") as file:  # overwrite the file with the buffer
            while received_data:
                file.write(received_data)
                received_data = client_socket.recv(1024)

        print("File received and stored.")

        client_socket.close()


if __name__ == "__main__":
    main()
