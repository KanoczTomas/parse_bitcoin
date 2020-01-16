use crate::{
    parsers::parse_var_int,
    types::{TxInput, TxInputBuilder}
};
use nom::{
    bytes::complete::take,
    multi::length_data,
    number::complete::le_u32,
    sequence::tuple,
    IResult
};

pub fn parse_tx_inputs(input: &[u8]) -> IResult<&[u8], (Vec<TxInput>, usize)> {
    let len_start = input.len();
    let (mut input, in_count) = parse_var_int(input)?;
    let mut vec: Vec<TxInput> = Vec::with_capacity(in_count as usize);
    for _ in 0..in_count {
        let (i, (previous_tx_hash, vout, script_sig, sequence)) =
            tuple((take(32u32), le_u32, length_data(parse_var_int), le_u32))(input)?;
        input = i;
        vec.push(
            TxInputBuilder::new()
                .previous_tx_hash(previous_tx_hash)
                .vout(vout)
                .script_sig(script_sig)
                .sequence(sequence)
                .build(),
        );
    }
    let inputs_raw_size = len_start - input.len();
    Ok((input, (vec, inputs_raw_size)))
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::types::{Bytes, Hash256};
    //all macros use wire format!!
    //test_input!(input, "hash",vout,"scriptsig","sequence")
    #[macro_export]
    macro_rules! test_input {
        ($input:expr, $hash:expr, $vout:expr, $script_sig:expr, $sequence:expr) => {{
            let Hash256(hash) = $input.previous_tx_hash;
            assert_eq!(hex::encode(hash), $hash);
            assert_eq!($input.vout, $vout);
            let Bytes(script_sig) = &$input.script_sig;
            assert_eq!(hex::encode(script_sig), $script_sig);
            assert_eq!($input.sequence, $sequence as u32);
        }};
    }
    #[test]
    fn test_parse_tx_inputs() {
        let data = include_bytes!("../test_data/tx_640d0279609c9047ebbffb1d0dcf78cbbe2ae12cadd41a28377e1a259ebf5b89.input.bin");
        let (_, (inputs, size)) = parse_tx_inputs(data).unwrap();
        assert_eq!(size, data.len());
        assert_eq!(inputs.len(), 5);
        test_input!(
            &inputs[0],
            "18b120842f139d232fa9ae944d38f3657aaa83ee3acb773cdafce39c0095bc65",
            0,
            "220020bcf9f822194145acea0f3235f4107b5bf1a91b6b9f8489f63bf79ec29b360913",
            4294967295
        );
        test_input!(
            &inputs[1],
            "e0d2b92daf4a117bc2ef18cb53fc075588db552e62336ece80384dc4e9b26e94",
            0,
            "4830450221008c89d5443e21c6db957ae6238f642e293c501492ad35ab0dc31d79f7f5e3128c02206e6b33b8eead01a1a0cf4e493432c543eb7000ff9077ebded4d6df0f46ab51dd012103efb03c939c79c5b2609c4e4cf296455a4e40688d8f5e89dcda25088049b252cb",
            4294967295
        );
        test_input!(
            &inputs[2],
            "5a55d746ea6c651e0a9830f1129519fbf2afad9551adf41b345b76c28cf1a79c",
            0,
            "483045022100a37a74bf92e77e80a56838d8d4333111e5dcf7029c0fed82a5f777bd37431b1102202c13c26350215cba09d359cef055170d5629ce28ebbd6ee34c66b4ac2a240c57012102bc454fb76c8fb5517c81853458e0cb42c1136869ab7d62250a39261c5c63c43e",
            4294967295
        );
        test_input!(
            &inputs[3],
            "03d843b16ecaa13a0371286d478073728feeac367888f6f146f58dec36cf3351",
            0,
            "483045022100a152a58ceeaa2a8989bb975e84bf3a68ba740bd31e0dd66d72bad64dac8b39b202201c45aeda6a69e364b72390ed8a28d25b10208f7db23c8b5bb54c7ed6122694c2012103f62f4b41ff70a5b6398c961d4c7bae47942ae37b7e1ed00324375af8d005a336",
            4294967295
        );
        test_input!(
            &inputs[4],
            "6a539477a0d1e2760678751d5a3c8667c72b0287e8ea1d347025cc9a45638de7",
            0,
            "473044022075c22dbd96f00c265d8eef217b9c48692334e6cca0c1a49c760b7e47a6273c8202203b25a16ba1aeb6626e4655fbc782253ba1d2666ccdd72638503c1d055d4eeb40012102e162d3d6f52b56dbf59f35ea977d5683b546105fbc9a638b64262192b9ed2da4",
            4294967295
        );
    }
}
