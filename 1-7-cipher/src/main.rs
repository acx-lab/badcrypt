use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const KEY: &'static str = "YELLOW SUBMARINE";

fn main() {
    let p: Vec<String> = env::args().collect();
    let cipher_file = Path::new(p.get(1).unwrap().as_str());
    let mut f = File::open(cipher_file).unwrap();

    let mut cipher = String::new();
    f.read_to_string(&mut cipher).unwrap();
    let buf = base64::decode(cipher.replace("\n", "").as_str()).unwrap();
    let message = openssl::symm::decrypt(
      openssl::symm::Cipher::aes_128_ecb(),
      KEY.as_bytes(),
      None,
      &buf).unwrap();
    println!("{}", String::from_utf8(message).unwrap())
}
