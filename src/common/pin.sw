library;

use ::common::action::GuildAction;

use std::assert::assert;
use std::bytes::Bytes;
use std::bytes_conversions::u64::*;
use std::hash::{Hash, Hasher};
use std::string::String;

const Q: u8 = 34; // character \"
const LCB: u8 = 123; // character {
const RCB: u8 = 125; // character }
const LSB: u8 = 91; // character [
const RSB: u8 = 93; // character ]
const COLON: u8 = 58; // character :
const COMMA: u8 = 44; // character ,
pub struct PinData {
    holder: Address,
    action: GuildAction,
    user_id: u64,
    guild_id: u64,
    guild_name: str[64],
    created_at: u64,
    mint_date: u64,
    cid: str[64],
}

impl PinData {
    // I know this is horrible but do you have a better solution?
    // I need to format this as proper json string.
    pub fn encode(self, pin_id: u64) -> String {
        let mut hasher = Hasher::new();
        // name
        LCB.hash(hasher);
        Q.hash(hasher);
        "name".hash(hasher);
        Q.hash(hasher);
        COLON.hash(hasher);
        Q.hash(hasher);
        self.action.to_str().hash(hasher);
        Q.hash(hasher);
        COMMA.hash(hasher);
        // description
        Q.hash(hasher);
        "description".hash(hasher);
        Q.hash(hasher);
        COLON.hash(hasher);
        Q.hash(hasher);
        "This is an onchain proof that you".hash(hasher);
        self.action.to_description().hash(hasher);
        unpad(String::from_ascii_str(from_str_array(self.guild_name)))
            .hash(hasher);
        " on Guild.xyz".hash(hasher);
        Q.hash(hasher);
        COMMA.hash(hasher);
        // image
        Q.hash(hasher);
        "image".hash(hasher);
        Q.hash(hasher);
        COLON.hash(hasher);
        Q.hash(hasher);
        "ipfs://".hash(hasher);
        unpad(String::from_ascii_str(from_str_array(self.cid)))
            .hash(hasher);
        Q.hash(hasher);
        COMMA.hash(hasher);
        // attributes
        Q.hash(hasher);
        "attributes".hash(hasher);
        Q.hash(hasher);
        COLON.hash(hasher);
        LSB.hash(hasher);
        // type
        LCB.hash(hasher);
        type_json("type", hasher);
        value_json(self.action.to_str(), false, hasher);
        RCB.hash(hasher);
        COMMA.hash(hasher);
        // guild_id
        LCB.hash(hasher);
        type_json("guildId", hasher);
        value_json(u64_to_string(self.guild_id), true, hasher);
        RCB.hash(hasher);
        COMMA.hash(hasher);
        // user_id
        LCB.hash(hasher);
        type_json("userId", hasher);
        value_json(u64_to_string(self.user_id), true, hasher);
        RCB.hash(hasher);
        COMMA.hash(hasher);
        // rank
        LCB.hash(hasher);
        type_json("rank", hasher);
        value_json(u64_to_string(pin_id), true, hasher);
        RCB.hash(hasher);
        COMMA.hash(hasher);
        // mint_date
        LCB.hash(hasher);
        type_json("mintDate", hasher);
        value_json(u64_to_string(self.mint_date), false, hasher); //encode as string
        date_json(hasher);
        RCB.hash(hasher);
        COMMA.hash(hasher);
        // action_date
        LCB.hash(hasher);
        type_json("actionDate", hasher);
        value_json(u64_to_string(self.created_at), false, hasher); // encode as string
        date_json(hasher);
        RCB.hash(hasher);

        RSB.hash(hasher);
        RCB.hash(hasher);

        String::from(hasher.bytes)
    }
}

fn type_json(ty: str, ref mut hasher: Hasher) {
    Q.hash(hasher);
    "trait_type".hash(hasher); // str cannot be const
    Q.hash(hasher);
    COLON.hash(hasher);
    Q.hash(hasher);
    ty.hash(hasher);
    Q.hash(hasher);
    COMMA.hash(hasher);
}

fn value_json<T>(value: T, numeric: bool, ref mut hasher: Hasher)
where
    T: Hash,
{
    Q.hash(hasher);
    "value".hash(hasher);
    Q.hash(hasher);
    COLON.hash(hasher);
    if numeric {
        value.hash(hasher);
    } else {
        Q.hash(hasher);
        value.hash(hasher);
        Q.hash(hasher);
    }
}

fn date_json(ref mut hasher: Hasher) {
    COMMA.hash(hasher);
    Q.hash(hasher);
    "display_type".hash(hasher); // str cannot be const
    Q.hash(hasher);
    COLON.hash(hasher);
    Q.hash(hasher);
    "date".hash(hasher);
    Q.hash(hasher);
}

fn u64_to_string(num: u64) -> String {
    let mut num = num;
    let mut bytes = Bytes::new();
    match num {
        0 => String::from_ascii_str("0"),
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
            String::from(bytes)
        }
    }
}

fn unpad(s: String) -> String {
    let mut bytes = s.as_bytes();
    let mut len = bytes.len();
    // space = 32 (padding character)
    while len > 0 && bytes.get(len - 1).unwrap() == 32 {
        let _ = bytes.pop();
        len = bytes.len();
    }
    String::from(bytes)
}

#[test]
fn convert_to_string() {
    assert(u64_to_string(0) == String::from_ascii_str("0"));
    assert(u64_to_string(1) == String::from_ascii_str("1"));
    assert(u64_to_string(2) == String::from_ascii_str("2"));
    assert(u64_to_string(3) == String::from_ascii_str("3"));
    assert(u64_to_string(10) == String::from_ascii_str("10"));
    assert(u64_to_string(11) == String::from_ascii_str("11"));
    assert(u64_to_string(12) == String::from_ascii_str("12"));
    assert(u64_to_string(13) == String::from_ascii_str("13"));
    assert(u64_to_string(100) == String::from_ascii_str("100"));
    assert(u64_to_string(11111) == String::from_ascii_str("11111"));
    assert(
        u64_to_string(9876543210) == String::from_ascii_str("9876543210"),
    );
}

#[test]
fn unpad_string() {
    assert(
        unpad(String::from_ascii_str("abc")) == String::from_ascii_str("abc"),
    );
    assert(
        unpad(String::from_ascii_str(" ")) == String::from_ascii_str(""),
    );
    assert(
        unpad(String::from_ascii_str("    ")) == String::from_ascii_str(""),
    );
    assert(
        unpad(String::from_ascii_str("hello       ")) == String::from_ascii_str("hello"),
    );
}
