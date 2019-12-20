use std::str::FromStr;
use crate::types::{BlockHeader, Transaction};

#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub chain: String,
    pub size: u32,
    pub transactions: Vec<Transaction>
}

impl Block {
    pub fn new(h: BlockHeader, c: &str, s: u32, t: Vec<Transaction>) -> Block {
        Block {
            header: h, chain: String::from_str(c).unwrap(), size: s, transactions: t
        }
    }
}
