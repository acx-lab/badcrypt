extern crate encode;

use encode::FromHex;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// This command will just spit out csv-like results of scores and strings. To
// output the final result, run it through sort and truncate.
//
// cargo run --bin 1-4-cipher -- data/1-4-cipher.txt | sort --reverse | head -n 1
fn main() {
    let all_args: Vec<String> = env::args().collect();
    let data_fp = Path::new(all_args.get(1).unwrap().as_str());
    let mut f = File::open(data_fp).unwrap();
    let mut ciphers = String::new();
    f.read_to_string(&mut ciphers).unwrap();

    let mut guesses = vec![];
    for line in ciphers.lines() {
        let buf = Vec::from_hex(line).unwrap();
        guesses.push(xor::do_single_letter_key_speculation(buf));
    }

    for guess in guesses {
        println!("{},{}", guess.score, guess.phrase);
    }
}
