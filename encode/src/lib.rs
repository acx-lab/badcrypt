pub mod padding;

pub trait FromHex: Sized {
    type Error;

    fn from_hex<T: AsRef<[u8]>>(s: T) -> Result<Self, Self::Error>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FromHexError {
    InvalidHexCharacter { c: char, index: usize },
    InvalidHexLength,
}

impl FromHex for Vec<u8> {
    type Error = FromHexError;

    fn from_hex<T: AsRef<[u8]>>(s: T) -> Result<Self, Self::Error> {
        let bytes = s.as_ref();
        let mut b = Vec::with_capacity(bytes.len() / 2);
        let mut modulus = 0;
        let mut buf = 08;

        for (i, byte) in bytes.iter().enumerate() {
            buf <<= 4;

            match *byte {
                b'A'..=b'F' => buf |= byte - b'A' + 10,
                b'a'..=b'f' => buf |= byte - b'a' + 10,
                b'0'..=b'9' => buf |= byte - b'0',
                _ => {
                    return Err(FromHexError::InvalidHexCharacter {
                        c: bytes[i] as char,
                        index: i,
                    })
                }
            }

            modulus += 1;
            if modulus == 2 {
                modulus = 0;
                b.push(buf);
            }
        }

        match modulus {
            0 => Ok(b.into_iter().collect()),
            _ => Err(FromHexError::InvalidHexLength),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Base64(Vec<u8>);

impl Base64 {
    pub fn fixed_xor(&self, x: Base64) -> Base64 {
        let mut xor = vec![];
        for (i, v) in self.0.iter().enumerate() {
            // Panics if compared against buffer with a different length.
            let cmp = x.0.get(i).unwrap();
            xor.push(v ^ cmp);
        }
        assert!(xor.len() == self.0.len());
        Base64::from(xor)
    }
}

impl From<Vec<u8>> for Base64 {
    fn from(v: Vec<u8>) -> Self {
        Base64(v.clone())
    }
}

fn map_u8_to_base64(x: u8) -> char {
    match x {
        0..=25 => (x + 65) as char,
        26..=51 => ((x - 26) + 97) as char,
        52..=61 => ((x - 52) + 48) as char,
        62 => '+',
        63 => '/',
        _ => '=',
    }
}

impl ToString for Base64 {
    fn to_string(&self) -> String {
        let mut result = String::new();

        for (i, val) in self.0.iter().enumerate() {
            match i % 3 {
                0 => {
                    result.push(map_u8_to_base64(val >> 2));
                    if i == self.0.len() - 1 {
                        result.push(map_u8_to_base64(val << 6 >> 2));
                        result.push(map_u8_to_base64(255));
                        result.push(map_u8_to_base64(255));
                    }
                }
                1 => {
                    let head = self.0[i - 1] << 6;
                    let tail = val >> 2;
                    result.push(map_u8_to_base64((head | tail) >> 2));

                    if i == self.0.len() - 1 {
                        result.push(map_u8_to_base64(val << 4 >> 2));
                        result.push(map_u8_to_base64(255));
                    } else {
                        let bhead = val << 4;
                        let btail = self.0[i + 1] >> 4;
                        result.push(map_u8_to_base64((bhead | btail) >> 2));
                    }
                }
                2 => result.push(map_u8_to_base64(val << 2 >> 2)),
                _ => println!("oh no"),
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::{Base64, FromHex, FromHexError};

    #[test]
    fn test_decode_hex_to_bits() {
        assert_eq!(Vec::from_hex("68656C6C6F").unwrap(), b"hello");
    }

    #[test]
    fn test_odd_numbered_hex() {
        assert_eq!(
            Vec::from_hex("6656C6C6F").unwrap_err(),
            FromHexError::InvalidHexLength
        );
    }

    #[test]
    fn test_whitespace_in_hex() {
        assert_eq!(
            Vec::from_hex("6865 6C6C6F").unwrap_err(),
            FromHexError::InvalidHexCharacter { c: ' ', index: 4 },
        );
    }

    #[test]
    fn test_hex_buffer_to_base64() {
        let buf = Vec::from_hex("68656C6C6F6D").unwrap();
        let test = Base64::from(buf);
        assert_eq!(test.to_string(), "aGVsbG9t".to_string());
    }

    #[test]
    fn test_padding_once_hex_buffer_to_base64() {
        let buf = Vec::from_hex("68656C6C6F6DA3D2").unwrap();
        let test = Base64::from(buf);
        assert_eq!(test.to_string(), "aGVsbG9to9I=".to_string());
    }

    #[test]
    fn test_padded_twice_hex_buffer_to_base64() {
        let buf = Vec::from_hex("68656C6C6F6DA3").unwrap();
        let test = Base64::from(buf);
        assert_eq!(test.to_string(), "aGVsbG9tow==".to_string());
    }

    #[test]
    fn test_cryptopals() {
        let buf = Vec::from_hex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();
        let test = Base64::from(buf);
        assert_eq!(
            test.to_string(),
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string()
        );
    }

    #[test]
    fn test_cryptopals_fixed_xor() {
        let in_one = Base64::from(Vec::from_hex("1c0111001f010100061a024b53535009181c").unwrap());
        let in_two = Base64::from(Vec::from_hex("686974207468652062756c6c277320657965").unwrap());
        let xor = in_one.fixed_xor(in_two);

        assert_eq!(
            hex::encode(xor.0),
            "746865206b696420646f6e277420706c6179".to_string()
        )
    }
}
