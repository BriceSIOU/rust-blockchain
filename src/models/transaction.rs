use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub sender: String,
    pub recipient: String,
    pub amount: f64, 
    pub timestamp: i64,
}

impl Transaction {
    pub fn new(sender: String, recipient: String, amount: f64) -> Self {
        Transaction {
            id: Uuid::new_v4().to_string(),

            sender, 
            recipient,
            amount,

            timestamp: Utc::now().timestamp(),
        }
    }
}
