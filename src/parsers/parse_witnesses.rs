use crate::{
    parsers::parse_var_int,
    types::Witness
};
use nom::{
    multi::length_data,
    IResult
};

pub fn parse_witnesses(input: &[u8]) -> IResult<&[u8], (Vec<Witness>, usize)> {
    let len_start = input.len();
    let mut vec = Vec::new();
    let (mut input, witness_count) = parse_var_int(input)?;
    if witness_count == 0 {
        vec.push(Witness::empty());
    } else {
        for _ in 0..witness_count {
            let (i, witness) = length_data(parse_var_int)(input)?;
            vec.push(witness.into());
            input = i;
        }
    }
    let witnesses_raw_size = len_start - input.len();
    Ok((input, (vec, witnesses_raw_size)))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::Bytes;
    //all macros use wire format!!
    //test_witness(witness, "" | "witness_script";
    #[macro_export]
    macro_rules! test_witness {
        ($witness:expr, $witness_script:expr) => {{
            match $witness {
                Witness(None) => {
                    assert_eq!("", $witness_script);
                }
                Witness(Some(Bytes(bytes))) => assert_eq!(hex::encode(bytes), $witness_script),
            }
        }};
    }
    #[test]
    fn test_parse_witnesses() {
        let data = include_bytes!("../test_data/tx_640d0279609c9047ebbffb1d0dcf78cbbe2ae12cadd41a28377e1a259ebf5b89.witnesses.bin");
        let (_, (witnesses, size)) = parse_witnesses(data).unwrap();
        assert_eq!(size, data.len());
        assert_eq!(witnesses.len(), 4);
        test_witness!(&witnesses[0], "");
        test_witness!(&witnesses[1], "3045022100aa2570dde15cdcb834e3490b8d10787decf3c0f6c388e949177d3531e99068c9022053a2decd7f5859cd5f2a583c8c12ba621f09721b3bc74a64d362bb9c2d57b27e01");
        test_witness!(&witnesses[2], "304402200da46260a1a6b6e7fe0e23372adcf7e9569c9f27501728a5d61ab4a3c74732b302200790fb7ce382c742b8e23f53c302b19a33cba9d68a83f33974b971511e2c712e01");
        test_witness!(&witnesses[3], "5221026c8f72b9e63db63907115e65d4da86eaae595b22fdc85ec75301bb4adbf203582103806535be3e3920e5eedee92de5714188fd6a784f2bf7b04f87de0b9c3ae1ecdb21024b23bfdce2afcae7e28c42f7f79aa100f22931712c52d7414a526ba494d44a2553ae");
    }
}
