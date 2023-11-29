library;

use ::common::PinData;
use ::interfaces::token::TokenError;

use src_7::Metadata;

use std::call_frames::contract_id;
use std::hash::Hash;
use std::string::String;

pub enum MetadataError {
    InvalidKeyLength: (),
    MIAFASZ: String,
}

abi PinMetadata {
    #[storage(read)]
    fn metadata(key: u64) -> String;
}

#[storage(read)]
pub fn _metadata(pin_id: u64, key: StorageKey<StorageMap<u64, PinData>>) -> String {
    if let Some(pin_data) = key.get(pin_id).try_read() {
        pin_data.encode()
    } else {
        require(false, TokenError::PinIdDoesNotExist);
        revert(0);
    }
}

fn parse(s: String) -> u64 {
    // copied from sway-lib-std/src/bytes_conversions/u64.sw, cuz I have ZERO idea why
    // the compiler cannot find "from_be_bytes" for "u64"
    let bytes = s.as_bytes();
    require(bytes.len() == 8, MetadataError::InvalidKeyLength);
    let ptr = bytes.buf.ptr();
    let h = ptr.read_byte();
    let g = (ptr.add_uint_offset(1)).read_byte();
    let f = (ptr.add_uint_offset(2)).read_byte();
    let e = (ptr.add_uint_offset(3)).read_byte();
    let d = (ptr.add_uint_offset(4)).read_byte();
    let c = (ptr.add_uint_offset(5)).read_byte();
    let b = (ptr.add_uint_offset(6)).read_byte();
    let a = (ptr.add_uint_offset(7)).read_byte();

    asm(a: a, b: b, c: c, d: d, e: e, f: f, g: g, h: h, i: 0x8, j: 0x10, k: 0x18, l: 0x20, m: 0x28, n: 0x30, o: 0x38, r1, r2, r3) {
        sll  r1 h o;
        sll  r2 g n;
        or   r3 r1 r2;
        sll  r1 f m;
        or   r2 r3 r1;
        sll  r3 e l;
        or   r1 r2 r3;
        sll  r2 d k;
        or   r3 r1 r2;
        sll  r1 c j;
        or   r2 r3 r1;
        sll  r3 b i;
        or   r1 r2 r3;
        or   r2 r1 a;

        r2: u64
    }
}
