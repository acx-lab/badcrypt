use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn guess_key_size(cipher: &Vec<u8>) -> Vec<u8> {
    let mut result = vec![];
    // Recommended key size range from challenge.
    for key_size in 2..=40 {
        let first_chunk: Vec<u8> = cipher
          .clone()
          .into_iter()
          .take(key_size)
          .collect();
        let second_chunk: Vec<u8> = cipher
            .clone()
            .into_iter()
            .skip(key_size)
            .take(key_size)
            .collect();

        let dist = xor::hamming_distance(first_chunk, second_chunk) / key_size as u8;
        result.push((dist, key_size as u8));
    }
    result.sort_by(|x, y| x.0.cmp(&y.0));
    // Return top three results.
    result.iter().map(|x| x.1).take(3).collect::<Vec<u8>>()
}

fn main() {
    let p: Vec<String> = env::args().collect();
    let cipher_file = Path::new(p.get(1).unwrap().as_str());
    let mut f = File::open(cipher_file).unwrap();

    let mut cipher = String::new();
    f.read_to_string(&mut cipher).unwrap();

    // Flatten the base64 cipher into a continuous string.
    let buf = base64::decode(cipher.replace("\n", "").as_str()).unwrap();
    let sizes = guess_key_size(&buf);

    // Guess a key based on the estimated size of the key.
    let mut keys: Vec<Vec<char>> = vec![];
    for size in sizes {
        let mut key_guess = vec![];
        for i in 0..(size as usize) {
            let block: Vec<u8> = buf
                .clone()
                .into_iter()
                .skip(i)
                .step_by(size as usize)
                .collect();
            key_guess.push(xor::do_best_guess(block).key.unwrap());
        }
        keys.push(key_guess);
      }

    for key in keys {
        let mut cv = key.iter().cycle();
        // Decipher original text.
        let text: Vec<u8> = buf
            .iter()
            .map(|c| {
                let kv = cv.next().unwrap();
                c ^ *kv as u8
            })
            .collect();
        // println!("{:?}", key);
        println!("{}", String::from_utf8(text).unwrap_or("failed to parse".to_string()));
    }
}
