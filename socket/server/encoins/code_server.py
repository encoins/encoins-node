import socket
import os
import yaml

n_node = os.environ['NUM_NODE']

# Read the architecture file

net_config_file = "net_config.yml"
	
with open(net_config_file, "r") as archi_file:

	lines = archi_file.read()
	archi_file.close()

archi = yaml.load(lines, Loader=yaml.FullLoader)

# Listen

s = socket.socket()
s.bind((archi[f'server{n_node}']['ip'], archi[f'server{n_node}']['port']))
s.listen()
new_s, addr = s.accept()

data = new_s.recv(1024)
print(data.decode())

new_s.close()
s.close()
