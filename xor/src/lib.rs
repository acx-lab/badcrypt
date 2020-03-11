extern crate encode;
extern crate openssl;

pub mod cbc;

use encode::FromHex;
use std::collections::HashMap;

/// Count the number of bits that differ between two sequence of bytes.
pub fn hamming_distance(first: Vec<u8>, second: Vec<u8>) -> u32 {
    let mut distance = 0;
    for (i, x) in first.iter().enumerate() {
        // FIXME(allancalix): Fails if the two bytes are not equal in length. It
        // probably makes more sense to count the difference in length towards the
        // distance.
        let y = second.get(i).unwrap();
        let mut diff = x ^ y;
        for _ in 0..8 {
            distance += (diff & 1) as u32;
            diff >>= 1;
        }
    }
    distance
}

/// A basic scoring algorithm based on frequencies of characters in a text specimen
/// of 40,000 words. The score is the sum of the distances from the expected
/// character frequencies and the real chracter frequencies. The higher the score,
/// the more the frequencies align with the expected distribution.
///
/// http://pi.math.cornell.edu/~mec/2003-2004/cryptography/subs/frequencies.html
pub fn score(phrase: &str) -> u64 {
    let mut freq: HashMap<char, i32> = HashMap::new();

    let upper_count = phrase
        .chars()
        .filter(|c| c.is_uppercase())
        .collect::<Vec<_>>()
        .len();

    // FIXME(allancalix): This is a hack to bias against data decrypted with
    // an abnormal amount of capital letters. This can happen because the same key,
    // (i.e. 'i' and 'I') can xor to the same character with opposite casing.
    let multiplier = if upper_count > phrase.len() / 2 { 2 } else { 1 };

    // Init frequency map "buckets". This ensures that a character with no characters
    // is still counted against for having a distribution of 0%.
    for c in b'a'..=b'z' {
        freq.insert(char::from(c), 0);
    }
    freq.insert(' ', 0);
    for c in phrase.to_lowercase().chars() {
        match c {
            'a'..='z' | ' ' => {
                let e = freq.entry(c).or_insert(0);
                *e += 1;
            }
            _ => {
                let e = freq.entry('*').or_insert(0);
                *e += 1;
            }
        }
    }

    let hist = freq
        .into_iter()
        .map(|(k, v)| {
            let expected_freq = match k {
                'a' => 8.12,
                'b' => 1.49,
                'c' => 2.71,
                'd' => 4.32,
                'e' => 12.02,
                'f' => 2.30,
                'g' => 2.03,
                'h' => 5.92,
                'i' => 7.31,
                'j' => 0.10,
                'k' => 0.69,
                'l' => 3.98,
                'm' => 2.61,
                'n' => 6.95,
                'o' => 7.68,
                'p' => 1.82,
                'q' => 0.11,
                'r' => 6.02,
                's' => 6.28,
                't' => 9.10,
                'u' => 2.88,
                'v' => 1.11,
                'w' => 2.09,
                'x' => 0.17,
                'y' => 2.11,
                'z' => 0.07,
                ' ' => 13.0,
                // Any other characters bias against overall score.
                '*' => 0.0,
                _ => panic!("This should never happen, non letter characters should be *."),
            } * 0.01;
            let real_freq = v as f64 / phrase.len() as f64;
            let distance = (expected_freq - real_freq).abs();
            let score = (distance * f64::from(1000)) as u64;
            (k, score)
        })
        .collect::<HashMap<char, u64>>();
    hist.into_iter().map(|(_, v)| v).sum::<u64>() * multiplier
}

/// Returns a decrypted message from a string buffer.
pub fn decrypt(cipher: &Vec<u8>, key: &str) -> Vec<u8> {
    let mut cv = key.as_bytes().iter().cycle();

    return cipher
        .iter()
        .map(|c| {
            let kv = cv.next().unwrap();
            c ^ *kv as u8
        })
        .collect();
}

#[derive(Debug, Clone)]
pub struct Guess {
    pub phrase: String,
    pub score: u64,
    pub key: char,
}

pub fn do_single_letter_key_speculation(phrase: Vec<u8>) -> Guess {
    // Not sure if the characters are limited to letters. Run through a wide range
    // of ascii characters.
    let mut scores = vec![];
    for c in b' '..=b'~' {
        let phrase = decrypt(&phrase, String::from_utf8(vec![c]).unwrap().as_str());
        let encoded_phrase = String::from_utf8(phrase).unwrap_or("zzzzzzzzzzzzzzzz".to_string());
        let s = score(encoded_phrase.as_str());
        scores.push(Guess {
            phrase: encoded_phrase,
            score: s,
            key: char::from(c),
        });
    }

    scores.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
    scores.reverse();
    scores.last().unwrap().clone()
}

pub fn do_key_speculation(cipher: &Vec<u8>, key_size: usize) -> Vec<char> {
    buffer::chunk_by_size(cipher, key_size)
        .into_iter()
        .map(|chunk| do_single_letter_key_speculation(chunk).key)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hamming_distance() {
        let dist = hamming_distance(vec![0], vec![1]);
        assert_eq!(dist, 1);
    }

    #[test]
    fn test_crypto_example() {
        let s1 = Vec::from("this is a test");
        let s2 = Vec::from("wokka wokka!!!");
        assert_eq!(hamming_distance(s1, s2), 37);
    }

    #[test]
    fn test_decrypt_is_reversible() {
        let source = "hello i am a fake phrase";
        let phrase = decrypt(&Vec::from(source), "z");
        assert_eq!(decrypt(&Vec::from(phrase), "z"), source.as_bytes());
    }

    #[test]
    fn test_scoring_can_guess_key() {
        let source = "hello i am a fake phrase";
        let phrase = decrypt(&Vec::from(source), "z");
        // Can derive encryption key using scoring algorithm.
        assert_eq!(do_single_letter_key_speculation(Vec::from(phrase)).key, 'z');
    }

    #[test]
    fn test_scoring_can_guess_uppercase_key() {
        let source = "hello i am a FAKE phrase";
        let phrase = decrypt(&Vec::from(source), "I");
        // Can derive encryption key using scoring algorithm.
        assert_eq!(do_single_letter_key_speculation(Vec::from(phrase)).key, 'I');
    }

    #[test]
    fn test_do_key_speculation() {
        // Encrypted with "ICE" from cryptopals 1.5.
        const CIPHER: &'static str = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        let key = do_key_speculation(&Vec::from_hex(CIPHER).unwrap(), 3);
        assert_eq!(key.into_iter().collect::<String>().as_str(), "ICE");
    }

    #[test]
    fn test_do_key_speculation_with_encryption() {
        const CIPHER: &'static str = "Hello, I am a secret encoded message. Find the key!";
        let c = decrypt(&Vec::from(CIPHER), "NiInfp");
        let key = do_key_speculation(&c, 6);
        assert_eq!(key.into_iter().collect::<String>().as_str(), "NiInfp");
    }
}
