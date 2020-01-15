use crate::types::Hash256;
use crate::types::Bytes;

#[derive(Debug)]
pub struct TxInput {
    pub previous_tx_hash: Hash256,
    pub vout: u32,
    pub script_sig: Bytes,
    pub sequence: u32
}

impl TxInput {
    pub fn new (ptx: &[u8], vout: u32, scr: &[u8], seq: u32) -> TxInput {
        TxInput {
            previous_tx_hash: Hash256::new(ptx),
            vout,
            script_sig: Bytes::new(scr),
            sequence: seq
        }
    }
}

impl std::default::Default for TxInput {
    fn default() -> TxInput {
        TxInput{
            previous_tx_hash: Hash256::default(),
            vout: 0,
            script_sig: Bytes::default(),
            sequence: 0
        }
    }
}

pub struct TxInputBuilder {
    txi: TxInput
}

impl TxInputBuilder {
    pub fn new() -> Self {
        TxInputBuilder{
            txi: TxInput::default()
        }
    }
    pub fn previous_tx_hash<H: Into<Hash256>>(&mut self, hash: H) -> &mut Self {
        self.txi.previous_tx_hash = hash.into();
        self
    }
    pub fn vout(&mut self, vout: u32) -> &mut Self {
        self.txi.vout = vout;
        self
    }
    pub fn script_sig<B: Into<Bytes>>(&mut self, bytes: B) -> &mut Self {
        self.txi.script_sig = bytes.into();
        self
    }
    pub fn sequence(&mut self, sequence: u32) -> &mut Self {
        self.txi.sequence = sequence;
        self
    }
    pub fn build(&self) -> TxInput {
        TxInput{
            previous_tx_hash: self.txi.previous_tx_hash,
            vout: self.txi.vout,
            script_sig: self.txi.script_sig.clone(),
            sequence: self.txi.sequence
        }
    }
}
