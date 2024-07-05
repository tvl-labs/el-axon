# Axon EigenLayer Operator

## Run the EL-Axon Network

### Build and Run EL Testnet
```shell
sh run.sh start
```

### Build Axon and Operator
```shell
cargo build --release && cd avs-optional && cargo build --release && cd..
```

### Run Axon Network
```shell
./target/release/axon init --config devtools/chain/config.toml --chain-spec devtools/chain/specs/single_node/chain-spec.toml
./target/release/axon run --config devtools/chain/config.toml
```

### Run Axon Operator
```shell
./target/release/avs-optional
```
