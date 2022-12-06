use std::{error::Error, fs::File, io::Read, path::PathBuf};

use clap::Parser;

/// Simple program to compute the reverse complement of a DNA strand
#[derive(Parser, Debug)]
struct Args {
    input_file: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut data = String::new();
    // DRY taken to the extreme
    args.input_file
        .map(File::open)
        .transpose()?
        .map(|fi| Box::new(fi) as Box<dyn Read>)
        .unwrap_or(Box::new(std::io::stdin()))
        .read_to_string(&mut data)?;

    println!(
        "{}",
        data.chars()
            .rev()
            .map(|c| match c {
                'A' => 'T',
                'T' => 'A',
                'C' => 'G',
                'G' => 'C',
                x => x,
            })
            .collect::<String>()
    );

    Ok(())
}
