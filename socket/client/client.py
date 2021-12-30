import socket

def Main():
    host = 'localhost' #The host on your client needs to be the external-facing IP address of your router. Obtain it from here https://www.whatismyip.com/
    port = 42424 
    s = socket.socket()
    s.connect((host,port))
    message = raw_input("->") 
    print("before while")
    while message != 'q':
        s.send(message)
        data = s.recv(1024)
        message = raw_input("->")
    print("after while")
    s.close()

if __name__ == '__main__':
    Main()

