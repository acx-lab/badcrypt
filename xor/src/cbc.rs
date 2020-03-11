use encode::padding::pkcs7;
use openssl::symm::{Cipher, Crypter, Mode};
/// Implements CBC mode AES encryption.

const BLOCK_SIZE: usize = 16;

fn create_default_encrypter(key: &[u8], mode: Mode) -> Crypter {
    let mut c = Crypter::new(Cipher::aes_128_ecb(), mode, key, None).unwrap();
    // We're manually doing padding, no need for this.
    c.pad(false);

    c
}

/// 128-bit cbc encryption.
pub fn encrypt(input: &[u8], key: &[u8]) -> Vec<u8> {
    assert_eq!(key.len(), BLOCK_SIZE);
    assert_eq!(Cipher::aes_128_ecb().block_size(), BLOCK_SIZE);

    let mut encrypter = create_default_encrypter(key, Mode::Encrypt);
    let padded_input = pkcs7(input, BLOCK_SIZE);
    let mut cipher = Vec::new();
    let iv: &mut [u8] = &mut [b'\x00'; BLOCK_SIZE];
    for chunk in padded_input.chunks(BLOCK_SIZE) {
        let proccessed_chunk = xor(chunk, iv);
        let mut ciphertext = vec![0; chunk.len() + BLOCK_SIZE];

        let count = encrypter
            .update(proccessed_chunk.as_slice(), &mut ciphertext)
            .unwrap();
        assert_eq!(count, BLOCK_SIZE);

        iv.clone_from_slice(&ciphertext[..BLOCK_SIZE]);
        cipher.extend_from_slice(&ciphertext[..BLOCK_SIZE]);
    }

    cipher
}

pub fn decrypt(cipher: &[u8], key: &[u8]) -> Vec<u8> {
    let mut decrypter = create_default_encrypter(key, Mode::Decrypt);
    let mut plain = Vec::new();
    let iv: &mut [u8] = &mut [b'\x00'; BLOCK_SIZE];

    for chunk in cipher.chunks(BLOCK_SIZE) {
        let mut decrypted_chunk = vec![0; chunk.len() + BLOCK_SIZE];
        let count = decrypter.update(chunk, &mut decrypted_chunk).unwrap();
        assert_eq!(count, BLOCK_SIZE);

        plain.extend_from_slice(xor(&decrypted_chunk[..BLOCK_SIZE], iv).as_slice());
        iv.clone_from_slice(chunk);
    }

    plain.into_iter().filter(|b| *b != b'\x04').collect()
}

/// Returns a decrypted message from a string buffer.
pub fn xor(cipher: &[u8], key: &[u8]) -> Vec<u8> {
    let mut cv = key.iter().cycle();

    cipher
        .iter()
        .map(|c| {
            let kv = cv.next().unwrap();
            c ^ *kv as u8
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY: &[u8] = b"YELLOW SUBMARINE";

    #[test]
    fn cbc_encrypts_and_decrypts_single_block() {
        const SOURCE: &[u8] = b"YELLOW_SUBMARINE";
        let cipher = encrypt(SOURCE, KEY);
        assert_eq!(decrypt(cipher.as_slice(), KEY).len(), SOURCE.len());
        assert_eq!(decrypt(cipher.as_slice(), KEY), SOURCE);
    }

    #[test]
    fn encrypts_and_decrypts_single_nonconforming_block() {
        const SOURCE: &[u8] = b"YELLOW";
        let cipher = encrypt(SOURCE, KEY);
        assert_eq!(decrypt(cipher.as_slice(), KEY), SOURCE);
    }

    #[test]
    fn encrypts_and_decrypts_multiple_nonconforming_blocks() {
        const SOURCE: &[u8] = b"YELLOW SUBMARINE YELLOW SUBMARINE";
        let cipher = encrypt(SOURCE, KEY);
        assert_eq!(decrypt(cipher.as_slice(), KEY), SOURCE);
    }
}
