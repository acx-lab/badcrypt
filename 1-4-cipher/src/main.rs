extern crate encode;

use encode::FromHex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// A basic scoring algorithm based on frequencies of characters in a text specimen
/// of 40,000 words. Each character adds to the score weighted by the expected
/// frequency and the highest score wins.
///
/// http://pi.math.cornell.edu/~mec/2003-2004/cryptography/subs/frequencies.html
fn score(phrase: &str) -> f64 {
    let mut freq: HashMap<char, i32> = HashMap::new();
    for c in phrase.chars() {
        let e = freq.entry(c).or_insert(0);
        *e += 1;
    }

    let mut score = f64::from(1);
    for (k, v) in freq.iter() {
        score += (match k {
            'a' | 'A' => (8.12 * f64::from(*v)),
            'b' | 'B' => (1.49 * f64::from(*v)),
            'c' | 'C' => (2.71 * f64::from(*v)),
            'd' | 'D' => (4.32 * f64::from(*v)),
            'e' | 'E' => (12.02 * f64::from(*v)),
            'f' | 'F' => (2.30 * f64::from(*v)),
            'g' | 'G' => (2.03 * f64::from(*v)),
            'h' | 'H' => (5.92 * f64::from(*v)),
            'i' | 'I' => (7.31 * f64::from(*v)),
            'j' | 'J' => (0.10 * f64::from(*v)),
            'k' | 'K' => (0.69 * f64::from(*v)),
            'l' | 'L' => (3.98 * f64::from(*v)),
            'm' | 'M' => (2.61 * f64::from(*v)),
            'n' | 'N' => (6.95 * f64::from(*v)),
            'o' | 'O' => (7.68 * f64::from(*v)),
            'p' | 'P' => (1.82 * f64::from(*v)),
            'q' | 'Q' => (0.11 * f64::from(*v)),
            'r' | 'R' => (6.02 * f64::from(*v)),
            's' | 'S' => (6.28 * f64::from(*v)),
            't' | 'T' => (9.10 * f64::from(*v)),
            'u' | 'U' => (2.88 * f64::from(*v)),
            'v' | 'V' => (1.11 * f64::from(*v)),
            'w' | 'W' => (2.09 * f64::from(*v)),
            'x' | 'X' => (0.17 * f64::from(*v)),
            'y' | 'Y' => (2.11 * f64::from(*v)),
            'z' | 'Z' => (0.07 * f64::from(*v)),
            // Real messages have whitespace. This wasn't included in the character
            // frequency reference so I gave it a low-ish value.
            ' ' => (10.0 * f64::from(*v)),
            _ => f64::from(1),
        }) * f64::from(*v);
    }
    score
}

/// Returns a decrypted message from a string buffer.
fn decrypt(c: &Vec<u8>, key: u8) -> String {
    let mut decrypted: Vec<u8> = vec![];

    for v in c {
        decrypted.push(v ^ key);
    }
    // Some of the descrypted byte sequences don't produce valid utf-8 strings.
    // Since it's a message we're looking for, this is a useful gate.
    return String::from_utf8(decrypted).unwrap_or("".to_string());
}

struct Guess {
    phrase: String,
    score: f64,
}

fn do_best_guess(phrase: Vec<u8>) -> Guess {
    let mut best_guess = Guess {
        phrase: "".to_string(),
        score: f64::from(0),
    };

    // Not sure if the characters are limited to letters. Run through a wide range
    // of ascii characters.
    for c in b' '..b'~' {
        let phrase = decrypt(&phrase, c);
        let s = score(phrase.as_str());
        if s > best_guess.score {
            best_guess.phrase = phrase;
            best_guess.score = s;
        }
    }
    best_guess
}

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
        guesses.push(do_best_guess(buf));
    }

    for guess in guesses {
        println!("{},{}", guess.score, guess.phrase);
    }
}
