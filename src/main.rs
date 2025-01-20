use std::{fs, io::Error, sync::Arc};

use axum::routing::{get, post};
use axum::Router;
use chrono::Utc;
use handlers::{add_block, get_chain};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::net::SocketAddr;
use tokio::sync::Mutex;
mod handlers;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    hash: String,
    transactions: Vec<Transaction>,
    time_stamp: i64,
    prev_hash: String,
    nonce: u64,
}

impl Block {
    fn new(transations: Vec<Transaction>, previous: String) -> Self {
        let time_stamp = Utc::now().timestamp();
        let mut block = Block {
            time_stamp: time_stamp,
            transactions: transations,
            prev_hash: previous,
            nonce: 0,
            hash: "".to_string(),
        };

        block.hash = block.calculate_hash();

        return block;
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{:?}{:?}{}{}",
            self.time_stamp, self.transactions, self.prev_hash, self.nonce
        ));

        return format!("{:x}", hasher.finalize());
    }

    fn mine_block(&mut self, difficult: usize) {
        while &self.hash[0..difficult] != "0".repeat(difficult) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
}

impl Blockchain {
    fn new() -> Self {
        let start_block = Block::new(vec![], String::from("0"));
        return Blockchain {
            chain: vec![start_block],
            difficulty: 2,
        };
    }

    fn add_block(&mut self, transactions: Vec<Transaction>) {
        let prev_hash = self.chain.last().unwrap().hash.clone();
        let mut new_block = Block::new(transactions, prev_hash);
        new_block.mine_block(self.difficulty);
        self.chain.push(new_block);
    }

    fn is_chain_valid(&self) -> bool {
        for (i, block) in self.chain.iter().enumerate() {
            if (i == 0) {
                continue;
            }

            let prev_block = &self.chain[i - 1];
            if block.hash != block.calculate_hash() {
                return false;
            }
            if prev_block.hash != block.prev_hash {
                return false;
            }
        }
        return true;
    }

    fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        match fs::write(filename, data) {
            Ok(()) => Ok(()),
            Err(_) => return Err(Error::last_os_error()),
        }
    }

    fn load_from_file(filename: &str) -> std::io::Result<(Self)> {
        let data = match fs::read_to_string(filename) {
            Ok(dat) => dat,
            Err(e) => e.to_string(),
        };
        let blockchain: Blockchain = serde_json::from_str(&data)?;
        return Ok(blockchain);
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let filename = "blockchain.txt";
    let blockchain = if let Ok(blockchain) = Blockchain::load_from_file(filename) {
        blockchain
    } else {
        Blockchain::new()
    };
    let app_state = Arc::new(Mutex::new(blockchain));
    let appstate_clone = Arc::clone(&app_state);
    println!("Starting blockchain application on port 8000");

    let app: Router = Router::new()
        .route("/chain", get(get_chain))
        .route("/add_block", post(add_block))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Listening on {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();

    let blockchain = appstate_clone.lock().await;
    if blockchain.is_chain_valid() {
        println!("Blockchain is valid");
    } else {
        println!("Blockchain is invalid");
    }
    blockchain.save_to_file(filename)?;

    Ok(())
}
