Little code that simulates a network of S servers and C clients all in docker.
We can't communicate from the outside with the containers run.

How to:

	cd network
	make

Structure:

	client/encoins:
		code_client.py: 
			connects to the 'server{i}' where i 
			is given in an environment var, send "Hello world"
		Dockerfile: 
			runs code_client.py
	
	server/encoins:
		code_server.py:
			listen on 'server{i}', given in an environment var
			waits to receive a message
		Dockerfile:
			runs code_client.py
	
	network:
		gen_net_config.yml:
			receives two args:
				number of servers S and of clients C, must be the same
			creates a file net_config.yml with the description of the network
			copy it in the client and server directory, so that they have access
			to it
		gen_docker-compose.py
			receives one argument, the name of the network configuration file
			generates a docker-compose.yml file that should set up the network of
			clients and servers
