# Influence Maximization using ACO 
 * Víctor Emiliano Cruz Hernández

### Requirements
* Have <a href="https://doc.rust-lang.org/cargo/getting-started/installation.html" target="_blank"> Cargo</a> installed :crab:

### Instalation

1. Clone repository:
```
$ git clone https://github.com/Veppam2/influence-maximization-aco
```
2. Unzip epinions.zip files in directory ./influence-maximization-aco

### Use
```
$ cargo run --release graph NUMBER_OF_NODES SEED OUT_DIR
            We create a subgraph from epinions.
                -> NUMBER_OF_NODES: The desire number of nodes in the graph
                -> SEED: A number so we can replicate the created graph
                -> OUT_DIR: Dir with name of the output file.


$ cargo run --release OPTION FILE_NAME K SEED
    -> FILE_NAME : Name of the file generated previously.
    -> K: number of desire influencers
    -> OPTION:
            => Use 'eval' to evaluate a seed in SEED, SEED is a number.
            => Use 'expr' to experiment with diferent seeds, in total SEED number of seeds.
```
### Example

* You can build a graph with 500 nodes on seed 10 on output file "graph500S10.csv" by:
```
$ cargo run --release graph 500 10 ./graph500S10.csv
```   

* You can search for k=20 nodes with seed 40 in the graph in the file "graph500S10.csv" by:
```
$ cargo run --release eval ./graph500S10.csv  20 40
```
* You can experiment and find k=20 nodes with 5000 diferent seeds on the graph in file "graph500S10.csv" by:
```
$ cargo run --release expr ./graph500S10.csv 20 5000
```
