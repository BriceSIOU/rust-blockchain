use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use crate::models::Transaction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    // 0 = genesis block
    pub index: u64,
    pub timestamp: i64,
    pub transactions:Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Self{
        let mut block = Block {
            index,
            timestamp: Utc::now().timestamp(),
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0
        };

        block.hash = block.calculate_hash();
        block 
    }
    pub fn calculate_hash(&self) -> String {
        let transaction_json = serde_json::to_string(&self.transactions)
            .unwrap_or_default();
        let content = format!(
            "{}{}{}{}{}",
            self.index,
            self.timestamp,
            transaction_json,
            self.previous_hash,
            self.nonce 

        );
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let result = hasher.finalize();

        hex::encode(result)
    } 

    pub fn has_valid_hash(&self, difficulty: usize) -> bool {
        let prefix = "0".repeat(difficulty);
        self.hash.starts_with(&prefix)
    }
}
