extern crate encode;

use encode::padding::pkcs7;
use std::env;

fn main() {
    // len 16
    let args: Vec<String> = env::args().collect();
    let src = args.get(1).expect("Usage: <SOURCE TEXT>");
    println!("{:?}", pkcs7(src.as_bytes(), 20));
}
