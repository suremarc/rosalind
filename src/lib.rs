#![feature(array_chunks)]

use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

pub fn reader(path: Option<&Path>) -> io::Result<Box<dyn Read>> {
    Ok(path
        .map(File::open)
        .transpose()?
        .map(|fi| Box::new(fi) as Box<dyn Read>)
        .unwrap_or(Box::new(std::io::stdin())))
}

pub fn protein_str(rna: &str) -> String {
    rna.as_bytes()
        .array_chunks::<3>()
        .map_while(|codon| match codon {
            b"UUU" | b"UUC" => Some('F'),
            b"UUA" | b"UUG" => Some('L'),
            b"UCU" | b"UCC" | b"UCA" | b"UCG" => Some('S'),
            b"UAU" | b"UAC" => Some('Y'),
            b"UAA" | b"UAG" => None,
            b"UGU" | b"UGC" => Some('C'),
            b"UGA" => None,
            b"UGG" => Some('W'),
            b"CUU" | b"CUC" | b"CUA" | b"CUG" => Some('L'),
            b"CCU" | b"CCC" | b"CCA" | b"CCG" => Some('P'),
            b"CAU" | b"CAC" => Some('H'),
            b"CAA" | b"CAG" => Some('Q'),
            b"CGU" | b"CGC" | b"CGA" | b"CGG" => Some('R'),
            b"AUU" | b"AUC" | b"AUA" => Some('I'),
            b"AUG" => Some('M'),
            b"ACU" | b"ACC" | b"ACA" | b"ACG" => Some('T'),
            b"AAU" | b"AAC" => Some('N'),
            b"AAA" | b"AAG" => Some('K'),
            b"AGU" | b"AGC" => Some('S'),
            b"AGA" | b"AGG" => Some('R'),
            b"GUU" | b"GUC" | b"GUA" | b"GUG" => Some('V'),
            b"GCU" | b"GCC" | b"GCA" | b"GCG" => Some('A'),
            b"GAU" | b"GAC" => Some('D'),
            b"GAA" | b"GAG" => Some('E'),
            b"GGU" | b"GGC" | b"GGA" | b"GGG" => Some('G'),
            _ => panic!("unexpected sequence"),
        })
        .collect()
}
