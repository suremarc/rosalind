#![feature(iter_next_chunk)]

use std::error::Error;

use clap::Parser;
use itertools::Itertools;

/// Simple program to compute Hamming distance
#[derive(Parser, Debug)]
struct Args {
    n: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let perms = (0..args.n).permutations(args.n);
    let n = perms.clone().count();

    println!("{n}");
    for perm in perms {
        for x in perm.iter().map(|k| k + 1) {
            print!("{x} ");
        }
        println!();
    }

    Ok(())
}
