use hex;
use nom::number::complete::le_u32;
use parse_bitcoin::parsers::parse_block;
use parse_bitcoin::utils::find_block_start;
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

fn read_file<P: AsRef<Path>>(filename: P) -> std::io::Result<()> {
    let mut file = std::fs::File::open(&filename)?;
    let mut buffer = Vec::new();
    let read_bytes = file.read_to_end(&mut buffer)?;
    println!(
        "read {} MiB from {} file",
        read_bytes / 1024 / 1024,
        filename.as_ref().display()
    );
    let mut input = &buffer[..];
    // let n = 3;
    // let mut blocks = Vec::with_capacity(n);
    let mut blocks = Vec::new();
    // for x in 0..n {
    let mut blk_count = 0;
    let mut error_count = 0;
    let mut chains = HashMap::new();
    loop {
        // println!("part of input: {}", hex::encode(&input[0..63]));
        // println!("searching for a block start ...");
        let (i, blockchain) = match find_block_start(input) {
            Ok(res) => res,
            Err(nom::Err::Error(([0, 0, 0], nom::error::ErrorKind::Eof))) => {
                // println!("no magic number found, but fond 0 bytes in a row, incrementing counter");
                // error_count += 1;
                // if error_count > 3 {
                //     println!("we probably have a ton of zeros, input(64): {}", hex::encode(&input[0..63]));
                //     println!("aborting");
                //     break;
                // }
                // continue;
                println!("no magic number found and we have 0 bytes following, aborting read!");
                break;
            }
            Err(nom::Err::Error(([], nom::error::ErrorKind::Eof))) => {
                println!("end of file reached, exiting search for magic number");
                break;
            }
            Err(e) => {
                println!("no magic number found, skipping a byte: {:?}", e);
                error_count += 1;
                continue;
            }
        };
        let blkch_counter = chains.entry(blockchain).or_insert(0);
        *blkch_counter += 1;
        // println!("part of input after find_block_start: {}", hex::encode(&i[0..63]));
        // println!("found block with magic number signalling: {}", blockchain.unwrap());
        let (i, block_size) = match le_u32::<()>(i) {
            Ok(res) => res,
            Err(e) => {
                println!("There was an error {:?}", e);
                break;
            }
        };
        // println!("part of input after parse_var_int: {}", hex::encode(&i[0..63]));
        // println!("block has size {} Bytes", block_size);
        // let (i, o) = match parse_block (&i[0..block_size as usize]){
        let (i, o) = match parse_block(i) {
            Ok(res) => res,
            Err(nom::Err::Error((i, e))) => {
                println!("therewas an error {:?}", e);
                input = i;
                break;
            }
            Err(nom::Err::Failure((i, e))) => {
                println!("there was a failure {:?}", e);
                input = i;
                break;
            }
            Err(nom::Err::Incomplete(needed)) => {
                println!("we received incomplete, need {:?} bytes", needed);
                break;
            }
        };
        // println!("part of input after parse_block: {}", hex::encode(&i[0..63]));
        // println!("block found: {:?}", o);
        blocks.push(o);
        blk_count += 1;
        print!("\rprocessed {} blocks  ", blk_count);
        io::stdout().flush()?;
        input = i;
    }
    // println!("res: {:?}", blocks);
    println!("found {} blocks", blocks.len());
    println!("chains: {:?}", chains);
    Ok(())
}

fn main() -> Result<(), std::boxed::Box<dyn Error>> {
    let mut args = std::env::args_os();
    args.next().expect("Not even zeroth argument given");
    let block_file = args.next().unwrap_or_else(|| {
        let home_dir = std::env::var_os("HOME").expect("HOME environment variable is missing");
        let mut block_file = PathBuf::from(home_dir);
        block_file.push(".bitcoin/blocks/blk02063.dat");
        block_file.into()
    });
    match read_file(&block_file) {
        Ok(_) => println!("all went fine!"),
        // Err(e) => println!("Error is e")
        Err(e) => println!("Error is {:?}", e),
    }
    Ok(())
}

//spravit parse dat file util script a vyhodi to Vec<Block> + statistiky (hasmap toho, ze kolko nasiel blokov a magic numberov atd.)

//implementni marker trait BE a LE, potom mozes implementovat from trait z jedneho na druhy a nasledne implementnut display trate pre to
