library;

use ::common::action::GuildAction;
use ::common::utils::{str_to_bytes, push_str, u64_to_ascii_bytes, unpad};

use std::assert::assert_eq;
use std::bytes::Bytes;
use std::string::String;

const Q: u8 = 34; // character \"
const LCB: u8 = 123; // character {
const RCB: u8 = 125; // character }
const LSB: u8 = 91; // character [
const RSB: u8 = 93; // character ]
const COLON: u8 = 58; // character :
const COMMA: u8 = 44; // character ,
pub struct PinData {
    pub holder: Address,
    pub action: GuildAction,
    pub user_id: u64,
    pub guild_id: u64,
    pub guild_name: str[64],
    pub created_at: u64,
    pub mint_date: u64,
    pub cid: str[64],
}

impl PinData {
    // NOTE I know this is horrible but do you have a better solution? I need to format this as
    // proper json string but there's no serde here lol. Hasher's inner bytes field is not
    // accessible anymore, because it's declared as private, so I cannot use Hasher's inner state
    // to append binary data and access it at the end.
    pub fn encode(self, pin_id: u64) -> String {
        let mut bytes = Bytes::new();
        bytes.push(LCB);
        bytes.push(Q);
        // name
        push_str("name", bytes);
        bytes.push(Q);
        bytes.push(COLON);
        bytes.push(Q);
        push_str(self.action.to_str(), bytes);
        bytes.push(Q);
        bytes.push(COMMA);
        // description
        bytes.push(Q);
        push_str("description", bytes);
        bytes.push(Q);
        bytes.push(COLON);
        bytes.push(Q);
        push_str("This is an onchain proof that you", bytes);
        push_str(self.action.to_description(), bytes);
        bytes.append(unpad(from_str_array(self.guild_name)));
        push_str(" on Guild.xyz", bytes);
        bytes.push(Q);
        bytes.push(COMMA);
        // image
        bytes.push(Q);
        push_str("image", bytes);
        bytes.push(Q);
        bytes.push(COLON);
        bytes.push(Q);
        push_str("ipfs://", bytes);
        bytes.append(unpad(from_str_array(self.cid)));
        bytes.push(Q);
        bytes.push(COMMA);
        // attributes
        bytes.push(Q);
        push_str("attributes", bytes);
        bytes.push(Q);
        bytes.push(COLON);
        bytes.push(LSB);
        // type
        bytes.push(LCB);
        type_json("type", bytes);
        value_json_str(self.action.to_str(), bytes);
        bytes.push(RCB);
        bytes.push(COMMA);
        // guild_id
        bytes.push(LCB);
        type_json("guildId", bytes);
        value_json_u64(self.guild_id, bytes);
        bytes.push(RCB);
        bytes.push(COMMA);
        // user_id
        bytes.push(LCB);
        type_json("userId", bytes);
        value_json_u64(self.user_id, bytes);
        bytes.push(RCB);
        bytes.push(COMMA);
        // rank
        bytes.push(LCB);
        type_json("rank", bytes);
        value_json_u64(pin_id, bytes);
        bytes.push(RCB);
        bytes.push(COMMA);
        // mint_date
        bytes.push(LCB);
        type_json("mintDate", bytes);
        value_json(u64_to_ascii_bytes(self.mint_date), false, bytes); //encode as string
        date_json(bytes);
        bytes.push(RCB);
        bytes.push(COMMA);
        // action_date
        bytes.push(LCB);
        type_json("actionDate", bytes);
        value_json(u64_to_ascii_bytes(self.created_at), false, bytes); // encode as string
        date_json(bytes);
        bytes.push(RCB);

        bytes.push(RSB);
        bytes.push(RCB);

        String::from(bytes)
    }
}


fn type_json(ty: str, ref mut bytes: Bytes) {
    bytes.push(Q);
    push_str("trait_type", bytes); // str cannot be const
    bytes.push(Q);
    bytes.push(COLON);
    bytes.push(Q);
    push_str(ty, bytes);
    bytes.push(Q);
    bytes.push(COMMA);
}

fn value_json_u64(value: u64, ref mut bytes: Bytes) {
    let input = u64_to_ascii_bytes(value);
    value_json(input, true, bytes);
}

fn value_json_str(value: str, ref mut bytes: Bytes) {
    let input = str_to_bytes(value);
    value_json(input, false, bytes);
}

fn value_json(value: Bytes, is_numeric: bool, ref mut bytes: Bytes) {
    bytes.push(Q);
    push_str("value", bytes);
    bytes.push(Q);
    bytes.push(COLON);
    if is_numeric {
        bytes.append(value);
    } else {
        bytes.push(Q);
        bytes.append(value);
        bytes.push(Q);
    }
}

fn date_json(ref mut bytes: Bytes) {
    bytes.push(COMMA);
    bytes.push(Q);
    push_str("display_type", bytes);
    bytes.push(Q);
    bytes.push(COLON);
    bytes.push(Q);
    push_str("date", bytes);
    bytes.push(Q);
}
