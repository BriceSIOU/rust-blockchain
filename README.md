# rust-blockchain

A blockchain implementation written from scratch in Rust, for educational purposes.
Built without frameworks — every component is implemented manually to understand
the underlying mechanics of blockchain technology.

## Features

- [x] Block structure with SHA-256 hashing
- [x] Genesis block
- [x] Transaction model
- [x] Mempool (pending transactions pool)
- [x] Proof of Work mining
- [x] Chain validation (tamper detection)
- [ ] REST API
- [ ] Peer-to-peer networking

## Project Structure

src/
├── main.rs # Entry point
└── models/
├── mod.rs # Module declarations
├── transaction.rs # Transaction data structure
├── block.rs # Block structure and SHA-256 hashing
└── blockchain.rs # Blockchain, mempool, PoW mining and validation


## Getting Started

### Prerequisites

- Rust (stable) — install via [rustup.rs](https://rustup.rs)

### Run

```bash
git clone https://github.com/BriceSIOU/rust-blockchain.git
cd rust-blockchain
cargo run
```

### Test the tamper detection

The `main.rs` demonstrates chain validation by:
1. Creating a blockchain with difficulty 2
2. Adding transactions and mining blocks
3. Attempting to falsify a transaction in a mined block
4. Showing that `is_valid()` detects the tampering

## How It Works

### Block Structure

Each block contains:
- `index` — position in the chain
- `timestamp` — Unix timestamp of creation
- `transactions` — list of transactions included in the block
- `previous_hash` — cryptographic link to the previous block
- `hash` — SHA-256 hash of all block fields
- `nonce` — number iterated during Proof of Work

### Proof of Work

Mining consists of incrementing the `nonce` until the block's SHA-256 hash
starts with a required number of zeros (defined by `difficulty`).

difficulty = 2 → hash must start with "00"
difficulty = 4 → hash must start with "0000"


Each additional zero multiplies the average mining time by ~16.

### Chain Validation

`is_valid()` traverses the entire chain and verifies:
1. Each block's stored hash matches its recalculated hash
2. Each block's `previous_hash` matches the actual hash of the preceding block

Any modification to a past block is immediately detectable.

## Tech Stack

- **Rust** 2024 edition
- **sha2** — SHA-256 hashing
- **serde / serde_json** — JSON serialization
- **chrono** — timestamps
- **uuid** — unique transaction IDs
- **hex** — hexadecimal encoding

## Roadmap

- [ ] REST API (actix-web)
- [ ] Peer-to-peer networking (libp2p)
- [ ] Merkle Tree for transaction root
- [ ] Digital signatures (ECDSA)
- [ ] Dynamic difficulty adjustment
- [ ] Wallet implementation

## Author

Brice SIOU — [@BriceSIOU](https://github.com/BriceSIOU)
