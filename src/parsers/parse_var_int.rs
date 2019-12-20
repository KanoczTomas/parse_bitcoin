use nom::IResult;
use nom::number::complete::{le_u32, le_u16, le_u64, le_u8};

pub fn parse_var_int(input: &[u8]) -> IResult<&[u8], u64> {
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
mod test {
    use super::*;
    #[test]
    fn test_parse_var_int() {
        assert_eq!(parse_var_int(&[0xFA][..]), Ok((&[][..],0xFAu64)));
        assert_eq!(parse_var_int(&[0xFA,0xAA][..]), Ok((&[0xAA][..],0xFAu64)));
        assert_eq!(parse_var_int(&[0xFD,0xAA,0xBB][..]), Ok((&[][..],0xBBAAu64)));
        assert_eq!(parse_var_int(&[0xFD,0xAA,0xBB, 0xCC][..]), Ok((&[0xCC][..],0xBBAAu64)));
        assert_eq!(parse_var_int(&[0xFE,0xAA,0xBB, 0xCC, 0xDD][..]), Ok((&[][..],0xDDCCBBAAu64)));
        assert_eq!(parse_var_int(&[0xFE,0xAA,0xBB, 0xCC, 0xDD, 0xEE][..]), Ok((&[0xEE][..],0xDDCCBBAAu64)));
        assert_eq!(parse_var_int(&[0xFF,0xAA,0xBB, 0xCC, 0xDD,0xEE, 0xFF,0x10, 0x09][..]), Ok((&[][..],0x0910FFEEDDCCBBAAu64)));
        assert_eq!(parse_var_int(&[0xFF,0xAA,0xBB, 0xCC, 0xDD,0xEE, 0xFF,0x10, 0x09,0x08][..]), Ok((&[0x08][..],0x0910FFEEDDCCBBAAu64)));
    }
}
