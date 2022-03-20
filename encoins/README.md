## Usage
 - `cargo run <nb-nodes> <nb-byzantine-nodes>` to run with `nb-nodes` nodes and `nb-byzantine-nodes` byzantine nodes.

## Documentation
To read documentation, type:`cargo doc` and run the file `index.html` in path `target/doc/encoins`

## Docker
Docker image for encoins (about 85Mo). Nothing required but a working docker installation.
- build Docker image : `docker build -t encoins/encoins-node .`
- run docker image : `docker run [- d --restart always|--rm] --name running-encoins-node -e NUM_NODE={i} -v ~/encoins-config:/encoins-config -v ~/files:/files -p 12345:12345 -p 12346:12346 encoins/encoins-node` with {i} the number of the server in the net_config file
- sevrer `docker rm -f running-encoins-node; docker pull encoins/encoins-node:latest; docker run --rm -d --name running-encoins-node -v ~/files/:/usr/local/bin/files/ -v ~/encoins-config/:/encoins-config/ -v ~/files/:/files/ -p 12345:12345 -p 12346:12346 -e NUM_NODE=5 encoins/encoins-node`