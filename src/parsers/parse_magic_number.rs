use nom::number::complete::le_u32;
use nom::IResult;
use crate::types::Network;

pub fn parse_magic_number(input: &[u8]) -> IResult<&[u8], Option<Network>> {
    let (i, o) = le_u32(input)?;
    let result = match o {
        0xD9B4BEF9 => Some(Network::Mainnet),
        0xDAB5BFFA => Some(Network::Regtest),
        0x0709110B => Some(Network::Testnet),
        0xFEB4BEF9 => Some(Network::Namecoin),
        _ => None,
    };
    Ok((i, result))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_magic_number() {
        let data = &[0xf9, 0xbe, 0xb4, 0xd9][..];
        let (_, chain) = parse_magic_number(data).unwrap();
        assert_eq!(chain, Some(Network::Mainnet));
        let data = &[0xfa, 0xbf, 0xb5, 0xda][..];
        let (_, chain) = parse_magic_number(data).unwrap();
        assert_eq!(chain, Some(Network::Regtest));
        let data = &[0x0b, 0x11, 0x09, 0x07][..];
        let (_, chain) = parse_magic_number(data).unwrap();
        assert_eq!(chain, Some(Network::Testnet));
        let data = &[0xf9, 0xbe, 0xb4, 0xfe][..];
        let (_, chain) = parse_magic_number(data).unwrap();
        assert_eq!(chain, Some(Network::Namecoin));
        let data = &[0xf9, 0xbe, 0xb4, 0xff][..];
        let (_, chain) = parse_magic_number(data).unwrap();
        assert_eq!(chain, None);
        let data = &[0xf9, 0x00, 0xb4, 0xff][..];
        let (_, chain) = parse_magic_number(data).unwrap();
        assert_eq!(chain, None);
        let data = &[0xf9, 0xb4, 0xff, 0xff][..];
        let (_, chain) = parse_magic_number(data).unwrap();
        assert_eq!(chain, None);
    }
}
