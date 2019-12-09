extern crate nom;
use nom::sequence::tuple;
use parse_bitcoin::parser::{magic_number, block_size, block_header, var_int, transaction};
use std::io::prelude::*;


fn read_file(filename: &str) -> std::io::Result<()>{
    let mut file = std::fs::File::open(filename)?;
    // let mut buffer = Vec::with_capacity(2048);
    let mut buffer = [0u8;1024];
    let read_bytes = file.read_exact(&mut buffer)?;
    println!("read bytes: {:?}", read_bytes);
    // let res = magic_number(&buffer);
    let res = tuple((magic_number, block_size, block_header, var_int, transaction)) (&buffer);
    println!("res: {:?}", res);
    Ok(())
}

fn main() {
    match read_file("/home/tk/bin/bisq/docs/autosetup-regtest-dao/regtest/blocks/blk00000.dat") {
    // match read_file(".gitignore") {
        Ok(_) => println!("all went fine!"),
        Err(e) => println!("Error is {:?}", e)
    }
}
