extern crate needletail;

use needletail::{parse_fastx_stdin};
use std::str;

fn main() {

    let mut reader = parse_fastx_stdin().expect("valid path/file");
    
    while let Some(record) = reader.next() {
        let seqrec = record.expect("invalid record");
        
        let read_name = str::from_utf8(seqrec.id())
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>();
        
        if let Some(read_name) =read_name.first() {       
            let baseq = seqrec.qual().unwrap().iter().map(|b| *b as i32-33).collect::<Vec<_>>();
            
            println!("{}\t{}\t{}", read_name, seqrec.num_bases(), average_qual(&baseq));

        } 
    }
}

fn average_qual(quals: &[i32]) -> f32 {
    let mut sum = 0 as f32; 
    
    for q in quals.iter() {
        sum = sum + 10_f32.powf(*q as f32/-10 as f32);
    }
    
    return -10_f32 * (sum / quals.len() as f32).log10(); 
}
