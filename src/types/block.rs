use crate::types::{BlockHeader, Transaction};

#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>
}

impl Block {
    pub fn new(h: BlockHeader, t: Vec<Transaction>) -> Block {
        Block {
            header: h, transactions: t
        }
    }
}
