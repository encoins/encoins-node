# First set up the server and then the client

all: build run

build:
	docker build --tag server-docker .

run:
	docker run -it --rm --network=host --name=localhost server-docker
