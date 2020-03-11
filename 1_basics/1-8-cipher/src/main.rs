extern crate encode;

use encode::FromHex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Aligns chunks on key bits and counts similar bytes in each chunk. Text should
/// have a higher count of similar bytes because the same letter encrypted with
/// the same key produces the same cipher value.
fn detect_cycles(buf: &Vec<u8>, block_size: usize) -> u32 {
    let chunks = buffer::chunk_by_size(&buf, block_size);

    let mut chunk_maxes = vec![];
    for chunk in &chunks {
        let mut map: HashMap<u8, u32> = HashMap::new();
        for b in chunk {
            let e = map.entry(*b).or_insert(0);
            *e += 1;
        }
        let chunk_max = *map.iter().map(|(_, v)| v).max().unwrap();
        chunk_maxes.push(chunk_max);
    }
    chunk_maxes.iter().sum()
}

// The basic idea with this implementation is to align all buffers by key size (128 bits)
// and then try and measure how similar the blocks are to one another.
fn main() {
    let p: Vec<String> = env::args().collect();
    let cipher_file = Path::new(p.get(1).unwrap().as_str());
    let mut f = File::open(cipher_file).unwrap();

    let mut cipher = String::new();
    f.read_to_string(&mut cipher).unwrap();

    let mut results = vec![];
    for line in cipher.lines() {
        if line.is_empty() {
            continue;
        }

        let buf = Vec::from_hex(line).unwrap();
        let duplicates = detect_cycles(&buf, 16 as usize);
        results.push((duplicates, line));
    }
    results.sort_by(|x, y| x.0.cmp(&y.0));
    // Take the result with the highest count of byte collisions in all chunks.
    println!("{:?}", results.last().unwrap());
}
