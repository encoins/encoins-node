import socket
import os

def main():
    test_alone = True 
    if test_alone:
        assert "amialone.txt" not in os.listdir("../")
        os.system("touch ../amialone.txt")
    host = '0.0.0.0'
    port = 12345 
    s = socket.socket()
    s.bind((host, port))

    s.listen(1)
    print("start listening")
    c, addr = s.accept()
    while True:
        data = c.recv(1024)
        print(str(data))
        if not data:
            break
        data = str(data).upper()
        c.send(data.encode())
    c.close()
    print("stop listening")

if __name__ == '__main__':
    main()
