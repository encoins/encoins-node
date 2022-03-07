## Usage
 - `cargo run <nb-nodes> <nb-byzantine-nodes>` to run with `nb-nodes` nodes and `nb-byzantine-nodes` byzantine nodes.

## Documentation
To read documentation, type:`cargo doc` and run the file `index.html` in path `target/doc/encoins`

## Docker
Docker image for encoins (about 85Mo). Nothing required but a working docker installation.
- build Docker image : `docker build -t encoins-docker .`
- run docker image : `docker run --restart always --name running-encoins-docker -e NUM_NODE={i} -v ~/encoins-config:/encoins-config encoins-docker`
