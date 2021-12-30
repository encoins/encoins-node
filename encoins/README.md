## Usage
 - `cargo run <nb-nodes> <nb-byzantine-nodes>` to run with `nb-nodes` nodes and `nb-byzantine-nodes` byzantine nodes.

## Documentation
To read documentation, type:`cargo doc` and run the file `index.html` in path `target/doc/encoins`

## Docker
Docker image for encoins (about 2Go). Nothing required but a working docker installation.
- build Docker image : `docker build -t encoins-docker`
- run docker image : `docker run -it --rm --name running-encoins-docker encoins-docker`
