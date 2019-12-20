use nom::IResult;
use crate::types::{Block};
use crate::parsers::{parse_block_header, parse_var_int, parse_transaction};

pub fn parse_block(input: &[u8]) -> IResult<& [u8], Block> {
    // let (input, _) = many_till(take(1usize), peek(parse_magic_number))(input).unwrap();
    // let (input, chain) = parse_magic_number(input)?;
    // let (input, size) = le_u32(input)?;
    let chain="mainnet";
    let size=1172657;
    let (input, header) = parse_block_header(input)?;
    println!("header: {:?}", header);
    let (mut input, tx_count) = parse_var_int(input)?;
    println!("tx_count: {:?}", tx_count);
    let mut txs = Vec::with_capacity(tx_count as usize);
    for n in 0..tx_count {
        let (i, tx) = parse_transaction(input)?;
        println!("\rparsed transaction index: {}",n);
        println!("tx: {:?}", tx);
        txs.push(tx);
        input = i;
    }
    Ok((input, Block::new(header, chain, size, txs)))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_block (){
        let data = include_bytes!("../test_data/blk.0000000000000000000b0a682f47f187a712c42badd4ca1989c494d401457c3f.bin");
        let (_, block) = parse_block(data).unwrap();
        assert_eq!(block.transactions.len(),2996);
        // let res = parse_block_header(data);
        // #[derive(Debug)]
        // enum Test<T, K,L> {
        //     Result(T),
        //     Error(K),
        //     Incomplete(L)
        // }
        // let ize = match res {
        //     Ok((_,o)) => format!("Output: {:?}",o),
        //     Err(e) => {
        //         match e {
        //             nom::Err::Error((_,e)) => format!("Error {:?}",e),
        //             nom::Err::Failure(e) => format!("Failuer {:?}", e),
        //             nom::Err::Incomplete(n) => format!("Needed {:?}",e)
        //         }
        //     }
        // };
        // println!("ize: {}",ize);
        // assert_eq!(1,0);

        // println!("{:?}", block.header);
        // assert_eq!(block.size, 1);
    }
}
