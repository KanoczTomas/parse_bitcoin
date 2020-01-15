#[derive(Clone, PartialEq)]
pub struct Bytes(pub Vec<u8>);

impl Bytes {
    pub fn new(slice: &[u8]) -> Bytes {
        Bytes(Vec::from(slice))
    }
    pub fn len(&self) -> usize {
        let Bytes(bytes) = self;
        bytes.len()
    }
    pub fn take(&self, n: usize) -> &[u8] {
        let Bytes(bytes) = self;
        &bytes[0..n]
    }
}

impl std::fmt::Debug for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Bytes(bytes) = self;
        // for byte in bytes.iter().rev() {
        for byte in bytes.iter() {
            write!(f, "{:02X}", byte)?
        }
        write!(f, "")
    }
}

impl std::convert::From<&[u8]> for Bytes {
    fn from(slice: &[u8]) -> Bytes {
        Bytes(Vec::from(slice))
    }
}

impl std::default::Default for Bytes {
    fn default() -> Bytes {
        Bytes::new(&[][..])
    }
}
