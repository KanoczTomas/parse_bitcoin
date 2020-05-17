use crate::types::Bytes;
use crate::types::Hash256;
use crate::types::BlockTime;
use chrono::prelude::*;
use std::convert::TryInto;

#[derive(Debug)]
pub struct BlockHeader {
    pub version: u32,
    pub prev_block_hash: Hash256,
    pub merkle_root_hash: Hash256,
    pub time: BlockTime,
    pub bits: Bytes,
    pub nonce: Bytes,
    pub hash: Hash256,
}

impl BlockHeader {
    pub fn new(
        v: u32,
        pbh: &[u8],
        mrh: &[u8],
        t: BlockTime,
        b: &[u8],
        n: &[u8],
        h: Hash256,
    ) -> BlockHeader {
        BlockHeader {
            version: v,
            prev_block_hash: Hash256::new(pbh),
            merkle_root_hash: Hash256::new(mrh),
            time: t,
            bits: Bytes::new(b),
            nonce: Bytes::new(n),
            hash: h,
        }
    }
}

impl std::default::Default for BlockHeader {
    fn default() -> BlockHeader {
        BlockHeader {
            version: 0,
            prev_block_hash: Hash256::default(),
            merkle_root_hash: Hash256::default(),
            time: BlockTime(0),
            bits: Bytes::default(),
            nonce: Bytes::default(),
            hash: Hash256::default(),
        }
    }
}

pub struct BlockHeaderBuilder {
    blkh: BlockHeader,
}

impl BlockHeaderBuilder {
    pub fn new() -> Self {
        BlockHeaderBuilder {
            blkh: BlockHeader::default(),
        }
    }
    pub fn version(&mut self, version: u32) -> &mut Self {
        self.blkh.version = version;
        self
    }
    pub fn prev_block_hash<H: Into<Hash256>>(&mut self, hash: H) -> &mut Self {
        self.blkh.prev_block_hash = hash.into();
        self
    }
    pub fn merkle_root_hash<H: Into<Hash256>>(&mut self, hash: H) -> &mut Self {
        self.blkh.merkle_root_hash = hash.into();
        self
    }
    pub fn time(&mut self, time: BlockTime) -> &mut Self {
        self.blkh.time = time;
        self
    }
    pub fn bits<B: Into<Bytes>>(&mut self, bits: B) -> &mut Self {
        self.blkh.bits = bits.into();
        self
    }
    pub fn nonce<B: Into<Bytes>>(&mut self, nonce: B) -> &mut Self {
        self.blkh.nonce = nonce.into();
        self
    }
    pub fn hash<H: Into<Hash256>>(&mut self, hash: H) -> &mut Self {
        self.blkh.hash = hash.into();
        self
    }
    pub fn build(&self) -> BlockHeader {
        BlockHeader {
            version: self.blkh.version,
            prev_block_hash: self.blkh.prev_block_hash,
            merkle_root_hash: self.blkh.merkle_root_hash,
            time: self.blkh.time,
            bits: self.blkh.bits.clone(),
            nonce: self.blkh.nonce.clone(),
            hash: self.blkh.hash,
        }
    }
}
