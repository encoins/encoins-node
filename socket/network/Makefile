all : run

gen_net_config:
	python gen_net_config.py 4 4

gen_docker-compose : gen_net_config
	python gen_docker-compose.py net_config.yml

build : gen_docker-compose
	docker-compose build

run : build
	docker-compose up

kill :
	docker kill $$(docker ps -a -q)

clean : 
	rm net_config.yml
	rm docker-compose.yml

rmi : kill
	docker system prune -a
