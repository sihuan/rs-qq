use std::num::ParseIntError;
use std::fmt::Write;

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

#[cfg(test)]
mod tests {
    use crate::hex::{decode_hex, encode_hex};

    #[test]
    fn test_decode() {
        let h = decode_hex("04EBCA94D733E399B2DB96EACDD3F69A8BB0F74224E2B44E3357812211D2E62EFBC91BB553098E25E33A799ADC7F76FEB208DA7C6522CDB0719A305180CC54A82E").unwrap();
        println!("{:?}", h)
    }

    #[test]
    fn test_encode() {
        let h = encode_hex(&vec![1, 2, 3]);
        println!("{}", h)
    }
}