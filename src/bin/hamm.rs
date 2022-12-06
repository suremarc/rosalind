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
    if let [Ok(d1), Ok(d2)] = lines.next_chunk().unwrap() {
        let mut sum = 0;
        for (x, y) in d1.chars().zip(d2.chars()) {
            if x != y {
                sum += 1;
            }
        }
        // println!("{d1}, {d2}");
        println!("{sum}");
    } else {
        panic!("not enough lines");
    }

    Ok(())
}
