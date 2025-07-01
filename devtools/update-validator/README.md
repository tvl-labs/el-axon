# Update Validator Tool

A command-line tool for updating validator lists in the blockchain network through smart contract interactions.

## Overview

The `update-validator` tool allows you to update the validator list on a blockchain network by creating and sending a signed transaction to the metadata contract. It reads validator information from a TOML file, signs the transaction with a private key, and submits it to the network via RPC.

## Building

Navigate to the tool directory and build with Cargo:

```bash
cd devtools/update-validator
cargo build --release
```

## Usage

```bash
./target/release/update-validator -v <validators_file> -p <private_key_file> -u <node_url>
```

### Arguments

| Short | Long                   | Description                                             | Required |
| ----- | ---------------------- | ------------------------------------------------------- | -------- |
| `-v`  | `--validators <PATH>`  | Path to the validators configuration file (TOML format) | Yes      |
| `-p`  | `--private_key <PATH>` | Path to the private key file (32 bytes binary)          | Yes      |
| `-u`  | `--url <URL>`          | RPC URL of the blockchain node                          | Yes      |

### Example

```bash
./target/release/update-validator \
  --validators ./validators.toml \
  --private_key ./private_key.key \
  --url http://localhost:8545
```

## File Formats

### Validators File (TOML)

The validators file should be in TOML format with the following structure:

```toml
[[validators]]
address = "0x..."
pub_key = "0x..."
propose_weight = 1
vote_weight = 1

[[validators]]
address = "0x..."
pub_key = "0x..."
propose_weight = 1
vote_weight = 1
```

Each validator entry should contain:

-   `address`: The validator's address
-   `pub_key`: The validator's public key
-   `propose_weight`: Weight for block proposal voting
-   `vote_weight`: Weight for consensus voting

### Private Key File

The private key file should be a binary file containing exactly 32 bytes representing the private key. This key will be used to sign the update transaction.

## Output

Upon successful execution, the tool outputs the transaction hash:

```
tx hash: 0x1234567890abcdef...
```

You can use this hash to track the transaction status on the blockchain explorer or through RPC calls.
