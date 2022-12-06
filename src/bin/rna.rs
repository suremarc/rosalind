#![feature(portable_simd)]

use std::{error::Error, fs::File, io::Read, path::PathBuf};

use clap::Parser;

/// Simple program to count base pairs
#[derive(Parser, Debug)]
struct Args {
    input_file: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut data = String::new();
    File::open(args.input_file)?.read_to_string(&mut data)?;

    println!(
        "{}",
        data.chars()
            .map(|c| if c == 'T' { 'U' } else { c })
            .collect::<String>()
    );

    Ok(())
}
