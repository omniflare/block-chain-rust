# 🔗 Rust Blockchain Implementation

A modern, lightweight blockchain implementation built with Rust and Axum web framework. This project demonstrates the core concepts of blockchain technology through a simple HTTP API.

## 🚀 Tech Stack

- **Rust** `1.70+` - Systems programming language 🦀
- **Axum** `0.7` - Lightning-fast web framework for Rust
- **Tokio** - Asynchronous runtime for Rust
- **Serde** - Serialization framework
- **Chrono** - Date and time library
- **SHA-256** - Cryptographic hash function

## 💡 Motivation

This project was created to understand and demonstrate core blockchain concepts in a simple, educational way. Instead of dealing with the complexity of a full cryptocurrency implementation, we focus on the fundamental blockchain data structure and its key properties:

- Immutable chain of blocks
- Cryptographic linking between blocks
- Basic proof-of-work mining
- Transaction handling
- Chain validation

## 🏗️ Architecture

The project consists of three main components:

1. **Blockchain Core** - Handles the blockchain data structure and mining
2. **HTTP API** - Provides endpoints to interact with the blockchain
3. **Persistence Layer** - Saves and loads the blockchain from disk

### Core Data Structures

```rust
// Transaction represents a transfer between parties
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: u64,
}

// Block represents a set of transactions in the chain
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    hash: String,
    transactions: Vec<Transaction>,
    time_stamp: i64,
    prev_hash: String,
    nonce: u64,
}
```

## 🛠️ Installation

1. Clone the repository:
```bash
git clone https://github.com/omniflare/block-chain-rust.git
cd rust-blockchain
```

2. Build the project:
```bash
cargo build --release
```

3. Run the server:
```bash
cargo run
```

The server will start on `http://localhost:8000`

## 📡 API Endpoints

### Get Blockchain
Retrieve the entire blockchain.

```bash
curl http://localhost:8000/chain
```

### Add Block
Add a new block with transactions.

```bash
curl -X POST http://localhost:8000/add_block \
-H "Content-Type: application/json" \
-d '[
  {
    "sender": "Alice",
    "receiver": "Bob",
    "amount": 100
  }
]'
```

## 🔍 Core Features

### Proof of Work Mining
Each block is mined using a simple proof-of-work algorithm:

```rust
fn mine_block(&mut self, difficult: usize) {
    while &self.hash[0..difficult] != "0".repeat(difficult) {
        self.nonce += 1;
        self.hash = self.calculate_hash();
    }
}
```

### Chain Validation
The blockchain maintains integrity through continuous validation:

```rust
fn is_chain_valid(&self) -> bool {
    for (i, block) in self.chain.iter().enumerate() {
        if i == 0 { continue; }
        let prev_block = &self.chain[i - 1];
        if block.hash != block.calculate_hash() { return false; }
        if prev_block.hash != block.prev_hash { return false; }
    }
    return true;
}
```

## 💾 Persistence

The blockchain is automatically saved to and loaded from a file:

```rust
// Save blockchain to file
blockchain.save_to_file("blockchain.txt")?;

// Load blockchain from file
let blockchain = Blockchain::load_from_file("blockchain.txt")?;
```

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

## 🔧 Configuration

The blockchain has configurable parameters:

- **Mining Difficulty**: Set in the `Blockchain::new()` method (default: 2)
- **Server Port**: Configurable in `main.rs` (default: 8000)
- **Storage File**: Configurable in `main.rs` (default: "blockchain.txt")

## 🚧 Limitations

- No distributed consensus mechanism
- Simple proof-of-work implementation
- No transaction validation
- No peer-to-peer networking
- No cryptocurrency functionality

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📜 License

This project is licensed under the MIT License - see the LICENSE file for details.

## ⭐️ Future Improvements

- [ ] Add peer-to-peer networking
- [ ] Implement transaction validation
- [ ] Add merkle tree for transactions
- [ ] Create web interface
- [ ] Add configurable mining difficulty
- [ ] Implement wallet functionality

## 📚 Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Axum Documentation](https://docs.rs/axum/latest/axum/)
- [Blockchain Basics](https://developer.mozilla.org/en-US/docs/Web/API/Blockchain)