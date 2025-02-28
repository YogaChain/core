use crate::transaction::Transaction;
use std::collections::VecDeque;

pub struct Mempool {
    pub transactions: VecDeque<Transaction>,
}

impl Mempool {
    /// Creates a new mempool instance
    pub fn new() -> Self {
        Self {
            transactions: VecDeque::new(),
        }
    }

    /// Adds a new transaction to the mempool
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push_back(transaction);
    }

    /// Fetches transactions for block inclusion
    pub fn fetch_transactions(&mut self, limit: usize) -> Vec<Transaction> {
        self.transactions.drain(0..limit.min(self.transactions.len())).collect()
    }
}
