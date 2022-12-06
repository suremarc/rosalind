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
    let rna = reader.lines().next().unwrap()?;

    println!("{}", rosalind::protein_str(&rna));
    Ok(())
}
