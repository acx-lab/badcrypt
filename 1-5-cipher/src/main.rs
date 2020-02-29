// Source to hash.
const SOURCE: &'static str = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
// Expected result provided by challenge.
const EXPECTED_KEY: &'static str = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

fn main() {
  let mut cv = Vec::from("ICE".to_string()).into_iter().cycle();
  let key: Vec<u8> = SOURCE
      .chars()
      .map(|c| {
        let kv = cv.next().unwrap();
        c as u8 ^ kv
      })
      .collect();
  assert_eq!(hex::encode(key), EXPECTED_KEY.to_string());
}