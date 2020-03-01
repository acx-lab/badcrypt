use std::collections::HashMap;

/// Count the number of bits that differ between two sequence of bytes.
pub fn hamming_distance(first: Vec<u8>, second: Vec<u8>) -> u8 {
    let mut distance = 0;
    for (i, x) in first.iter().enumerate() {
        // FIXME(allancalix): Fails if the two bytes are not equal in length. It
        // probably makes more sense to count the difference in length towards the
        // distance.
        let y = second.get(i).unwrap();
        let mut diff = x ^ y;
        for _ in 0..8 {
            distance += diff & 1;
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
pub fn score(phrase: &str) -> f64 {
    let mut freq: HashMap<char, i32> = HashMap::new();

    for c in b'A'..=b'z' {
        freq.insert(char::from(c), 0);
    }

    for c in phrase.to_lowercase().chars() {
        let e = freq.entry(c).or_insert(0);
        *e += 1;
    }

    let mut score = f64::from(0);
    for (k, v) in freq.iter() {
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
            // Real messages have whitespace. This wasn't included in the character
            // frequency reference so I gave it a low-ish value.
            _ => continue,
        } / f64::from(100);
        let real_freq = f64::from(*v) / f64::from(phrase.len() as i32);
        let distance = (expected_freq - real_freq).abs();
        score += distance;
    }
    f64::from(1) - score
}

/// Returns a decrypted message from a string buffer.
pub fn decrypt(c: &Vec<u8>, key: u8) -> String {
    let mut decrypted: Vec<u8> = vec![];

    for v in c {
        decrypted.push(v ^ key);
    }
    // Some of the descrypted byte sequences don't produce valid utf-8 strings.
    // Since it's a message we're looking for, this is a useful gate.
    return String::from_utf8(decrypted).unwrap_or("".to_string());
}

#[derive(Debug, Clone)]
pub struct Guess {
    pub phrase: String,
    pub score: f64,
    pub key: char,
}

pub fn do_best_guess(phrase: Vec<u8>) -> Guess {
    // Not sure if the characters are limited to letters. Run through a wide range
    // of ascii characters.
    let mut scores = vec![];
    for c in b' '..=b'~' {
        let phrase = decrypt(&phrase, c);
        let s = score(phrase.as_str());
        scores.push(Guess {
            phrase,
            score: s,
            key: char::from(c),
        });
    }

    scores.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
    scores.last().unwrap().clone()
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
        let phrase = decrypt(&Vec::from(source), b'z');
        assert_eq!(decrypt(&Vec::from(phrase), b'z'), source);
    }

    #[test]
    fn test_scoring_can_guess_key() {
        let source = "hello i am a fake phrase";
        let phrase = decrypt(&Vec::from(source), b'z');
        // Can derive encryption key using scoring algorithm.
        assert_eq!(do_best_guess(Vec::from(phrase)).key, 'z');
    }

    #[test]
    fn test_scoring_can_guess_uppercase_key() {
        let source = "hello i am a FAKE phrase";
        let phrase = decrypt(&Vec::from(source), b'I');
        // Can derive encryption key using scoring algorithm.
        assert_eq!(do_best_guess(Vec::from(phrase)).key, 'I');
    }
}
