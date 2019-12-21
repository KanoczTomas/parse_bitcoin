use nom::IResult;
use crate::types::Block;
use crate::parsers::{parse_block_header, parse_var_int, parse_transaction};

pub fn parse_block(input: &[u8]) -> IResult<& [u8], Block> {
    let (input, header) = parse_block_header(input)?;
    let (mut input, tx_count) = parse_var_int(input)?;
    let mut txs = Vec::with_capacity(tx_count as usize);
    for _ in 0..tx_count {
        let (i, tx) = parse_transaction(input)?;
        txs.push(tx);
        input = i;
    }
    Ok((input, Block::new(header, txs)))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::Hash256;
    #[test]
    fn test_parse_block (){
        //generated with: for i in $(ls ../blk*.rpc);do ./generate_block_tests.sh $i;done
        //testing if txid matches for 1st and last tx in a block

        let data = include_bytes!("../test_data/blk_0000000000000000000215160a3490f82c7203d9683802148a56282d1f80993d.bin");
        let (_, block) = parse_block(data).unwrap();
        assert_eq!(block.transactions.len(),447);
        assert_eq!(block.transactions[0].txid, Hash256::new(&hex::decode("89eea9100fe0a42ee210766d1c1c4ce703c648ca3c88ce2cc4830b5b30f0723c").unwrap()));
        assert_eq!(block.transactions[446].txid, Hash256::new(&hex::decode("fe87e797e7a29ebba726a9287128eacf4b2c73051070b8e02631d01e19d45968").unwrap()));

        let data = include_bytes!("../test_data/blk_0000000000000000000b0a682f47f187a712c42badd4ca1989c494d401457c3f.bin");
        let (_, block) = parse_block(data).unwrap();
        assert_eq!(block.transactions.len(),2996);
        assert_eq!(block.transactions[0].txid, Hash256::new(&hex::decode("bf803bfcee4e86c850e0c2077f9777949b7e6d9eae87d1cb7390acead8c9def1").unwrap()));
        assert_eq!(block.transactions[2995].txid, Hash256::new(&hex::decode("fe6a1370c0ae6ecaa0a95184604ae25a91eb12fbe090fcee399dcbd016d0e414").unwrap()));

        let data = include_bytes!("../test_data/blk_000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f.bin");
        let (_, block) = parse_block(data).unwrap();
        assert_eq!(block.transactions.len(),1);
        assert_eq!(block.transactions[0].txid, Hash256::new(&hex::decode("3ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a").unwrap()));

        // read the whole tx data and make a hash256 of it, and check if it is the same as in the rpc, for consinstency

        //iterating blocks means to give the parser a chunk of the slice as big as the size, not bigger
        //no need to probably, as if the parsing fails the input is not returned, so we can start find_block_start to find a new block
    }
}
