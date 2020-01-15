use crate::types::{BlockHeader, Transaction};

#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(h: BlockHeader, t: Vec<Transaction>) -> Block {
        Block {
            header: h,
            transactions: t,
        }
    }
}

impl std::default::Default for Block {
    fn default() -> Block {
        Block {
            header: BlockHeader::default(),
            transactions: Vec::new(),
        }
    }
}

pub struct BlockBuilder {
    blk: Block,
}

impl BlockBuilder {
    pub fn new() -> Self {
        BlockBuilder {
            blk: Block::default(),
        }
    }
    pub fn header(&mut self, header: BlockHeader) -> &mut Self {
        self.blk.header = header;
        self
    }
    pub fn transaction(&mut self, tx: Transaction) -> &mut Self {
        self.blk.transactions.push(tx);
        self
    }
    pub fn transactions(&mut self, txs: &mut Vec<Transaction>) -> &mut Self {
        self.blk.transactions.append(txs);
        self
    }
    pub fn build(self) -> Block {
        Block {
            header: self.blk.header,
            transactions: self.blk.transactions,
        }
    }
}
