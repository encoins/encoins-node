import socket
import os 

def main():
    print("start client")
    #return None
    host = socket.gethostbyname('server') 
    port = 12345 
    s = socket.socket()
    s.connect((host,port))

    print("start sending")
    message = "ENcoinS is the best consensus-free cryptocurrency" 
    s.send(message.encode())
    print("stop sending")
    s.close()

if __name__ == '__main__':
    main()

