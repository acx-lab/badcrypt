extern crate xor;
use std::env;
use std::fs::read_to_string;
use std::io::prelude::*;
use std::path::Path;
use xor::cbc;

const KEY: &[u8] = b"YELLOW SUBMARINE";

fn main() {
    // len 16
    let args: Vec<String> = env::args().collect();
    let src = args.get(1).expect("Usage: <B64 CIPHER>");
    let cipher = read_to_string(&Path::new(src)).unwrap();
    let buf = base64::decode(cipher.replace("\n", "").as_str()).unwrap();
    println!(
        "{}",
        String::from_utf8(cbc::decrypt(buf.as_slice(), KEY)).unwrap()
    );
}
