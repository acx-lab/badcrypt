use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn guess_key_size(cipher: &Vec<u8>) -> Vec<u8> {
    let mut result: Vec<(f64, u8)> = vec![];
    // Recommended key size range from challenge.
    for key_size in 2..=40 {
        let first_chunk: Vec<u8> = cipher.clone().into_iter().take(key_size).collect();
        let second_chunk: Vec<u8> = cipher
            .clone()
            .into_iter()
            .skip(key_size)
            .take(key_size)
            .collect();
        let third_chunk: Vec<u8> = cipher
            .clone()
            .into_iter()
            .skip(key_size * 2)
            .take(key_size)
            .collect();
        let fourth_chunk: Vec<u8> = cipher
            .clone()
            .into_iter()
            .skip(key_size * 3)
            .take(key_size)
            .collect();

        let dist1 = xor::hamming_distance(first_chunk.clone(), second_chunk) / key_size as u32;
        let dist2 = xor::hamming_distance(first_chunk.clone(), third_chunk) / key_size as u32;
        let dist3 = xor::hamming_distance(first_chunk.clone(), fourth_chunk) / key_size as u32;
        let mean = vec![dist1, dist2, dist3]
            .into_iter()
            .map(f64::from)
            .sum::<f64>()
            / f64::from(3);
        result.push((mean, key_size as u8));
    }
    result.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap());
    // Return top three results.
    result.iter().map(|x| x.1).take(4).collect::<Vec<u8>>()
}

fn main() {
    let p: Vec<String> = env::args().collect();
    let cipher_file = Path::new(p.get(1).unwrap().as_str());
    let mut f = File::open(cipher_file).unwrap();

    let mut cipher = String::new();
    f.read_to_string(&mut cipher).unwrap();

    // Flatten the base64 cipher into a continuous string before decoding.
    let buf = base64::decode(cipher.replace("\n", "").as_str()).unwrap();
    let sizes = guess_key_size(&buf);

    let keys: Vec<Vec<char>> = sizes
        .into_iter()
        .map(|size| xor::do_key_speculation(&buf, size as usize))
        .collect();

    let mut results = vec![];
    for key in keys {
        let key: String = key.iter().collect();
        // Decipher original text.
        let decrypted = xor::decrypt(&buf, key.as_str());
        results.push((
            xor::score(String::from_utf8(decrypted.clone()).unwrap().as_str()),
            decrypted,
        ));
    }
    results.sort_by(|x, y| x.0.cmp(&y.0));
    println!(
        "{}",
        String::from_utf8(results.first().unwrap().1.clone()).unwrap()
    );
}
