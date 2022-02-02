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

# Connect to the server

s = socket.socket()
s.connect((archi[f'server{n_node}']['ip'], archi[f'server{n_node}']['port']))

s.send('Hello_world'.encode())

s.close()
