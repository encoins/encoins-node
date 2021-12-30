import socket

def Main():
    host = 'localhost'
    port = 42424
    s = socket.socket()
    s.bind((host, port))

    s.listen(1)
    print("before accept")
    c, addr = s.accept()
    print("before while")
    while True:
        data = c.recv(1024)
        if not data:
            break
        data = str(data).upper()
        c.send(data)
    c.close()
    print("after while")

if __name__ == '__main__':
    Main()
