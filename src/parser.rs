use nom::IResult;
use nom::bytes::complete::{take, tag};
use nom::number::complete::{le_u32,le_u16, le_u64, le_u8};
use nom::sequence::tuple;
use chrono::prelude::*;
use std::convert::TryInto;

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

//without header
pub fn parse_block_size (input: &[u8]) -> IResult<&[u8], u32> {
    let (i, size) = le_u32(input)?;
    Ok((i, size))
}

//warning LE on wire, keeping format!
// #[derive(Debug)]
pub struct Hash256([u8;32]);


impl Hash256 {
    fn new(slice: &[u8]) -> Hash256 {
        let mut arr = [0;32];
        arr.copy_from_slice(slice);
        Hash256(arr)
    }
}

//we print the hash in BE, as that is how bitcoind and block explorers show it
impl std::fmt::Debug for Hash256 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let Hash256(hash) = self;
        for byte in hash.iter().rev() {
            write!(f, "{:02X}", byte)?
        }
        write!(f, "")
    }
}

#[derive(PartialEq)]
struct Bytes(Vec<u8>);

impl Bytes {
    fn new(slice: &[u8]) -> Bytes{
        Bytes(Vec::from(slice))
    }
}

impl std::fmt::Debug for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Bytes(bytes) = self;
        // for byte in bytes.iter().rev() {
        for byte in bytes.iter() {
            write!(f, "{:02X}", byte)?
        }
        write!(f, "")
    }
}


#[derive(Debug)]
pub struct BlockHeader {
    version: u32,
    prev_block_hash: Hash256,
    merkle_root_hash: Hash256,
    time: String,
    bits: Bytes,
    nonce: Bytes
}

 impl BlockHeader {
    pub fn new(v: u32, pbh: &[u8], mrh: &[u8], t: u32, b: &[u8], n:&[u8] ) -> BlockHeader{
        BlockHeader{
            version: v,
            prev_block_hash: Hash256::new(pbh),
            merkle_root_hash: Hash256::new(mrh),
            time: chrono::Utc.timestamp(t.try_into().unwrap(), 0u32).to_rfc2822(),
            bits: Bytes::new(b),
            nonce: Bytes::new(n)
        }
    }
}

pub fn parse_block_header(input: &[u8]) -> IResult<&[u8], BlockHeader> {
    let (i, (
        version,
        prev_block_hash,
        merkle_root_hash,
        time,
        bits,
        nonce
    )) = tuple((
        le_u32, //version
        take(32 as usize), //prev_block_hash
        take(32 as usize), //merkle_root_hash
        le_u32, //time
        take(4 as usize), //bits
        take(4 as usize) //nonce
    )
    )(input)?;
    Ok((i, BlockHeader::new(
        version,
        prev_block_hash,
        merkle_root_hash,
        time,
        bits,
        nonce
    )))
}

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

#[derive(Debug)]
pub struct TxInput {
    previos_tx_hash: Hash256,
    vout: u32,
    script_sig: Bytes,
    sequence: Bytes
}

impl TxInput {
    fn new (ptx: &[u8], vout: u32, scr: &[u8], seq: &[u8]) -> TxInput {
        TxInput {
            previos_tx_hash: Hash256::new(ptx),
            vout,
            script_sig: Bytes::new(scr),
            sequence: Bytes::new(seq)
        }
    }
}

fn parse_tx_inputs(input: &[u8]) -> IResult<&[u8], Vec<TxInput>> {
    let (mut input, in_count) = parse_var_int(input)?;
    let mut vec: Vec<TxInput> = Vec::with_capacity(in_count as usize);
    for _ in 0..in_count {
        let (i, previos_tx_hash) = take(32u32)(input)?;
        let (i, vout) = le_u32(i)?;
        let (i, script_len) = parse_var_int(i)?;
        let (i, script_sig) = take(script_len)(i)?;
        // println!("script_sig: {}", String::from_utf8_lossy(script_sig));
        let (i, sequence) = take(4u32)(i)?;
        input = i;
        vec.push(TxInput::new(
            previos_tx_hash,
            vout,
            script_sig,
            sequence
        ));
    }
    Ok((input, vec))
}

#[derive(Debug)]
pub struct TxOutput {
    value: u64,
    script_pub_key: Bytes
}

impl TxOutput {
    fn new(value: u64, spk: &[u8]) -> TxOutput {
        TxOutput {
            value,
            script_pub_key: Bytes::new(spk)
        }
    }
}

fn parse_tx_outputs(input: &[u8]) -> IResult<&[u8], Vec<TxOutput>> {
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
    Ok((input, vec))
}

#[derive(Debug,PartialEq)]
pub struct Witness(Option<Bytes>);

impl Witness {
    fn new(slice: &[u8]) -> Witness {
        let witness = match slice.len() {
            0 => None,
            _ => Some(Bytes::new(slice))
        };
        Witness(witness)
    }
}

fn parse_witnesses(input: &[u8]) -> IResult<&[u8], Vec<Witness>> {
    let (mut input, witness_count) = parse_var_int(input)?;
    let mut vec: Vec<Witness> = Vec::with_capacity(witness_count as usize);
    for _ in 0..witness_count {
        let (i, witness_len) = parse_var_int(input)?;
        let (i, witness) = take(witness_len)(i)?;
        input = i;
        vec.push(Witness::new(witness))
    }
    Ok((input, vec))
}

#[derive(Debug)]
pub struct Transaction {
    version: u32,
    in_count: usize,
    inputs: Vec<TxInput>,
    out_count: usize,
    outputs: Vec<TxOutput>,
    witnesses: Option<Vec<Witness>>,
    lock_time: u32
}

impl Transaction {
    fn new (version: u32, in_count: usize, inputs: Vec<TxInput>,
            out_count: usize, outputs: Vec<TxOutput>, witnesses: Option<Vec<Witness>>,
            lock_time: u32) -> Transaction {
        Transaction {
            version,
            in_count,
            inputs,
            out_count,
            outputs,
            witnesses,
            lock_time
        }
    }
}

pub fn parse_transaction (input: &[u8]) -> IResult<&[u8], Transaction> {
    let (input,version) = le_u32(input)?;
    let res : IResult<&[u8], &[u8]> = tag([0x00,0x01])(input);
    let (input, witness_data) = match res {
        Ok((input, _)) => (input, true),
        Err(_) => (input, false)
    };
    let (input, inputs) = parse_tx_inputs(input)?;
    let in_count = inputs.len();
    let (mut input, outputs) = parse_tx_outputs(input)?;
    let out_count = outputs.len();
    let witnesses = if witness_data == true {
        let (i, witnesses) = parse_witnesses(input)?;
        input = i;
        Some(witnesses)
    } else {
        None
    };
    let (input, lock_time) = le_u32(input)?;
    Ok((input,Transaction::new(
        version,
        in_count,
        inputs,
        out_count,
        outputs,
        witnesses,
        lock_time
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;
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
    #[test]
    fn test_parse_tx_inputs() {
        let data = include_bytes!("tx_640d0279609c9047ebbffb1d0dcf78cbbe2ae12cadd41a28377e1a259ebf5b89.input.bin");
        let (_, inputs) = parse_tx_inputs(data).unwrap();
        assert_eq!(inputs.len(),5);
        //test_input!(input, input_index, "hash",vout,"scriptsig","sequence")
        macro_rules! test_input {
            ($input:expr, $hash:expr, $vout:expr, $script_sig:expr, $sequence:expr) => {
                {
                let Hash256(hash) = $input.previos_tx_hash;
                assert_eq!(hex::encode(hash), $hash);
                assert_eq!($input.vout, $vout);
                let Bytes(script_sig) = &$input.script_sig;
                assert_eq!(hex::encode(script_sig), $script_sig);
                let Bytes(sequence) = &$input.sequence;
                assert_eq!(hex::encode(sequence), $sequence);
                }
            };
        }
        test_input!(
            &inputs[0],
            "18b120842f139d232fa9ae944d38f3657aaa83ee3acb773cdafce39c0095bc65",
            0,
            "220020bcf9f822194145acea0f3235f4107b5bf1a91b6b9f8489f63bf79ec29b360913",
            "ffffffff"
        );
        test_input!(
            &inputs[1],
            "e0d2b92daf4a117bc2ef18cb53fc075588db552e62336ece80384dc4e9b26e94",
            0,
            "4830450221008c89d5443e21c6db957ae6238f642e293c501492ad35ab0dc31d79f7f5e3128c02206e6b33b8eead01a1a0cf4e493432c543eb7000ff9077ebded4d6df0f46ab51dd012103efb03c939c79c5b2609c4e4cf296455a4e40688d8f5e89dcda25088049b252cb",
            "ffffffff"
        );
        test_input!(
            &inputs[2],
            "5a55d746ea6c651e0a9830f1129519fbf2afad9551adf41b345b76c28cf1a79c",
            0,
            "483045022100a37a74bf92e77e80a56838d8d4333111e5dcf7029c0fed82a5f777bd37431b1102202c13c26350215cba09d359cef055170d5629ce28ebbd6ee34c66b4ac2a240c57012102bc454fb76c8fb5517c81853458e0cb42c1136869ab7d62250a39261c5c63c43e",
            "ffffffff"
        );
        test_input!(
            &inputs[3],
            "03d843b16ecaa13a0371286d478073728feeac367888f6f146f58dec36cf3351",
            0,
            "483045022100a152a58ceeaa2a8989bb975e84bf3a68ba740bd31e0dd66d72bad64dac8b39b202201c45aeda6a69e364b72390ed8a28d25b10208f7db23c8b5bb54c7ed6122694c2012103f62f4b41ff70a5b6398c961d4c7bae47942ae37b7e1ed00324375af8d005a336",
            "ffffffff"
        );
        test_input!(
            &inputs[4],
            "6a539477a0d1e2760678751d5a3c8667c72b0287e8ea1d347025cc9a45638de7",
            0,
            "473044022075c22dbd96f00c265d8eef217b9c48692334e6cca0c1a49c760b7e47a6273c8202203b25a16ba1aeb6626e4655fbc782253ba1d2666ccdd72638503c1d055d4eeb40012102e162d3d6f52b56dbf59f35ea977d5683b546105fbc9a638b64262192b9ed2da4",
            "ffffffff"
        );
    }
    #[test]
    fn test_parse_tx_outputs(){
        let data = include_bytes!("tx_640d0279609c9047ebbffb1d0dcf78cbbe2ae12cadd41a28377e1a259ebf5b89.output.bin");
        let (_, outputs) = parse_tx_outputs(data).unwrap();
        println!("{:#?}", outputs);
        assert_eq!(outputs.len(), 2);
        //test_output(output, value, script_pub_key)
        macro_rules! test_output {
            ($output:expr, $value:expr, $script_pub_key:expr) => {
                {
                    assert_eq!($output.value,$value);
                    let Bytes(script_pub_key) = &$output.script_pub_key;
                    assert_eq!(hex::encode(script_pub_key), $script_pub_key)
                }
            };
        }
        test_output!(outputs[0], 7357023, "a91430897cc6c9d69f6a2c2f1c651d51f22219f1a4f687");
        test_output!(outputs[1], 28734702, "a914fa68aba99b21ce4bba393eacc17305fe12f9021b87");
    }
    #[test]
    fn test_parse_witnesses() {
        let data = include_bytes!("tx_640d0279609c9047ebbffb1d0dcf78cbbe2ae12cadd41a28377e1a259ebf5b89.witnesses.bin");
        let (_, witnesses) = parse_witnesses(data).unwrap();
        // println!("{:#?}", witnesses );
        assert_eq!(witnesses.len(), 4);
        //test_witness(witness, "" | "witness_script";
        macro_rules! test_witness {
            ($witness:expr, $witness_script:expr) => {
                {
                    match $witness {
                        Witness(None) => {
                            assert_eq!("", $witness_script);
                        }
                        Witness(Some(Bytes(bytes))) => {
                            assert_eq!(hex::encode(bytes), $witness_script)
                        }
                    }
                }
            }
        };
        test_witness!(&witnesses[0], "");
        test_witness!(&witnesses[1], "3045022100aa2570dde15cdcb834e3490b8d10787decf3c0f6c388e949177d3531e99068c9022053a2decd7f5859cd5f2a583c8c12ba621f09721b3bc74a64d362bb9c2d57b27e01");
        test_witness!(&witnesses[2], "304402200da46260a1a6b6e7fe0e23372adcf7e9569c9f27501728a5d61ab4a3c74732b302200790fb7ce382c742b8e23f53c302b19a33cba9d68a83f33974b971511e2c712e01");
        test_witness!(&witnesses[3], "5221026c8f72b9e63db63907115e65d4da86eaae595b22fdc85ec75301bb4adbf203582103806535be3e3920e5eedee92de5714188fd6a784f2bf7b04f87de0b9c3ae1ecdb21024b23bfdce2afcae7e28c42f7f79aa100f22931712c52d7414a526ba494d44a2553ae");
    }
}
