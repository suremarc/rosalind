#![feature(portable_simd)]

use memmap::MmapOptions;
use std::{
    error::Error,
    fs::File,
    path::PathBuf,
    simd::{u8x64, Simd, SimdPartialEq, ToBitMask},
};

use clap::Parser;

/// Simple program to count base pairs
#[derive(Parser, Debug)]
struct Args {
    input_file: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let data = unsafe { MmapOptions::new().map(&File::open(args.input_file)?) }?;

    let (head, body, tail) = unsafe { data.align_to::<u8x64>() };

    let mut cnt = [0u32; 4];
    for elem in body {
        for (i, &base) in b"ACGT".iter().enumerate() {
            cnt[i] += elem.simd_eq(Simd::splat(base)).to_bitmask().count_ones();
        }
    }

    for &c in head.iter().chain(tail) {
        for (i, &base) in b"ACGT".iter().enumerate() {
            if c == base {
                cnt[i] += 1;
                break;
            }
        }
    }

    println!("{} {} {} {}", cnt[0], cnt[1], cnt[2], cnt[3]);

    Ok(())
}
