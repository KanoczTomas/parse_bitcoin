// extern crate nom;
// use nom::sequence::tuple;
// use nom::multi::many_till;
// use nom::bytes::complete::take;
// use nom::combinator::peek;
// use parse_bitcoin::parser::{
//     parse_magic_number,
//     parse_block_size,
//     parse_block_header,
//     parse_var_int,
//     parse_transaction,
//     parse_block};
use parse_bitcoin::parsers::parse_block;
use std::io::prelude::*;


fn read_file(filename: &str) -> std::io::Result<()>{
    let mut file = std::fs::File::open(filename)?;
    // let mut buffer = Vec::with_capacity(2048);
    // let mut buffer = [0u8;1024];
    // let read_bytes = file.read_exact(&mut buffer)?;
    let mut buffer = Vec::new();
    let read_bytes = file.read_to_end(&mut buffer)?;
    // println!("read bytes: {:?}", read_bytes);
    // let res = magic_number(&buffer);
    // let res = tuple((parse_magic_number, parse_block_size, parse_block_header, parse_var_int, parse_transaction)) (&buffer);
    // println!("res: {:?}", res);
    let mut input = &buffer[..];
    let n = 1;
    let mut blocks = Vec::with_capacity(n);
    for x in 0..n {
        // let (i, o) = many_till(take(1usize), peek(parse_magic_number))(input).unwrap();
        // println!("I: {:?}, o: {:?} ",hex::encode(&i[0..6]),&o);
        // print!("\rparsed {} blocks", x);
        let (i, o) = match parse_block (input){
        // let (i, o) = match parse_block (input){
                Ok(res) => res,
                Err(e) => {
                    // panic!("therewas an error, {:?}", e);
                    panic!("therewas an error ");
                    // println!("error in parsing block: {:?}", e);
                }
        };
        blocks.push(o);
        input = i;
    }
    println!("res: {:?}", blocks);
    // let test: nom::IResult<&[u8], &[u8]> = nom::bytes::complete::take(0u32)(&[0,1,2,3][..]);
    // println!("{:?}", test);
    // let ize = match test {
    //     Ok(i) => i,
    //     Err(e) => (&[0][..],&[0][..])
    // };
    // println!("ize: {:?}", ize.1.len());
    Ok(())
}

fn main() {
    // match read_file("/home/tk/bin/bisq/docs/autosetup-regtest-dao/regtest/blocks/blk00000.dat") {
    match read_file("/home/tk/.bitcoin/blocks/blk01893.dat") {
    // match read_file(".gitignore") {
        Ok(_) => println!("all went fine!"),
        Err(e) => println!("Error is e")
        // Err(e) => println!("Error is {:?}",e)
    }
}
