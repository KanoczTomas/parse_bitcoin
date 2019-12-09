extern crate nom;
use nom::sequence::tuple;
use parse_bitcoin::parser::{
    parse_magic_number,
    parse_block_size,
    parse_block_header,
    parse_var_int,
    parse_transaction};
use std::io::prelude::*;


fn read_file(filename: &str) -> std::io::Result<()>{
    let mut file = std::fs::File::open(filename)?;
    // let mut buffer = Vec::with_capacity(2048);
    let mut buffer = [0u8;1024];
    let read_bytes = file.read_exact(&mut buffer)?;
    println!("read bytes: {:?}", read_bytes);
    // let res = magic_number(&buffer);
    let res = tuple((parse_magic_number, parse_block_size, parse_block_header, parse_var_int, parse_transaction)) (&buffer);
    println!("res: {:#?}", res);
    let test: nom::IResult<&[u8], &[u8]> = nom::bytes::complete::take(0u32)(&[0,1,2,3][..]);
    println!("{:?}", test);
    let ize = match test {
        Ok(i) => i,
        Err(e) => (&[0][..],&[0][..])
    };
    println!("ize: {:?}", ize.1.len());
    Ok(())
}

fn main() {
    match read_file("/home/tk/bin/bisq/docs/autosetup-regtest-dao/regtest/blocks/blk00000.dat") {
    // match read_file(".gitignore") {
        Ok(_) => println!("all went fine!"),
        Err(e) => println!("Error is {:?}", e)
    }
}
