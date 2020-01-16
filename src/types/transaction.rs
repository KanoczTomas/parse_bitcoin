use crate::types::{Hash256, TxInput, TxOutput, Witness};

#[derive(Debug)]
pub struct Transaction {
    pub version: u32,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub witnesses: Option<Vec<Vec<Witness>>>,
    pub lock_time: u32,
    pub txid: Hash256,
    pub wtxid: Hash256,
    pub size: usize,
}

impl Transaction {
    pub fn new(
        version: u32,
        inputs: Vec<TxInput>,
        outputs: Vec<TxOutput>,
        witnesses: Option<Vec<Vec<Witness>>>,
        lock_time: u32,
        txid: Hash256,
        wtxid: Hash256,
        size: usize,
    ) -> Transaction {
        Transaction {
            version,
            inputs,
            outputs,
            witnesses,
            lock_time,
            txid,
            wtxid,
            size,
        }
    }
}

impl std::default::Default for Transaction {
    fn default() -> Transaction {
        Transaction {
            version: 0,
            inputs: Vec::new(),
            outputs: Vec::new(),
            witnesses: None,
            lock_time: 0,
            txid: Hash256::default(),
            wtxid: Hash256::default(),
            size: 0,
        }
    }
}

pub struct TransactionBuilder {
    tx: Transaction,
}

impl TransactionBuilder {
    pub fn new() -> Self {
        TransactionBuilder {
            tx: Transaction::default(),
        }
    }
    pub fn version(&mut self, version: u32) -> &mut Self {
        self.tx.version = version;
        self
    }
    pub fn input(&mut self, input: TxInput) -> &mut Self {
        self.tx.inputs.push(input);
        self
    }
    pub fn inputs(&mut self, inputs: &mut Vec<TxInput>) -> &mut Self {
        self.tx.inputs.append(inputs);
        self
    }
    pub fn output(&mut self, output: TxOutput) -> &mut Self {
        self.tx.outputs.push(output);
        self
    }
    pub fn outputs(&mut self, outputs: &mut Vec<TxOutput>) -> &mut Self {
        self.tx.outputs.append(outputs);
        self
    }
    pub fn witnesses(&mut self, witnesses: Option<Vec<Vec<Witness>>>) -> &mut Self {
        self.tx.witnesses = witnesses;
        self
    }
    pub fn lock_time(&mut self, lock_time: u32) -> &mut Self {
        self.tx.lock_time = lock_time;
        self
    }
    pub fn txid<H: Into<Hash256>>(&mut self, hash: H) -> &mut Self {
        self.tx.txid = hash.into();
        self
    }
    pub fn wtxid<H: Into<Hash256>>(&mut self, hash: H) -> &mut Self {
        self.tx.wtxid = hash.into();
        self
    }
    pub fn size(&mut self, size: usize) -> &mut Self {
        self.tx.size = size;
        self
    }
    pub fn build(&self) -> Transaction {
        Transaction {
            version: self.tx.version,
            inputs: self.tx.inputs.clone(),
            outputs: self.tx.outputs.clone(),
            witnesses: self.tx.witnesses.clone(),
            lock_time: self.tx.lock_time,
            txid: self.tx.txid,
            wtxid: self.tx.wtxid,
            size: self.tx.size,
        }
    }
}
