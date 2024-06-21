library;

use std::assert::assert;
use std::bytes::Bytes;
use std::convert::TryFrom;
use std::primitive_conversions::u8::*;
use std::string::String;

// NOTE this is far from optimal, the only goal was for this
// to work. If time allows this could be optimized.
pub fn base64(input: String) -> Bytes {
    let input = input.as_bytes();
    let mut result = Bytes::new();
    let mut i = 0;
    while i < input.len() {
        let mut chunk = Bytes::new();
        let mut j = 0;
        while j < 3 && (i + j) < input.len() {
            chunk.push(input.get(i + j).unwrap());
            j += 1;
        }
        i += 3;

        let chunk = split(chunk);
        let encoded = encode_chunk(chunk);
        j = 0;
        while j < encoded.len() {
            result.push(encoded.get(j).unwrap());
            j += 1;
        }
    }
    result
}

// NOTE
// A-Z: 65-90 -> 26 letters
// a-z: 97-122 -> 26 letters
// 0-9: 48-57 -> 10 numbers
// '+': 43
// '/': 47
// '=': 61
fn get_char_for_index(index: u8) -> u8 {
    //let index = <u8 as TryFrom<u32>>::try_from(index).unwrap();
    if index <= 25 {
        index + 65u8
    } else if index >= 26 && index <= 51 {
        index - 26u8 + 97u8
    } else if index >= 52 && index <= 61 {
        index - 52u8 + 48u8
    } else if index == 62 {
        43
    } else if index == 63 { 47 } else { revert(0) }
}

fn split(input: Bytes) -> Bytes {
    let mut output = Bytes::new();
    match input.len() {
        0 => {},
        1 => {
            output.push(input.get(0).unwrap() >> 2);
            output.push((input.get(0).unwrap() & 3) << 4);
        },
        2 => {
            output.push(input.get(0).unwrap() >> 2);
            output.push((input.get(0).unwrap() & 3) << 4 | input.get(1).unwrap() >> 4);
            output.push((input.get(1).unwrap() & 15) << 2);
        },
        3 => {
            output.push(input.get(0).unwrap() >> 2);
            output.push((input.get(0).unwrap() & 3) << 4 | input.get(1).unwrap() >> 4);
            output.push((input.get(1).unwrap() & 15) << 2 | input.get(2).unwrap() >> 6);
            output.push(input.get(2).unwrap() & 63);
        },
        _ => revert(0),
    }
    output
}

fn encode_chunk(input: Bytes) -> Bytes {
    let mut output = Bytes::new();
    let mut i = 0;
    while i < input.len() {
        output.push(get_char_for_index(input.get(i).unwrap()));
        i += 1;
    }
    while output.len() < 4 {
        output.push(61);
    }
    output
}

#[test]
fn base64_encoding() {
    assert_eq(String::from(base64(String::from_ascii_str("a"))), String::from_ascii_str("YQ=="));
    assert_eq(String::from(base64(String::from_ascii_str("ab"))), String::from_ascii_str("YWI="));
    assert_eq(String::from(base64(String::from_ascii_str("abc"))), String::from_ascii_str("YWJj"));
    assert_eq(String::from(base64(String::from_ascii_str("hello"))), String::from_ascii_str("aGVsbG8="));
    assert_eq(
        String::from(base64(String::from_ascii_str("hello-world"))), String::from_ascii_str("aGVsbG8td29ybGQ="),
    );
}
