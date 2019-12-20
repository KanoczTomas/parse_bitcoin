use nom::number::complete::le_u32;
use nom::IResult;

pub fn parse_magic_number(input: &[u8]) -> IResult<&[u8], &str> {
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
