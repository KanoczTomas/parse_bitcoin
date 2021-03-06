use crate::{
    types::{BlockHeader, BlockHeaderBuilder, BlockVersion, BlockTime},
    utils::hash256
};
use nom::{
    bytes::complete::take,
    number::complete::le_u32,
    sequence::tuple,
    IResult,
};

pub fn parse_block_header(input: &[u8]) -> IResult<&[u8], BlockHeader> {
    let block_header_start = input;
    let (i, (version, prev_block_hash, merkle_root_hash, time, bits, nonce)) = tuple((
        le_u32,            //version
        take(32 as usize), //prev_block_hash
        take(32 as usize), //merkle_root_hash
        le_u32,            //time
        take(4 as usize),  //bits
        take(4 as usize),  //nonce
    ))(input)?;
    let block_header_raw = &block_header_start[0..block_header_start.len() - i.len()];
    Ok((
        i,
        BlockHeaderBuilder::new()
            .version(BlockVersion(version))
            .prev_block_hash(prev_block_hash)
            .merkle_root_hash(merkle_root_hash)
            .time(BlockTime(time))
            .bits(bits)
            .nonce(nonce)
            .hash(hash256(block_header_raw))
            .build(),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::{Bytes, Hash256};
    use hex;
    #[test]
    fn test_parse_block_header() {
        //generated with from helper_scripts directory:
        // for i in $(ls ../blk*.rpc);do ./generate_block_header_tests.sh $i;done
        let data = include_bytes!("../test_data/blk_0000000000000000000215160a3490f82c7203d9683802148a56282d1f80993d.bin");
        let (_, header) = parse_block_header(data).unwrap();
        assert_eq!(header.version.0, 536870912);
        assert_eq!(header.prev_block_hash, Hash256::new(&hex::decode("21e4e008ffdaa5382ffe57d4419641dc89c7f610906104000000000000000000").unwrap()));
        assert_eq!(header.merkle_root_hash, Hash256::new(&hex::decode("2e13f67bd6f0944b17f385008364b9120cfc362c546a384db7066b02eb938e88").unwrap()));
        assert_eq!(header.time.0, 1576880064);
        assert_eq!(header.bits, Bytes::new(&hex::decode("d0bc1517").unwrap()));
        assert_eq!(header.nonce, Bytes::new(&hex::decode("6129429F").unwrap()));
        assert_eq!(header.hash, Hash256::new(&hex::decode("3d99801f2d28568a14023868d903722cf890340a161502000000000000000000").unwrap()));

        let data = include_bytes!("../test_data/blk_0000000000000000000b0a682f47f187a712c42badd4ca1989c494d401457c3f.bin");
        let (_, header) = parse_block_header(data).unwrap();
        assert_eq!(header.version.0, 1073725440);
        assert_eq!(header.prev_block_hash, Hash256::new(&hex::decode("66c810e611643b26ddc0bf0a4d9fc21f409d5ad9a6ac09000000000000000000").unwrap()));
        assert_eq!(header.merkle_root_hash, Hash256::new(&hex::decode("96cf49dcc64a3f405ca144c9e61752896f5bcde78fe0089b61952d48ee0826b7").unwrap()));
        assert_eq!(header.time.0, 1576151029);
        assert_eq!(header.bits, Bytes::new(&hex::decode("d2db1517").unwrap()));
        assert_eq!(header.nonce, Bytes::new(&hex::decode("9AB9308D").unwrap()));
        assert_eq!(header.hash, Hash256::new(&hex::decode("3f7c4501d494c48919cad4ad2bc412a787f1472f680a0b000000000000000000").unwrap()));

        let data = include_bytes!("../test_data/blk_000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f.bin");
        let (_, header) = parse_block_header(data).unwrap();
        assert_eq!(header.version.0, 1);
        assert_eq!(header.prev_block_hash, Hash256::new(&hex::decode("0000000000000000000000000000000000000000000000000000000000000000").unwrap()));
        assert_eq!(header.merkle_root_hash, Hash256::new(&hex::decode("3ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a").unwrap()));
        assert_eq!(header.time.0, 1231006505);
        assert_eq!(header.bits, Bytes::new(&hex::decode("ffff001d").unwrap()));
        assert_eq!(header.nonce, Bytes::new(&hex::decode("1DAC2B7C").unwrap()));
        assert_eq!(header.hash, Hash256::new(&hex::decode("6fe28c0ab6f1b372c1a6a246ae63f74f931e8365e15a089c68d6190000000000").unwrap()));
    }
}
