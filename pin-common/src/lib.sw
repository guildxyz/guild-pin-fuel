library;

use std::string::String;

pub enum GuildAction {
    Joined: (),
    Owner: (),
    Admin: (),
}

pub struct PinData {
    holder: Address,
    action: GuildAction,
    user_id: u64,
    guild_name: String,
    guild_or_role_id: u64,
    pin_number: u64,
    mint_date: u64,
    created_at: u64,
}

pub struct PinDataParams {
    receiver: Address,
    action: GuildAction,
    user_id: u64,
    gulid_name: String,
    guild_or_role_id: u64,
    created_at: u64,
    signed_at: u64,
    cid: String,
}

pub struct TokenUri {
    name: GuildAction,
    description: str[128],
    image: str[128],
    attributes: [Attribute; 4],
}

pub enum Attribute {
    UserId: u64,
    Rank: u64,
    ActionDate: u64,
    MintDate: u64,
}
