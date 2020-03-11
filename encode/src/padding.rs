pub fn pkcs7(cipher: &[u8], block_size: u8) -> Vec<u8> {
    let distance = cipher.len() % block_size as usize;
    if distance == 0 {
        return Vec::from(cipher);
    }

    let padding = block_size as usize - distance;
    let mut padded_cipher = Vec::from(cipher);
    padded_cipher.extend_from_slice(vec![b'\x04'; padding].as_slice());

    padded_cipher
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pkcs7_pads_block_underflow() {
        // Len 16, padded by 4.
        let test = b"YELLOW SUBMARINE";
        let padded_block = pkcs7(test, 20);
        assert_eq!(padded_block.len(), 20);
    }

    #[test]
    fn test_pkcs7_pads_block_overflow() {
        // Len 32, padded by 8.
        let test = b"YELLOW SUBMARINE YELLOW SUBMARINE";
        let padded_block = pkcs7(test, 40);
        assert_eq!(padded_block.len(), 40);
    }

    #[test]
    fn test_pkcs7_nopad_matching_blocksize() {
        let test = b"YELLOW SUBMARINE YELLOW SUBMARINE YELLOW";
        let padded_block = pkcs7(test, 40);
        assert_eq!(padded_block.len(), 40);
        assert!(!padded_block.contains(&b'\x04'));
    }
}
