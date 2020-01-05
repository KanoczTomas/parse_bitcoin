use crate::types::Bytes;
use crate::types::Hash256;
use chrono::prelude::*;
use std::convert::TryInto;

#[derive(Debug)]
pub struct BlockHeader {
    pub version: u32,
    pub prev_block_hash: Hash256,
    pub merkle_root_hash: Hash256,
    pub time_str: String,
    pub time: u32,
    pub bits: Bytes,
    pub nonce: Bytes,
    pub hash: Hash256
}

 impl BlockHeader {
    pub fn new(v: u32, pbh: &[u8], mrh: &[u8], t: u32, b: &[u8], n:&[u8], h: Hash256 ) -> BlockHeader{
        BlockHeader{
            version: v,
            prev_block_hash: Hash256::new(pbh),
            merkle_root_hash: Hash256::new(mrh),
            time: t,
            time_str: chrono::Utc.timestamp(t.try_into().unwrap(), 0u32).to_rfc2822(),
            bits: Bytes::new(b),
            nonce: Bytes::new(n),
            hash: h
        }
    }
}
