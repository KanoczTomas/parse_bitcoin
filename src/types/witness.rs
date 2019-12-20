use crate::types::Bytes;

#[derive(Debug,PartialEq)]
pub struct Witness(pub Option<Bytes>);

impl Witness {
    pub fn new(slice: &[u8]) -> Witness {
        let witness = match slice.len() {
            0 => None,
            _ => Some(Bytes::new(slice))
        };
        Witness(witness)
    }
    pub fn empty() -> Witness {
        Witness(None)
    }
}
