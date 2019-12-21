use nom::IResult;
use nom::bytes::complete::take;
use nom::number::complete::le_u64;
use crate::types::TxOutput;
use crate::parsers::parse_var_int;

pub fn parse_tx_outputs(input: &[u8]) -> IResult<&[u8], (Vec<TxOutput>, usize)> {
    let len_start = input.len();
    let (mut input, out_count) = parse_var_int(input)?;
    let mut vec: Vec<TxOutput> = Vec::with_capacity(out_count as usize);
    for _ in 0..out_count {
        let (i, value) = le_u64(input)?;
        let (i, script_len) = parse_var_int(i)?;
        let (i, script_pub_key) = take(script_len)(i)?;
        input = i;
        vec.push(TxOutput::new(
            value,
            script_pub_key
        ));
    }
    let outputs_raw_size = len_start - input.len();
    Ok((input, (vec, outputs_raw_size)))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::Bytes;
    //all macros use wire format!!
    //test_output(output, value, script_pub_key)
    #[macro_export]
    macro_rules! test_output {
        ($output:expr, $value:expr, $script_pub_key:expr) => {
            {
                assert_eq!($output.value,$value);
                let Bytes(script_pub_key) = &$output.script_pub_key;
                assert_eq!(hex::encode(script_pub_key), $script_pub_key)
            }
        };
    }
    #[test]
    fn test_parse_tx_outputs(){
        let data = include_bytes!("../test_data/tx_640d0279609c9047ebbffb1d0dcf78cbbe2ae12cadd41a28377e1a259ebf5b89.output.bin");
        let (_, (outputs, size)) = parse_tx_outputs(data).unwrap();
        assert_eq!(size, data.len());
        assert_eq!(outputs.len(), 2);
        test_output!(outputs[0], 7357023, "a91430897cc6c9d69f6a2c2f1c651d51f22219f1a4f687");
        test_output!(outputs[1], 28734702, "a914fa68aba99b21ce4bba393eacc17305fe12f9021b87");
    }
}
