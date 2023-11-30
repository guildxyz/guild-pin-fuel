library;

use ::common::action::GuildAction;

use std::hash::{Hash, Hasher};

const X19: u8 = 25; // character \x19
const NEWLINE: u8 = 10; // character \n
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
