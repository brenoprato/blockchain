# Blockchain Implementation in Rust

A minimal blockchain implementation for educational purposes, focusing on core concepts like block structure, hashing, proof-of-work, and persistence.

## Project Structure

- `src/main.rs` - Application entry point and CLI setup
- `src/block.rs` - Block structure and mining logic
- `src/blockchain.rs` - Blockchain management and iterator
- `src/cli.rs` - Command-line interface implementation

## Dependencies

- `sled` - Embedded key-value database for block storage
- `postcard` - Binary serialization for blocks
- `sha2` - SHA-256 hashing for proof-of-work
- `clap` - Command-line argument parsing
- `anyhow` - Error handling

## Building and Running

Compile the project:
```bash
cargo build
```

Run with CLI commands:
```bash
cargo run -- [COMMAND]
```

## CLI Commands

### Clear the blockchain
```bash
cargo run -- clear
```
Resets the entire blockchain, removing all blocks except the genesis block. Deletes the database directory (`data/blocks/`) and creates a new chain.

### Print all blocks
```bash
cargo run -- printchain
```
Displays every block in the blockchain in order, showing:
- Block height (genesis = 0)
- Block hash
- Previous block hash
- Transaction data
- Timestamp
- Nonce value

### Add a new block
```bash
cargo run -- addblock "Your data here"
```
Creates a new block containing the provided string data, mines it using proof-of-work, and adds it to the blockchain. The mining process finds a nonce that results in a hash with the required number of leading zeros (difficulty).

## Implementation Details

- Each block contains a timestamp, transaction data, previous block hash, and nonce
- Blocks are linked via cryptographic hashes (SHA-256)
- Proof-of-work requires finding a nonce that produces a hash with 4 leading hex zeros
- Blocks are serialized using Postcard format and stored in a Sled database
- The blockchain iterator traverses blocks from newest to oldest

## Educational Purpose

This implementation demonstrates:
- Basic blockchain structure and block linking
- Proof-of-work consensus mechanism
- Persistent storage of blockchain data
- Simple CLI for blockchain interaction
- Rust module organization and error handling