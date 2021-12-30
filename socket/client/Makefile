# First set up the server and then the client

all: build run

build:
	docker build --tag client-docker .

run:
	docker run -it --rm --network=host client-docker
