library;

use std::bytes::Bytes;
use std::hash::{Hash, Hasher};
use std::string::String;

pub type BalancesMap = StorageMap<Address, u64>;
pub type OwnersMap = StorageMap<u64, Option<Address>>;
pub type GuildIdActionTokenIdMap = StorageMap<u64, StorageMap<GuildAction, u64>>;
pub type TokenIdByAddressMap = StorageMap<Address, GuildIdActionTokenIdMap>;
pub type TokenIdByUserIdMap = StorageMap<u64, StorageKey<GuildIdActionTokenIdMap>>;
pub type TotalMintedPerGuildMap = StorageMap<u64, u64>;

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

pub struct PinDataParams {
    recipient: Address,
    action: GuildAction,
    user_id: u64,
    guild_id: u64,
    guild_name: str[64],
    created_at: u64,
    signed_at: u64,
    cid: str[64],
}

impl Hash for PinDataParams {
    fn hash(self, ref mut state: Hasher) {
        self.recipient.hash(state);
        self.action.hash(state);
        self.user_id.hash(state);
        self.guild_id.hash(state);
        from_str_array(self.guild_name).hash(state);
        self.created_at.hash(state);
        self.signed_at.hash(state);
        from_str_array(self.cid).hash(state);
    }
}

impl PinDataParams {
    pub fn to_message(
        self,
        admin_treasury: Identity,
        admin_fee: u64,
        contract_id: ContractId,
) -> b256 {
        let mut hasher = Hasher::new();
        self.hash(hasher);
        admin_treasury.hash(hasher);
        admin_fee.hash(hasher);
        contract_id.hash(hasher);
        let hashed_msg = hasher.keccak256();

        // hash again with ETH prefix
        let mut hasher = Hasher::new();
        // NOTE msg len will always be 32 bytes due to hashing stuff first
        "\x19Ethereum Signed Message:\n".hash(hasher);
        32u8.hash(hasher);
        hashed_msg.hash(hasher);
        hasher.keccak256()
    }
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