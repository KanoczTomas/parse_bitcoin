use crate::types::Bytes;

#[derive(Debug)]
pub struct TxOutput {
    pub value: u64,
    pub script_pub_key: Bytes
}

impl TxOutput {
    pub fn new(value: u64, spk: &[u8]) -> TxOutput {
        TxOutput {
            value,
            script_pub_key: Bytes::new(spk)
        }
    }
}
