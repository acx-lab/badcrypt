extern crate encode;

use encode::FromHex;

const CIPHER: &'static str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

fn main() {
    let buf = Vec::from_hex(CIPHER).unwrap();

    let guess = xor::do_best_guess(buf);
    println!("{}:{}", guess.score, guess.phrase);
}
