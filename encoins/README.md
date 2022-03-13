## Usage
 - `cargo run <nb-nodes>` with two environment vars:
    NUM_NODE: id of proc
    OBJ_TRANSACTIONS: nb of transactions to reach
    when the number is reached, a file 'result.txt' is written in debug directory with the number of milliseconds taken.

## Warning
Every node must be ran on different locations (for instance containers). If two nodes are ran in the same directory there will be problems when writing hists/logs/seqs files

## Documentation
To read documentation, type:`cargo doc` and run the file `index.html` in path `target/doc/encoins`

## Docker
Docker image for encoins (about 85Mo). Nothing required but a working docker installation.
- build Docker image : `docker build -t encoins/encoins-node .`
- run docker image : `docker run [- d --restart always|--rm] --name running-encoins-node -e NUM_NODE={i} -v ~/encoins-config:/encoins-config -p 12345:12345 -p 12346:12346 encoins/encoins-node` with {i} the number of the server in the net_config file
