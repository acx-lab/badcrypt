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

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_hamming_distance() {
    let dist = hamming_distance(vec!(0), vec!(1));
    assert_eq!(dist, 1);
  }

  #[test]
  fn test_crypto_example() {
    let s1 = Vec::from("this is a test");
    let s2 = Vec::from("wokka wokka!!!");
    assert_eq!(hamming_distance(s1, s2), 37);
  }
}