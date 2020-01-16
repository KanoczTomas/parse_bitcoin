use crate::types::Bytes;

#[derive(Debug, Clone)]
pub struct TxOutput {
    pub value: u64,
    pub script_pub_key: Bytes,
}

impl TxOutput {
    pub fn new(value: u64, spk: &[u8]) -> TxOutput {
        TxOutput {
            value,
            script_pub_key: Bytes::new(spk),
        }
    }
}

impl std::default::Default for TxOutput {
    fn default() -> TxOutput {
        TxOutput {
            value: 0,
            script_pub_key: Bytes::default(),
        }
    }
}

pub struct TxOutputBuilder {
    txo: TxOutput,
}

impl TxOutputBuilder {
    pub fn new() -> Self {
        TxOutputBuilder {
            txo: TxOutput::default(),
        }
    }
    pub fn value(&mut self, value: u64) -> &mut Self {
        self.txo.value = value;
        self
    }
    pub fn script_pub_key<B: Into<Bytes>>(&mut self, bytes: B) -> &mut Self {
        self.txo.script_pub_key = bytes.into();
        self
    }
    pub fn build(&self) -> TxOutput {
        TxOutput {
            value: self.txo.value,
            script_pub_key: self.txo.script_pub_key.clone(),
        }
    }
}
