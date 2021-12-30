import socket
import os 

def main():
    test_alone = False 
    if test_alone:
        assert "../amialone.txt" not in os.listdir(".")
        os.system("touch ../amialone.txt")
    host = 'localhost' 
    port = 12345 
    s = socket.socket()
    s.connect((host,port))

    print("start sending")
    message = input(">>") 
    while message != 'quit':
        s.send(message.encode())
        data = s.recv(1024)
        message = input(">>")
    print("stop sending")
    s.close()

if __name__ == '__main__':
    main()

