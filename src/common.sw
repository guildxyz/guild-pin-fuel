library;

use std::assert::assert;
use std::bytes::Bytes;
use std::bytes_conversions::u64::*;
use std::constants::ZERO_B256;
use std::hash::{Hash, Hasher};
use std::string::String;

pub type BalancesMap = StorageMap<Address, u64>;
pub type OwnersMap = StorageMap<u64, Option<Address>>;
pub type GuildIdActionTokenIdMap = StorageMap<u64, StorageMap<GuildAction, u64>>;
pub type TokenIdByAddressMap = StorageMap<Address, GuildIdActionTokenIdMap>;
pub type TokenIdByUserIdMap = StorageMap<u64, StorageKey<GuildIdActionTokenIdMap>>;
pub type TotalMintedPerGuildMap = StorageMap<u64, u64>;

const Q: u8 = 34; // character \"
const LCB: u8 = 123; // character {
const RCB: u8 = 125; // character }
const LSB: u8 = 91; // character [
const RSB: u8 = 93; // character ]
const X19: u8 = 25; // character \x19
const NEWLINE: u8 = 10; // character \n
const COLON: u8 = 58; // character :
const COMMA: u8 = 44; // character ,
pub enum GuildAction {
    Joined: (),
    Owner: (),
    Admin: (),
}

impl GuildAction {
    pub fn into_byte(self) -> u8 {
        match self {
            GuildAction::Joined => 0,
            GuildAction::Owner => 1,
            GuildAction::Admin => 2,
        }
    }

    pub fn to_str(self) -> str {
        match self {
            GuildAction::Joined => "Joined",
            GuildAction::Owner => "Owner of",
            GuildAction::Admin => "Admin of",
        }
    }

    pub fn to_description(self) -> str {
        match self {
            GuildAction::Joined => " joined ",
            GuildAction::Owner => "'re the owner of ",
            GuildAction::Admin => "'re the admin of ",
        }
    }
}

impl Hash for GuildAction {
    fn hash(self, ref mut state: Hasher) {
        let mut bytes = Bytes::with_capacity(1);
        bytes.push(self.into_byte());
        state.write(bytes);
    }
}

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
        from_str_array(self.guild_name).hash(hasher);
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
        from_str_array(self.cid).hash(hasher);
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
        value_json(u64_to_string(self.mint_date), true, hasher);
        RCB.hash(hasher);
        COMMA.hash(hasher);
        // action_date
        LCB.hash(hasher);
        type_json("actionDate", hasher);
        value_json(u64_to_string(self.created_at), true, hasher);
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

pub struct ClaimParameters {
    recipient: Address,
    action: GuildAction,
    user_id: u64,
    guild_id: u64,
    guild_name: str[64],
    created_at: u64,
    signed_at: u64,
    cid: str[64],
    admin_treasury: Identity,
    admin_fee: u64,
    contract_id: ContractId,
}

impl Hash for ClaimParameters {
    fn hash(self, ref mut state: Hasher) {
        self.recipient.hash(state);
        self.action.hash(state);
        self.user_id.hash(state);
        self.guild_id.hash(state);
        from_str_array(self.guild_name).hash(state);
        self.created_at.hash(state);
        self.signed_at.hash(state);
        from_str_array(self.cid).hash(state);
        self.admin_treasury.hash(state);
        self.admin_fee.hash(state);
        self.contract_id.hash(state);
    }
}

impl ClaimParameters {
    pub fn to_message(self) -> b256 {
        let mut hasher = Hasher::new();
        self.hash(hasher);
        let hashed_msg = hasher.keccak256();

        // hash again with ETH prefix
        let mut hasher = Hasher::new();
        // NOTE msg len will always be 32 bytes due to keccak256-hashing stuff first. Furthermore
        // sway compiler cant handle \x19 and \n so I need to append special characters manually
        X19.hash(hasher); // \x19
        "Ethereum Signed Message:".hash(hasher);
        NEWLINE.hash(hasher); // \n
        "32".hash(hasher); // length
        hashed_msg.hash(hasher);
        hasher.keccak256()
    }
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
