use crate::types::Hash256;
use crate::types::Bytes;

#[derive(Debug)]
pub struct TxInput {
    pub previos_tx_hash: Hash256,
    pub vout: u32,
    pub script_sig: Bytes,
    pub sequence: u32
}

impl TxInput {
    pub fn new (ptx: &[u8], vout: u32, scr: &[u8], seq: u32) -> TxInput {
        TxInput {
            previos_tx_hash: Hash256::new(ptx),
            vout,
            script_sig: Bytes::new(scr),
            sequence: seq
        }
    }
}
