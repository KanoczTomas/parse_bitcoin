use crate::parsers::parse_magic_number;
use nom::combinator::peek;
use nom::number::complete::le_u8;
use nom::IResult;

pub fn find_block_start(mut input: &[u8]) -> IResult<&[u8], Option<&str>> {
    //move per byte untill magic number is found
    loop {
        match peek(parse_magic_number)(input)?.1 {
            Some(_) => {
                break;
            }
            None => {
                input = le_u8(input)?.0;
            }
        };
    }
    let (input, chain) = parse_magic_number(input)?;
    Ok((input, chain))
}

#[cfg(test)]
mod test {
    use super::*;
    use hex;
    #[test]
    fn test_find_block_start() {
        let data = &hex::decode("D9B4BEF9F9BEB4D9aabbccddeeff").unwrap();
        let (rest, chain) = find_block_start(data).unwrap();
        println!("rest: {}", hex::encode(rest));
        assert_eq!(rest, &[0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff][..]);
        assert_eq!(chain, Some("mainnet"));

        let data = &hex::decode("01010100101010F9BEB4FEaabbccddeeff").unwrap();
        let (rest, chain) = find_block_start(data).unwrap();
        assert_eq!(rest, &[0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff][..]);
        assert_eq!(chain, Some("namecoin"));

        let data = &hex::decode("123124984011010101001010100B11090B110907aabbccddeeff").unwrap();
        let (rest, chain) = find_block_start(data).unwrap();
        assert_eq!(rest, &[0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff][..]);
        assert_eq!(chain, Some("testnet"));

        let data = &hex::decode("010203040506070809101112FABFB5DAaabb").unwrap();
        let (rest, chain) = find_block_start(data).unwrap();
        assert_eq!(rest, &[0xaa, 0xbb][..]);
        assert_eq!(chain, Some("regtest"));

        let data = &hex::decode("010101001010100709110Baabbccddeeff").unwrap();
        assert_eq!(true, find_block_start(data).is_err());
    }
}
