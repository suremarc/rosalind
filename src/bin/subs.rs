#![feature(iter_next_chunk)]

use std::{
    error::Error,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use clap::Parser;

/// Simple program to compute Hamming distance
#[derive(Parser, Debug)]
struct Args {
    input_file: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let reader = BufReader::new(rosalind::reader(args.input_file.as_deref())?);
    let mut lines = reader.lines();
    let d1 = lines.next().unwrap()?;
    let d2 = lines.next().unwrap()?;

    for i in 0..d1.len() - d2.len() {
        if d1[i..i + d2.len()] == d2 {
            print!("{} ", i + 1);
        }
    }

    println!();

    Ok(())
}
