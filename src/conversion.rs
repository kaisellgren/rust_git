use std::str;
use std::num::FromStrRadix;
use std::num::ToStrRadix;

static HEX_ARRAY: &'static [u8] = b"0123456789abcdef";

pub fn bytes_to_hex_string(bytes: &[u8]) -> String {
    let mut v = Vec::with_capacity(bytes.len() * 2);

    for &byte in bytes.iter() {
        v.push(HEX_ARRAY[(byte >> 4) as uint]);
        v.push(HEX_ARRAY[(byte & 0xf) as uint]);
    }

    str::from_utf8(v.as_slice()).unwrap().into_string()
}

pub fn hex_string_to_bytes(hex: &str) -> Vec<u8> {
    let mut data = Vec::new();

    for index in range(0, hex.len() / 2).map(|x| x * 2) {
        let value: u8 = FromStrRadix::from_str_radix(hex.slice(index, index + 2), 16).unwrap();
        data.push(value);
    }

    data
}