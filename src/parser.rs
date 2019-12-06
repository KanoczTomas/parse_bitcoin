use nom::IResult;
use nom::bytes::complete::take;
use nom::number::complete::{le_u32,le_u16, le_u64, le_u8};
use nom::sequence::tuple;
use chrono::prelude::*;
use std::convert::TryInto;
use std::io::Write;
use hex;

// pub fn length_value(input: &[u8]) -> IResult<&[u8],&[u8]> {
//     let (input, length) = be_u16(input)?;
//     take(length)(input)
// }

pub fn magic_number(input: &[u8]) -> IResult<&[u8], &str> {
    let (i, o) = le_u32(input)?;
    let result = match o {
        0xD9B4BEF9 => "mainnet",
        0xDAB5BFFA => "regtest",
        0x0709110B => "testnet",
        0xFEB4BEF9 => "namecoin",
        _ => "Unknown"
    };
    Ok((i, result))
}

//without header
pub fn block_size (input: &[u8]) -> IResult<&[u8], u32> {
    let (i, size) = le_u32(input)?;
    Ok((i, size))

}

//warning LE on wire, keeping format!
// #[derive(Debug)]
pub struct Hash256([u8;32]);

impl Hash256 {
    fn new(slice: &[u8]) -> Hash256 {
        let mut arr = [0;32];
        arr.copy_from_slice(slice);
        Hash256(arr)
    }
}

//we print the hash in BE, as that is how bitcoind and block explorers show it
impl std::fmt::Debug for Hash256 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let Hash256(hash) = self;
        for byte in hash.iter().rev() {
            write!(f, "{:X}", byte)?
        }
        write!(f, "")
    }
}

#[derive(Debug)]
pub struct BlockHeader {
    version: u32,
    prev_block_hash: Hash256,
    merkle_root_hash: Hash256,
    time: String,
    bits: String,
    nonce: String
}

 impl BlockHeader {
    pub fn new(v: u32, pbh: &[u8], mrh: &[u8], t: u32, b: &[u8], n:&[u8] ) -> BlockHeader{
        BlockHeader{
            version: v,
            // prev_block_hash: hex::encode_upper(pbh),
            prev_block_hash: Hash256::new(pbh),
            // merkle_root_hash: hex::encode_upper(mrh),
            merkle_root_hash: Hash256::new(mrh),
            time: chrono::Utc.timestamp(t.try_into().unwrap(), 0u32).to_rfc2822(),
            bits: hex::encode_upper(b),
            nonce: hex::encode_upper(n)
        }
    }
}

pub fn block_header(input: &[u8]) -> IResult<&[u8], BlockHeader> {
    let (i, (
        version,
        prev_block_hash,
        merkle_root_hash,
        time,
        bits,
        nonce
    )) = tuple((
        le_u32, //version
        take(32 as usize), //prev_block_hash
        take(32 as usize), //merkle_root_hash
        le_u32, //time
        take(4 as usize), //bits
        take(4 as usize) //nonce
    )
    )(input)?;
    Ok((i, BlockHeader::new(
        version,
        prev_block_hash,
        merkle_root_hash,
        time,
        bits,
        nonce
    )))
}

pub fn var_int(input: &[u8]) -> IResult<&[u8], u64> {
    let (i, o) = le_u8(input)?;
    if o == 0xFD {
        let (i, o) = le_u16(i)?;
        return Ok((i, o.into()))
    }else if o == 0xFE {
        let (i, o) = le_u32(i)?;
        return Ok((i, o.into()))
    }else if o == 0xFF {
        let (i, o) = le_u64(i)?;
        return Ok((i, o.into()))
    }
    Ok((i,o.into()))
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_var_int() {
        assert_eq!(var_int(&[0xFA][..]), Ok((&[][..],0xFAu64)));
        assert_eq!(var_int(&[0xFA,0xAA][..]), Ok((&[0xAA][..],0xFAu64)));
        assert_eq!(var_int(&[0xFD,0xAA,0xBB][..]), Ok((&[][..],0xBBAAu64)));
        assert_eq!(var_int(&[0xFD,0xAA,0xBB, 0xCC][..]), Ok((&[0xCC][..],0xBBAAu64)));
        assert_eq!(var_int(&[0xFE,0xAA,0xBB, 0xCC, 0xDD][..]), Ok((&[][..],0xDDCCBBAAu64)));
        assert_eq!(var_int(&[0xFE,0xAA,0xBB, 0xCC, 0xDD, 0xEE][..]), Ok((&[0xEE][..],0xDDCCBBAAu64)));
        assert_eq!(var_int(&[0xFF,0xAA,0xBB, 0xCC, 0xDD,0xEE, 0xFF,0x10, 0x09][..]), Ok((&[][..],0x0910FFEEDDCCBBAAu64)));
        assert_eq!(var_int(&[0xFF,0xAA,0xBB, 0xCC, 0xDD,0xEE, 0xFF,0x10, 0x09,0x08][..]), Ok((&[0x08][..],0x0910FFEEDDCCBBAAu64)));
    }
}
