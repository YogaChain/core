use serde::{Serialize, Deserialize};
use blake3::hash;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub timestamp: u64,
    pub signature: Vec<u8>,
}

impl Transaction {
    /// Creates a new transaction
    pub fn new(sender: &str, receiver: &str, amount: u64, signature: Vec<u8>) -> Self {
        let timestamp = chrono::Utc::now().timestamp() as u64;
        let tx_id = Self::calculate_tx_id(sender, receiver, amount, timestamp);
        Self {
            id: tx_id,
            sender: sender.to_string(),
            receiver: receiver.to_string(),
            amount,
            timestamp,
            signature,
        }
    }

    /// Generates a unique transaction ID using hashing
    fn calculate_tx_id(sender: &str, receiver: &str, amount: u64, timestamp: u64) -> String {
        let tx_data = format!("{}{}{}{}", sender, receiver, amount, timestamp);
        hash(tx_data.as_bytes()).to_hex().to_string()
    }
}
