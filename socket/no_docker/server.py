import socket
import os

def main():
    test_alone = True 
    if test_alone:
        assert "amialone.txt" not in os.listdir("../")
        os.system("touch ../amialone.txt")
    host = 'localhost'
    port = 12345 
    s = socket.socket()
    s.bind((host, port))

    s.listen(1)
    print("start listening")
    c, addr = s.accept()
    while True:
        data = c.recv(1024)
        print(data.decode())
        if not data:
            break
        data = (data.decode()).upper()
        c.send(data.encode())
    c.close()
    os.system("rm -f ../amialone.txt")
    print("stop listening")

if __name__ == '__main__':
    main()
