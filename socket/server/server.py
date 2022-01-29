import socket
import os

def main():
    print("start server")
    host = socket.gethostbyname('server') 
    port = 12345 
    s = socket.socket()
    s.bind((host, port))

    s.listen(1)
    print("start listening")
    c, addr = s.accept()
    data = c.recv(1024)
    print("received : "+data.decode())
    c.close()
    print("stop listening")

if __name__ == '__main__':
    main()
