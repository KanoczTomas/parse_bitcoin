use crate::types::{TxInput, TxOutput, Witness};

#[derive(Debug)]
pub struct Transaction {
    pub version: u32,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub witnesses: Option<Vec<Vec<Witness>>>,
    pub lock_time: u32
}

impl Transaction {
    pub fn new (version: u32, inputs: Vec<TxInput>,
            outputs: Vec<TxOutput>, witnesses: Option<Vec<Vec<Witness>>>,
            lock_time: u32) -> Transaction {
        Transaction {
            version,
            inputs,
            outputs,
            witnesses,
            lock_time
        }
    }
}
