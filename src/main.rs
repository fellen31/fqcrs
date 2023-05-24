extern crate needletail;

use std::str;
use needletail::{parse_fastx_stdin, Sequence, FastxReader};
use std::io;

fn main() {
    let filename = "..";

    let mut n_bases = 0;
    let mut n_valid_kmers = 0;
    let mut reader = parse_fastx_stdin().expect("valid path/file");
    while let Some(record) = reader.next() {
        let seqrec = record.expect("invalid record");
        // keep track of the total number of bases
        n_bases += seqrec.num_bases();
        let mut read_name = str::from_utf8(seqrec.id())
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>();
        
        if let Some(read_name) =read_name.first() {
            
            let baseq = seqrec.qual().unwrap().iter().map(|b| *b as i32-33).collect::<Vec<_>>();
            
            println!("{}\t{}\t{}", read_name, seqrec.num_bases(), average(&baseq));

        } 
    }
}

fn average(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}
