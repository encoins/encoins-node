import sys
from yaml import load, dump
from random import randrange

def main():
    nb_args = len(sys.argv)
    if nb_args != 4:
        print("Usage : \npython3 get_net_config.py s b c \nwith s number of server, b number of byzantine, c number of client.\nExit")
        return 1
    
    # init args
    nb_servers = int(sys.argv[1])
    nb_byzantines = int(sys.argv[2])
    nb_clients = int(sys.argv[3])
    
    # init dictionnary
    dic = {
            "parameters": {
                "nb_servers": nb_servers,
                #"nb_byzantines": nb_byzantines,
                "nb_clients": nb_clients}
            }
    
    # fill dictionnary
    for server in range(nb_servers):
        name = "server"+str(server)
        dic[name] = {
                "address": name,
                "port_server": 12345,
                "port_client": 12346}
    
    #for byzantine in range(nb_servers):
    #    name = "byzantine"+str(byzantine)
    #    dic[name] = {
    #            "address": name,
    #            "port_server": 12345
    #            "port_client": 12346
    
    for client in range(nb_clients):
        name = "client"+str(client)
        dic[name] = {
                "address_server": "server"+str(randrange(nb_servers)),
                "port_server": 12346} 

    print(dump(dic, sort_keys=False))
    return 0

if __name__ == '__main__':
    sys.exit(main())
