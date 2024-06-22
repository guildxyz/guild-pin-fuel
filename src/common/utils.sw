library;

use std::bytes::Bytes;
use std::bytes_conversions::u64::*;
use std::string::String;

pub fn str_to_bytes(s: str) -> Bytes {
    let str_size = s.len();
    let str_ptr = s.as_ptr();
    Bytes::from(raw_slice::from_parts::<u8>(str_ptr, str_size))
}

pub fn push_str(s: str, ref mut bytes: Bytes) {
    let str_bytes = str_to_bytes(s);
    bytes.append(str_bytes);
}

pub fn u64_to_ascii_bytes(num: u64) -> Bytes {
    let mut num = num;
    let mut bytes = Bytes::new();
    match num {
        0 => {
            bytes.push(48); // ascii for 0
            bytes
        }
        _ => {
            while num > 0 {
                let mut be_bytes = (num % 10).to_be_bytes();
                let digit = be_bytes.pop().unwrap() + 48;
                bytes.push(digit);
                num /= 10;
            }
            // there's no for loop???
            let len = bytes.len();
            let mut i = 0;
            while i < len / 2 {
                bytes.swap(i, len - i - 1);
                i += 1;
            }
            bytes
        }
    }
}

pub fn unpad(s: str) -> Bytes {
    let mut bytes = str_to_bytes(s);
    let mut len = bytes.len();
    // NOTE cannot do len > 0 && bytes.get(len - 1).unwrap() as the condition for while, because it
    // causes an internal compiler error (Block while has a misplaced terminator)
    while len > 0 {
        let last_byte = bytes.get(len - 1).unwrap();
        // space = 32 (padding character)
        if last_byte == 32 {
            let _ = bytes.pop();
            len = bytes.len();
        } else {
            break
        }
    }
    bytes
}

#[test]
fn convert_to_string() {
    assert_eq(
        String::from(u64_to_ascii_bytes(0)),
        String::from_ascii_str("0"),
    );
    assert_eq(
        String::from(u64_to_ascii_bytes(1)),
        String::from_ascii_str("1"),
    );
    assert_eq(
        String::from(u64_to_ascii_bytes(2)),
        String::from_ascii_str("2"),
    );
    assert_eq(
        String::from(u64_to_ascii_bytes(3)),
        String::from_ascii_str("3"),
    );
    assert_eq(
        String::from(u64_to_ascii_bytes(10)),
        String::from_ascii_str("10"),
    );
    assert_eq(
        String::from(u64_to_ascii_bytes(11)),
        String::from_ascii_str("11"),
    );
    assert_eq(
        String::from(u64_to_ascii_bytes(12)),
        String::from_ascii_str("12"),
    );
    assert_eq(
        String::from(u64_to_ascii_bytes(13)),
        String::from_ascii_str("13"),
    );
    assert_eq(
        String::from(u64_to_ascii_bytes(100)),
        String::from_ascii_str("100"),
    );
    assert_eq(
        String::from(u64_to_ascii_bytes(11111)),
        String::from_ascii_str("11111"),
    );
    assert_eq(
        String::from(u64_to_ascii_bytes(9876543210)),
        String::from_ascii_str("9876543210"),
    );
}

#[test]
fn unpad_string() {
    assert_eq(String::from(unpad("abc")), String::from_ascii_str("abc"));
    assert_eq(String::from(unpad(" ")), String::from_ascii_str(""));
    assert_eq(String::from(unpad("    ")), String::from_ascii_str(""));
    assert_eq(
        String::from(unpad("hello       ")),
        String::from_ascii_str("hello"),
    );
}
