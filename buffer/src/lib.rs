pub fn chunk_by_size(buf: &Vec<u8>, size: usize) -> Vec<Vec<u8>> {
    let mut chunks = vec![];
    for i in 0..size {
        chunks.push(
            buf.clone()
                .into_iter()
                .skip(i)
                .step_by(size as usize)
                .collect(),
        );
    }
    chunks
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_chunk_buffer_symmetric() {
        let t = vec![b'A', b'B', b'C', b'A', b'B', b'C'];
        let chunks = chunk_by_size(&t, 3);
        assert_eq!(*chunks.get(0).unwrap(), vec!(b'A', b'A'));
        assert_eq!(*chunks.get(1).unwrap(), vec!(b'B', b'B'));
        assert_eq!(*chunks.get(2).unwrap(), vec!(b'C', b'C'));
    }

    #[test]
    fn test_chunk_buffer_asymmetric() {
        let t = vec![b'A', b'B', b'C', b'A'];
        let chunks = chunk_by_size(&t, 3);
        assert_eq!(*chunks.get(0).unwrap(), vec!(b'A', b'A'));
        assert_eq!(*chunks.get(1).unwrap(), vec!(b'B'));
        assert_eq!(*chunks.get(2).unwrap(), vec!(b'C'));
    }
}
